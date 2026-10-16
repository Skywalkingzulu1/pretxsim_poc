//! Modular per-hackathon adapter framework.
//!
//! Each target ecosystem gets one `Adapter` implementation. EVM-native
//! targets reuse the shared revm engine via `evm::EvmAdapter`; non-EVM
//! targets (Sui, Solana, Stellar, Aleo, MCP, agent firewall, ZK) implement
//! structural inspection against their native transaction shape without
//! touching the network.

use serde_json::Value;
use crate::config::ChainProfile;

pub mod evm;
pub mod sui;
pub mod solana;
pub mod stellar;
pub mod aleo;
pub mod mcp;
pub mod agent_firewall;
pub mod zk;
pub mod registry;

use std::sync::Arc;

/// Builds the default registry with every target adapter registered:
/// Sui, Solana, Stellar, Aleo, MCP, agent firewall, ZK, plus EVM-native
/// profiles (ethereum, monad, arc, base) through the shared revm engine.
pub fn default_registry() -> registry::AdapterRegistry {
    let mut reg = registry::AdapterRegistry::new();
    reg = reg.register(Arc::new(sui::SuiAdapter));
    reg = reg.register(Arc::new(solana::SolanaAdapter));
    reg = reg.register(Arc::new(stellar::StellarAdapter));
    reg = reg.register(Arc::new(aleo::AleoAdapter));
    reg = reg.register(Arc::new(mcp::McpAdapter));
    reg = reg.register(Arc::new(agent_firewall::AgentFirewallAdapter));
    reg = reg.register(Arc::new(zk::ZkAdapter));
    reg = reg.register(Arc::new(evm::EvmAdapter::new("ethereum", crate::config::ChainProfile::ethereum())));
    reg = reg.register(Arc::new(evm::EvmAdapter::new("monad", crate::config::ChainProfile::monad())));
    reg = reg.register(Arc::new(evm::EvmAdapter::new("arc", crate::config::ChainProfile::arc())));
    reg = reg.register(Arc::new(evm::EvmAdapter::new("base", crate::config::ChainProfile::base())));
    reg
}

/// Verdict returned by an adapter gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterVerdict {
    Allow,
    Block,
    Warn,
}

impl AdapterVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            AdapterVerdict::Allow => "ALLOW",
            AdapterVerdict::Block => "BLOCK",
            AdapterVerdict::Warn => "WARN",
        }
    }
}

/// Structured result of a single-tx inspection.
#[derive(Debug, Clone)]
pub struct AdapterResult {
    pub target: String,
    pub verdict: AdapterVerdict,
    pub reasons: Vec<String>,
    pub details: Value,
}

/// Every adapter exposes a name, an optional EVM chain profile, and a
/// pure structural inspection that never broadcasts.
pub trait Adapter: Send + Sync {
    fn name(&self) -> &'static str;
    fn chain_profile(&self) -> Option<ChainProfile>;
    fn inspect(&self, tx: &Value) -> AdapterResult;
}
