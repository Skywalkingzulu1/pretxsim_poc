//! Solana privacy RPC proxy (Solana Privacy Hack — Private Payments,
//! Privacy Tooling tracks).

use crate::adapters::{Adapter, AdapterResult, AdapterVerdict};
use serde_json::Value;

pub struct SolanaAdapter;

impl Adapter for SolanaAdapter {
    fn name(&self) -> &'static str {
        "solana"
    }

    fn chain_profile(&self) -> Option<crate::config::ChainProfile> {
        None
    }

    fn inspect(&self, tx: &Value) -> AdapterResult {
        let mut reasons: Vec<String> = Vec::new();
        let message = tx.get("message").and_then(|v| v.as_object()).cloned().unwrap_or_default();
        let instructions = message
            .get("instructions")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let details = serde_json::json!({
            "signatures": tx.get("signatures").cloned().unwrap_or_default(),
            "instructions": instructions,
            "recent_blockhash": message.get("recent_blockhash").cloned().unwrap_or_default(),
        });

        let program_ids: Vec<String> = message
            .get("account_keys")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|k| k.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let privacy_allowed = ["zoo", "night", "solana_memo", "spl_token_2022"];
        for pid in &program_ids {
            if !privacy_allowed.iter().any(|a| pid.contains(a)) && !pid.starts_with("1111") {
                reasons.push(format!("non-privacy program id in instruction set: {}", pid));
            }
        }

        let has_encrypted_memo = instructions.iter().any(|ix| {
            ix.get("program_id").and_then(|p| p.as_str()).unwrap_or("") == "spl_memo"
        });
        if !has_encrypted_memo {
            reasons.push("no encrypted memo hint — payment amount may be linkable".to_string());
        }

        let verdict = if reasons.is_empty() { AdapterVerdict::Allow } else { AdapterVerdict::Warn };

        AdapterResult { target: "solana".to_string(), verdict, reasons, details }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn private_payment_allowed() {
        let tx = json!({
            "signatures": ["sig"],
            "message": {
                "recent_blockhash": "bh",
                "account_keys": ["11111111111111111111111111111111", "zoo_program"],
                "instructions": [{"program_id": "spl_memo", "data": "enc"}]
            }
        });
        let r = SolanaAdapter.inspect(&tx);
        assert_eq!(r.verdict, AdapterVerdict::Allow);
    }

    #[test]
    fn no_memo_warns() {
        let tx = json!({
            "signatures": ["sig"],
            "message": {
                "recent_blockhash": "bh",
                "account_keys": ["11111111111111111111111111111111"],
                "instructions": []
            }
        });
        let r = SolanaAdapter.inspect(&tx);
        assert_eq!(r.verdict, AdapterVerdict::Warn);
    }
}
