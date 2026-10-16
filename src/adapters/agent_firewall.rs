//! Autonomous agent firewall (Open Agent Hackathon 2026).
//!
//! Wraps an autonomous agent's intended on-chain action and runs the
//! deterministic risk check plus a local-LLM advisory brief. The agent
//! consults the firewall before acting — keys and intents never leave
//! the machine.

use crate::adapters::{Adapter, AdapterResult, AdapterVerdict};
use serde_json::Value;

pub struct AgentFirewallAdapter;

impl Adapter for AgentFirewallAdapter {
    fn name(&self) -> &'static str {
        "agent_firewall"
    }

    fn chain_profile(&self) -> Option<crate::config::ChainProfile> {
        None
    }

    fn inspect(&self, tx: &Value) -> AdapterResult {
        let mut reasons: Vec<String> = Vec::new();
        let agent = tx.get("agent_id").and_then(|v| v.as_str()).unwrap_or("unknown");
        let action = tx.get("action").and_then(|v| v.as_str()).unwrap_or("unknown");
        let target = tx.get("target").and_then(|v| v.as_str()).unwrap_or("");
        let value = tx.get("value").and_then(|v| v.as_str()).unwrap_or("0");
        let details = serde_json::json!({
            "agent_id": agent,
            "action": action,
            "target": target,
            "value": value,
        });

        let autonomous = tx.get("autonomous").and_then(|v| v.as_bool()).unwrap_or(false);
        if autonomous && target.is_empty() {
            reasons.push("autonomous action with no target address — blind execution risk".to_string());
        }

        let drain_patterns = ["drain", "sweep", "transfer_all", "approve_max"];
        if drain_patterns.iter().any(|p| action.contains(p)) {
            reasons.push(format!("drain-pattern action '{}' refused pre-broadcast", action));
        }

        let requires_approval = tx.get("requires_approval").and_then(|v| v.as_bool()).unwrap_or(false);
        if requires_approval && !tx.get("user_consent").and_then(|v| v.as_bool()).unwrap_or(false) {
            reasons.push("user consent missing for approval-gated action".to_string());
        }

        let verdict = if reasons.is_empty() { AdapterVerdict::Allow } else { AdapterVerdict::Block };

        AdapterResult { target: "agent_firewall".to_string(), verdict, reasons, details }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn consented_action_allowed() {
        let tx = json!({
            "agent_id": "agent-1",
            "action": "swap",
            "target": "0xrouter",
            "value": "100",
            "autonomous": false,
            "requires_approval": true,
            "user_consent": true
        });
        let r = AgentFirewallAdapter.inspect(&tx);
        assert_eq!(r.verdict, AdapterVerdict::Allow);
    }

    #[test]
    fn drain_pattern_blocked() {
        let tx = json!({"agent_id": "agent-1", "action": "transfer_all", "target": "0xevil", "autonomous": true});
        let r = AgentFirewallAdapter.inspect(&tx);
        assert_eq!(r.verdict, AdapterVerdict::Block);
    }
}
