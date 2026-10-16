//! EVM adapter: wraps the existing revm engine (`run_single_tx`) so any
//! registered `ChainProfile` target becomes a hackathon adapter.

use crate::adapters::{Adapter, AdapterResult, AdapterVerdict};
use crate::config::ChainProfile;
use serde_json::Value;

pub struct EvmAdapter {
    pub name: &'static str,
    pub profile: ChainProfile,
}

impl EvmAdapter {
    pub fn new(name: &'static str, profile: ChainProfile) -> Self {
        Self { name, profile }
    }
}

impl Adapter for EvmAdapter {
    fn name(&self) -> &'static str {
        self.name
    }

    fn chain_profile(&self) -> Option<ChainProfile> {
        Some(self.profile.clone())
    }

    fn inspect(&self, tx: &Value) -> AdapterResult {
        let mut reasons: Vec<String> = Vec::new();
        let mut details = serde_json::json!({});

        let out = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(crate::run_single_tx(
                self.profile.clone(),
                tx.clone(),
                false,
            ))
        });

        let blocked = out.get("blocked").and_then(|v| v.as_bool()).unwrap_or(false);
        let analysis = out.get("analysis").cloned().unwrap_or(Value::Null);
        let exec_success = out.get("execution_success").and_then(|v: &Value| v.as_bool()).unwrap_or(false);
        let gas_used = out.get("gas_used").and_then(|v: &Value| v.as_u64()).unwrap_or(0);
        let sim_micros = out.get("sim_micros").and_then(|v: &Value| v.as_u64()).unwrap_or(0);

        details = serde_json::json!({
            "chain": self.profile.key,
            "chain_id": self.profile.chain_id,
            "execution_success": exec_success,
            "gas_used": gas_used,
            "sim_micros": sim_micros,
            "analysis": analysis,
        });

        if let Some(err) = out.get("error") {
            reasons.push(format!("simulation error: {}", err));
        }

        let verdict = if blocked {
            AdapterVerdict::Block
        } else if !exec_success {
            AdapterVerdict::Warn
        } else {
            AdapterVerdict::Allow
        };

        if blocked {
            reasons.push("structural BLOCK heuristic triggered".to_string());
        }

        AdapterResult {
            target: self.name.to_string(),
            verdict,
            reasons,
            details,
        }
    }
}
