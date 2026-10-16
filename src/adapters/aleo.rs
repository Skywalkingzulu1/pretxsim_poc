//! Aleo private app simulation (Aleo Privacy Virtual Hackathon).

use crate::adapters::{Adapter, AdapterResult, AdapterVerdict};
use serde_json::Value;

pub struct AleoAdapter;

impl Adapter for AleoAdapter {
    fn name(&self) -> &'static str {
        "aleo"
    }

    fn chain_profile(&self) -> Option<crate::config::ChainProfile> {
        None
    }

    fn inspect(&self, tx: &Value) -> AdapterResult {
        let mut reasons: Vec<String> = Vec::new();
        let program = tx.get("program_id").and_then(|v| v.as_str()).unwrap_or("unknown");
        let caller = tx.get("caller").and_then(|v| v.as_str()).unwrap_or("unknown");
        let records = tx
            .get("records")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let inputs = tx
            .get("inputs")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let details = serde_json::json!({
            "program_id": program,
            "caller": caller,
            "records": records,
            "inputs": inputs,
            "fee": tx.get("fee").cloned().unwrap_or_default(),
        });

        let encrypted = tx.get("encrypted").and_then(|v| v.as_bool()).unwrap_or(false);
        if !encrypted {
            reasons.push("records not encrypted — private data exposed".to_string());
        }

        let view_key = tx.get("viewing_key").and_then(|v| v.as_str()).unwrap_or("");
        if view_key.is_empty() {
            reasons.push("no viewing key bound — caller cannot audit own records".to_string());
        }

        let plain_amount = tx.get("amount").and_then(|v| v.as_u64()).unwrap_or(0);
        if plain_amount > 0 {
            reasons.push("plaintext credit amount present — use record instead".to_string());
        }

        let verdict = if reasons.is_empty() { AdapterVerdict::Allow } else { AdapterVerdict::Warn };

        AdapterResult { target: "aleo".to_string(), verdict, reasons, details }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn private_call_allowed() {
        let tx = json!({
            "program_id": "credit.aleo",
            "caller": "aleo1...",
            "encrypted": true,
            "viewing_key": "avk1...",
            "records": [{"owner": "aleo1..."}],
            "inputs": []
        });
        let r = AleoAdapter.inspect(&tx);
        assert_eq!(r.verdict, AdapterVerdict::Allow);
    }

    #[test]
    fn unencrypted_warns() {
        let tx = json!({"program_id": "credit.aleo", "caller": "aleo1...", "encrypted": false});
        let r = AleoAdapter.inspect(&tx);
        assert_eq!(r.verdict, AdapterVerdict::Warn);
    }
}
