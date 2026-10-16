//! Stellar / Poseidon hash adapter (HackMeridian, Stellar Real-World ZK).

use crate::adapters::{Adapter, AdapterResult, AdapterVerdict};
use serde_json::Value;

pub struct StellarAdapter;

impl Adapter for StellarAdapter {
    fn name(&self) -> &'static str {
        "stellar"
    }

    fn chain_profile(&self) -> Option<crate::config::ChainProfile> {
        None
    }

    fn inspect(&self, tx: &Value) -> AdapterResult {
        let mut reasons: Vec<String> = Vec::new();
        let source = tx.get("source_account").and_then(|v| v.as_str()).unwrap_or("unknown");
        let fee = tx.get("fee").and_then(|v| v.as_u64()).unwrap_or(0);
        let operations = tx
            .get("operations")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let memo = tx.get("memo").and_then(|v| v.as_str()).unwrap_or("");
        let details = serde_json::json!({
            "source_account": source,
            "fee": fee,
            "memo": memo,
            "operations": operations,
            "time_bounds": tx.get("time_bounds").cloned().unwrap_or_default(),
        });

        let has_memo_hash = !memo.is_empty() && memo.len() >= 64;
        if !has_memo_hash {
            reasons.push("no hash memo commitment — ZK proof target not bound".to_string());
        }

        let sponsor = tx.get("sponsor").and_then(|v| v.as_str()).unwrap_or("");
        if sponsor.is_empty() {
            reasons.push("no sponsor set — threshold policy not anchored".to_string());
        }

        let op_types: Vec<String> = operations
            .iter()
            .filter_map(|op| op.get("type").and_then(|t| t.as_str()).map(|s| s.to_string()))
            .collect();
        let has_payment = op_types
            .iter()
            .any(|t| t == "payment" || t == "create_claimable_balance");
        if !has_payment {
            reasons.push("no payment/claimable-balance operation detected".to_string());
        }

        let verdict = if reasons.is_empty() { AdapterVerdict::Allow } else { AdapterVerdict::Warn };

        AdapterResult { target: "stellar".to_string(), verdict, reasons, details }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn zk_bound_tx_allowed() {
        let tx = json!({
            "source_account": "GAAAAA...",
            "fee": 100,
            "memo": "0000000000000000000000000000000000000000000000000000000000000000",
            "sponsor": "GSPONSOR...",
            "operations": [{"type": "payment"}]
        });
        let r = StellarAdapter.inspect(&tx);
        assert_eq!(r.verdict, AdapterVerdict::Allow);
    }

    #[test]
    fn missing_memo_hash_warns() {
        let tx = json!({"source_account": "G", "fee": 100, "operations": [{"type": "payment"}]});
        let r = StellarAdapter.inspect(&tx);
        assert_eq!(r.verdict, AdapterVerdict::Warn);
    }
}
