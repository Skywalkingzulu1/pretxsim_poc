//! Arc x402 gate: validate machine-to-machine payment payloads before
//! execution on Arc (USDC-native gas).
//!
//! Inspects an x402-style payment object attached to a transaction request
//! (as the `"x402"` field of the tx JSON) and enforces agent spending
//! limits: per-payment caps, asset allowlist, expiry, and replay nonces.
//! Pure logic — no network — so it is unit-testable and runs air-gapped.

use serde::{Deserialize, Serialize};

/// Amounts are integer micro-USDC (6 decimals): 1 USDC = 1_000_000.
pub const MICROS_PER_USDC: u64 = 1_000_000;

/// Default per-payment cap: 10,000 USDC.
pub const DEFAULT_MAX_SINGLE_MICROS: u64 = 10_000 * MICROS_PER_USDC;

/// x402 payment payload attached to a tx request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct X402Payment {
    /// Integer micro-USDC as a decimal string (e.g. "2500000" = 2.5 USDC).
    #[serde(default)]
    pub amount: String,
    /// Asset identifier; must be a USDC variant on Arc.
    #[serde(default)]
    pub asset: String,
    /// Recipient address (0x hex).
    #[serde(default)]
    pub recipient: String,
    /// Replay-protection nonce chosen by the agent.
    #[serde(default)]
    pub nonce: String,
    /// Unix timestamp after which the payment is invalid (0 = no expiry).
    #[serde(default)]
    pub expires_at: u64,
}

/// Agent spending policy enforced by the gate.
#[derive(Debug, Clone)]
pub struct GateLimits {
    pub max_single_micros: u64,
    pub require_expiry: bool,
    /// Lowercase asset allowlist, e.g. ["usdc", "0xa0b8..."].
    pub allowed_assets: Vec<String>,
}

impl Default for GateLimits {
    fn default() -> Self {
        Self {
            max_single_micros: DEFAULT_MAX_SINGLE_MICROS,
            require_expiry: true,
            allowed_assets: vec!["usdc".to_string()],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum X402Verdict {
    Allow,
    Block,
}

/// Structured inspection result (serializable for --ci/--sim-file output).
#[derive(Debug, Clone, Serialize)]
pub struct X402Inspection {
    pub present: bool,
    pub verdict: X402Verdict,
    pub amount_micros: Option<u64>,
    pub reasons: Vec<String>,
}

fn is_hex_address(s: &str) -> bool {
    s.len() == 42
        && s.starts_with("0x")
        && s[2..].chars().all(|c| c.is_ascii_hexdigit())
}

/// Current unix time (seconds). std-only to avoid new dependencies.
fn now_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn inspect(payment: &X402Payment, limits: &GateLimits) -> X402Inspection {
    let mut reasons: Vec<String> = Vec::new();

    let amount_micros = match payment.amount.trim().parse::<u64>() {
        Ok(v) => Some(v),
        Err(_) => {
            reasons.push("amount is not a valid integer micro-USDC string".to_string());
            None
        }
    };

    if let Some(v) = amount_micros {
        if v == 0 {
            reasons.push("amount is zero".to_string());
        }
        if v > limits.max_single_micros {
            reasons.push(format!(
                "amount {} micros exceeds agent single-payment cap {} micros",
                v, limits.max_single_micros
            ));
        }
    }

    let asset_norm = payment.asset.trim().to_ascii_lowercase();
    if asset_norm.is_empty() {
        reasons.push("asset is missing".to_string());
    } else if !limits
        .allowed_assets
        .iter()
        .any(|a| asset_norm.contains(&a.to_ascii_lowercase()))
    {
        reasons.push(format!("asset '{}' is not allowlisted", payment.asset));
    }

    if !is_hex_address(payment.recipient.trim()) {
        reasons.push("recipient is not a valid 0x address".to_string());
    }

    if payment.nonce.trim().is_empty() {
        reasons.push("nonce is missing (replay risk)".to_string());
    }

    if payment.expires_at == 0 {
        if limits.require_expiry {
            reasons.push("expires_at is missing but expiry is required".to_string());
        }
    } else if payment.expires_at <= now_unix() {
        reasons.push("payment is expired".to_string());
    }

    let verdict = if reasons.is_empty() {
        X402Verdict::Allow
    } else {
        X402Verdict::Block
    };

    X402Inspection {
        present: true,
        verdict,
        amount_micros,
        reasons,
    }
}

/// Extract an x402 payload from a tx JSON value, if present.
/// Accepts either `{"x402": {...}}` or a raw payload object.
pub fn extract(tx: &serde_json::Value) -> Option<X402Payment> {
    let node = tx.get("x402").unwrap_or(tx);
    // Only treat as x402 when it carries at least an amount + recipient.
    if node.get("amount").is_some() && node.get("recipient").is_some() {
        serde_json::from_value(node.clone()).ok()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn valid_payment() -> X402Payment {
        X402Payment {
            amount: "2500000".to_string(),
            asset: "USDC".to_string(),
            recipient: "0x1111111111111111111111111111111111111111".to_string(),
            nonce: "abc123".to_string(),
            expires_at: now_unix() + 3600,
        }
    }

    #[test]
    fn valid_payload_allowed() {
        let r = inspect(&valid_payment(), &GateLimits::default());
        assert!(r.present);
        assert_eq!(r.verdict, X402Verdict::Allow);
        assert_eq!(r.amount_micros, Some(2_500_000));
        assert!(r.reasons.is_empty());
    }

    #[test]
    fn over_limit_blocked() {
        let mut p = valid_payment();
        p.amount = (DEFAULT_MAX_SINGLE_MICROS + 1).to_string();
        let r = inspect(&p, &GateLimits::default());
        assert_eq!(r.verdict, X402Verdict::Block);
        assert!(r.reasons.iter().any(|x| x.contains("exceeds")));
    }

    #[test]
    fn expired_blocked() {
        let mut p = valid_payment();
        p.expires_at = now_unix().saturating_sub(10);
        let r = inspect(&p, &GateLimits::default());
        assert_eq!(r.verdict, X402Verdict::Block);
    }

    #[test]
    fn bad_recipient_and_missing_nonce_blocked() {
        let mut p = valid_payment();
        p.recipient = "not-an-address".to_string();
        p.nonce = "".to_string();
        let r = inspect(&p, &GateLimits::default());
        assert_eq!(r.verdict, X402Verdict::Block);
        assert_eq!(r.reasons.len(), 2);
    }

    #[test]
    fn non_usdc_asset_blocked() {
        let mut p = valid_payment();
        p.asset = "DOGE".to_string();
        let r = inspect(&p, &GateLimits::default());
        assert_eq!(r.verdict, X402Verdict::Block);
    }

    #[test]
    fn extract_from_tx_wrapper() {
        let tx = json!({
            "from": "0x0000000000000000000000000000000000000001",
            "x402": {
                "amount": "1000000",
                "asset": "USDC",
                "recipient": "0x1111111111111111111111111111111111111111",
                "nonce": "n1",
                "expires_at": 9999999999u64
            }
        });
        let p = extract(&tx).expect("should extract");
        assert_eq!(p.amount, "1000000");
    }

    #[test]
    fn extract_absent_when_no_payment_fields() {
        let tx = json!({"from": "0xabc", "value": "0x1"});
        assert!(extract(&tx).is_none());
    }
}
