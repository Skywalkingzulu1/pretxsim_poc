//! Sui agent wrapper (Sui Overflow 2026 — Autonomous AI agents track).
//!
//! Inspects a Sui transaction bytes/shape for autonomous-agent hazards:
//! object transfers to unknown recipients, publish upgrades, and call
//! entries touching sensitive modules. Pure structural read — no network.

use crate::adapters::{Adapter, AdapterResult, AdapterVerdict};
use serde_json::Value;

pub struct SuiAdapter;

impl Adapter for SuiAdapter {
    fn name(&self) -> &'static str {
        "sui"
    }

    fn chain_profile(&self) -> Option<crate::config::ChainProfile> {
        None
    }

    fn inspect(&self, tx: &Value) -> AdapterResult {
        let mut reasons: Vec<String> = Vec::new();
        let kind = tx.get("kind").and_then(|v| v.as_str()).unwrap_or("");
        let sender = tx.get("sender").and_then(|v| v.as_str()).unwrap_or("unknown");
        let gas = tx.get("gas").and_then(|v| v.as_str()).unwrap_or("0");
        let details = serde_json::json!({
            "kind": kind,
            "sender": sender,
            "gas_budget": gas,
            "transactions": tx.get("transactions").cloned().unwrap_or(Value::Null),
        });

        let klower = kind.to_ascii_lowercase();
        if klower.contains("transferobject") || klower.contains("transfer_sui") {
            let recipient = tx.get("recipient").or_else(|| tx.get("address")).and_then(|v| v.as_str()).unwrap_or("");
            if recipient.is_empty() || recipient == "0x0" || recipient == "0" {
                reasons.push("object transfer with missing or zero recipient".to_string());
            }
        }
        if klower.contains("publish") || klower.contains("upgrade") {
            reasons.push("module publish/upgrade detected — agent autonomy risk".to_string());
        }
        if klower.contains("call") {
            let target = tx.get("target").or_else(|| tx.get("package")).and_then(|v| v.as_str()).unwrap_or("");
            if target.is_empty() {
                reasons.push("call entry missing target package".to_string());
            }
        }

        let verdict = if reasons.is_empty() { AdapterVerdict::Allow } else { AdapterVerdict::Block };

        AdapterResult { target: "sui".to_string(), verdict, reasons, details }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn plain_call_allowed() {
        let tx = json!({"kind": "Call", "sender": "0xabc", "gas": "1000", "target": "0xpkg", "arguments": []});
        let r = SuiAdapter.inspect(&tx);
        assert_eq!(r.verdict, AdapterVerdict::Allow);
    }

    #[test]
    fn zero_recipient_blocked() {
        let tx = json!({"kind": "TransferObject", "sender": "0xabc", "gas": "1000", "recipient": "0x0"});
        let r = SuiAdapter.inspect(&tx);
        assert_eq!(r.verdict, AdapterVerdict::Block);
        assert!(r.reasons.iter().any(|x| x.contains("zero recipient")));
    }

    #[test]
    fn publish_blocked() {
        let tx = json!({"kind": "Publish", "sender": "0xabc", "gas": "1000"});
        let r = SuiAdapter.inspect(&tx);
        assert_eq!(r.verdict, AdapterVerdict::Block);
    }
}
