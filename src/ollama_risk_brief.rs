//! Ollama risk briefs: feed raw revm trace output into a local LLM and
//! get back a structured, human-readable verdict before signing.
//!
//! The deterministic RiskInspector always decides BLOCK/Allow mechanics;
//! this module only *explains*. Model + endpoint are configurable; the
//! default matches docker-compose (Ollama + smollm2).

use serde::{Deserialize, Serialize};

pub const DEFAULT_OLLAMA_URL: &str = "http://localhost:11434/api/generate";
pub const DEFAULT_MODEL: &str = "smollm2:135m";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BriefVerdict {
    Pass,
    Warn,
    Block,
    Unknown,
}

impl BriefVerdict {
    pub fn as_str(&self) -> &'static str {
        match self {
            BriefVerdict::Pass => "PASS",
            BriefVerdict::Warn => "WARN",
            BriefVerdict::Block => "BLOCK",
            BriefVerdict::Unknown => "UNKNOWN",
        }
    }
}

/// Structured brief returned to callers (serializable for JSON output).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskBrief {
    pub verdict: BriefVerdict,
    pub summary: String,
    pub model: String,
}

/// Parse a free-text LLM reply into a verdict. BLOCK wins over WARN wins
/// over PASS; anything else is Unknown. Case-insensitive.
pub fn parse_verdict(text: &str) -> BriefVerdict {
    let upper = text.to_ascii_uppercase();
    // Heuristic order matters: explicit BLOCK dominates.
    if upper.contains("BLOCK") || upper.contains("DO NOT") || upper.contains("REFUSE") {
        BriefVerdict::Block
    } else if upper.contains("WARN") || upper.contains("CAUTION") || upper.contains("RISKY") {
        BriefVerdict::Warn
    } else if upper.contains("PASS") || upper.contains("SAFE") || upper.contains("APPROVE") {
        BriefVerdict::Pass
    } else {
        BriefVerdict::Unknown
    }
}

pub fn build_prompt(analysis_json: &str) -> String {
    format!(
        "You are an EVM security agent for a self-custodial AI agent wallet. \
Analyze this pre-execution structural analysis of a pending transaction:\n{}\n\
Reply with exactly one verdict word on the first line (PASS, WARN, or BLOCK), \
then 2-4 short sentences a non-expert can understand: what the transaction does, \
the single biggest risk (if any), and what to do next.",
        analysis_json
    )
}

/// Query Ollama and wrap the reply. Never errors on model absence —
/// returns Unknown with the reason so pipelines degrade gracefully.
pub async fn brief_from_analysis_json(
    analysis_json: &str,
    ollama_url: &str,
    model: &str,
) -> RiskBrief {
    let client = crate::config::http_client();
    let prompt = build_prompt(analysis_json);
    let res = client
        .post(ollama_url)
        .json(&serde_json::json!({
            "model": model,
            "prompt": prompt,
            "stream": false,
        }))
        .send()
        .await;

    match res {
        Ok(r) if r.status().is_success() => match r.json::<serde_json::Value>().await {
            Ok(body) => {
                let text = body["response"].as_str().unwrap_or("").to_string();
                if text.trim().is_empty() {
                    RiskBrief {
                        verdict: BriefVerdict::Unknown,
                        summary: "Ollama returned an empty response.".to_string(),
                        model: model.to_string(),
                    }
                } else {
                    RiskBrief {
                        verdict: parse_verdict(&text),
                        summary: text,
                        model: model.to_string(),
                    }
                }
            }
            Err(e) => RiskBrief {
                verdict: BriefVerdict::Unknown,
                summary: format!("Ollama reply was not JSON: {}", e),
                model: model.to_string(),
            },
        },
        Ok(r) => RiskBrief {
            verdict: BriefVerdict::Unknown,
            summary: format!("Ollama HTTP {} (model '{}' may be missing)", r.status(), model),
            model: model.to_string(),
        },
        Err(e) => RiskBrief {
            verdict: BriefVerdict::Unknown,
            summary: format!("Ollama unreachable at {}: {}", ollama_url, e),
            model: model.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_wins_over_pass() {
        assert_eq!(parse_verdict("PASS overall but BLOCK this drain"), BriefVerdict::Block);
    }

    #[test]
    fn warn_detected() {
        assert_eq!(
            parse_verdict("WARN: unlimited allowance granted"),
            BriefVerdict::Warn
        );
        assert_eq!(
            parse_verdict("This looks risky, proceed with caution"),
            BriefVerdict::Warn
        );
    }

    #[test]
    fn pass_detected() {
        assert_eq!(parse_verdict("PASS: plain transfer"), BriefVerdict::Pass);
        assert_eq!(parse_verdict("looks safe to me"), BriefVerdict::Pass);
    }

    #[test]
    fn unknown_on_empty_or_gibberish() {
        assert_eq!(parse_verdict(""), BriefVerdict::Unknown);
        assert_eq!(parse_verdict("lorem ipsum dolor"), BriefVerdict::Unknown);
    }

    #[test]
    fn prompt_contains_analysis_and_format() {
        let p = build_prompt("{...}");
        assert!(p.contains("{...}"));
        assert!(p.contains("PASS"));
    }

    #[test]
    fn verdict_serializes() {
        let b = RiskBrief {
            verdict: BriefVerdict::Warn,
            summary: "s".to_string(),
            model: "m".to_string(),
        };
        let v = serde_json::to_value(&b).unwrap();
        assert_eq!(v["verdict"], "Warn");
    }
}
