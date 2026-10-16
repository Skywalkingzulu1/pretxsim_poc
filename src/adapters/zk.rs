//! ZK simulation wrapper (Privacy Virtual Hackathon, zkVerify Foundation).

use crate::adapters::{Adapter, AdapterResult, AdapterVerdict};
use serde_json::Value;

pub struct ZkAdapter;

impl Adapter for ZkAdapter {
    fn name(&self) -> &'static str {
        "zk"
    }

    fn chain_profile(&self) -> Option<crate::config::ChainProfile> {
        None
    }

    fn inspect(&self, tx: &Value) -> AdapterResult {
        let mut reasons: Vec<String> = Vec::new();
        let curve = tx.get("curve").and_then(|v| v.as_str()).unwrap_or("unknown");
        let proof = tx.get("proof").and_then(|v| v.as_str()).unwrap_or("");
        let pub_inputs = tx
            .get("public_inputs")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let vk = tx.get("verification_key").and_then(|v| v.as_str()).unwrap_or("");
        let details = serde_json::json!({
            "curve": curve,
            "proof_present": !proof.is_empty(),
            "public_inputs": pub_inputs,
            "verification_key_present": !vk.is_empty(),
        });

        let allowed_curves = ["bn254", "bls12_381", "p256", "secp256k1"];
        if !allowed_curves.iter().any(|c| curve.contains(c)) {
            reasons.push(format!("unsupported ZK curve: {}", curve));
        }

        if proof.is_empty() {
            reasons.push("proof missing — cannot verify".to_string());
        }

        if vk.is_empty() {
            reasons.push("verification key missing — unverifiable proof".to_string());
        }

        let inputs_len = pub_inputs.len();
        if inputs_len == 0 {
            reasons.push("no public inputs bound — proof not tied to statement".to_string());
        }

        let verdict = if reasons.is_empty() { AdapterVerdict::Allow } else { AdapterVerdict::Block };

        AdapterResult { target: "zk".to_string(), verdict, reasons, details }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn valid_proof_allowed() {
        let tx = json!({
            "curve": "bn254",
            "proof": "0xproof",
            "public_inputs": ["0x1"],
            "verification_key": "0xvk"
        });
        let r = ZkAdapter.inspect(&tx);
        assert_eq!(r.verdict, AdapterVerdict::Allow);
    }

    #[test]
    fn missing_proof_blocked() {
        let tx = json!({"curve": "bn254", "proof": "", "public_inputs": [], "verification_key": ""});
        let r = ZkAdapter.inspect(&tx);
        assert_eq!(r.verdict, AdapterVerdict::Block);
    }
}
