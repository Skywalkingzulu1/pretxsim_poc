//! Chain profiles: one core engine, many networks.
//!
//! Each hackathon target gets a named profile selecting RPC defaults,
//! chain id, gas bounds, and feature flags (parallel sim, x402 gate,
//! USDC-native gas accounting). Profiles are data — the revm engine,
//! RiskInspector, and gate logic stay shared.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainKind {
    Ethereum,
    Monad,
    Arc,
    Base,
}

#[derive(Debug, Clone)]
pub struct ChainProfile {
    pub kind: ChainKind,
    /// Short profile key used with `--chain`.
    pub key: &'static str,
    /// Upstream RPC endpoint (overridable via RPC_URL env / --rpc-url).
    pub rpc_url: String,
    /// EVM chain id used for CfgEnv + TxEnv.
    pub chain_id: u64,
    /// Block gas limit used for the simulation BlockEnv.
    pub block_gas_limit: u64,
    /// Fallback gas price (wei) when the tx omits one.
    pub default_gas_price_wei: u64,
    /// High-throughput concurrent simulation (Monad target).
    pub parallel_check: bool,
    /// x402 machine-to-machine payment inspection (Arc target).
    pub x402_gate: bool,
    /// Gas is paid in native USDC, not the chain token (Arc target).
    pub usdc_native_gas: bool,
    /// Attach a local-LLM brief by default (Agentmaxxing target).
    pub ai_brief_default: bool,
    /// Human note shown in banners / docs.
    pub notes: &'static str,
}

impl ChainProfile {
    pub fn ethereum() -> Self {
        Self {
            kind: ChainKind::Ethereum,
            key: "ethereum",
            rpc_url: "https://eth.llamarpc.com".to_string(),
            chain_id: 1,
            block_gas_limit: 30_000_000,
            default_gas_price_wei: 1_000_000_000,
            parallel_check: false,
            x402_gate: false,
            usdc_native_gas: false,
            ai_brief_default: true,
            notes: "Default profile: Ethereum mainnet state via public RPC.",
        }
    }

    /// Monad: parallel-execution EVM, ~300ms block times, very high
    /// throughput. Simulation runs concurrently per tx.
    pub fn monad() -> Self {
        Self {
            kind: ChainKind::Monad,
            key: "monad",
            rpc_url: "https://rpc.monad.xyz".to_string(),
            chain_id: 143,
            block_gas_limit: 150_000_000,
            default_gas_price_wei: 1_000_000_000,
            parallel_check: true,
            x402_gate: false,
            usdc_native_gas: false,
            ai_brief_default: true,
            notes: "Monad mainnet: concurrent pre-flight sim aligned to parallel execution.",
        }
    }

    /// Arc (Circle): USDC is the native gas token; machine-to-machine
    /// payments carry x402 payload headers inspected pre-execution.
    pub fn arc() -> Self {
        Self {
            kind: ChainKind::Arc,
            key: "arc",
            rpc_url: "https://rpc.arc.network".to_string(),
            // NOTE: verify against current Circle Arc docs; override with --chain-id.
            chain_id: 5042002,
            block_gas_limit: 30_000_000,
            default_gas_price_wei: 1_000_000,
            parallel_check: false,
            x402_gate: true,
            usdc_native_gas: true,
            ai_brief_default: true,
            notes: "Arc: USDC-native gas accounting + x402 payment inspection.",
        }
    }

    /// Base Sepolia: agent-wallet execution testbed (Agentmaxxing target).
    pub fn base() -> Self {
        Self {
            kind: ChainKind::Base,
            key: "base",
            rpc_url: "https://sepolia.base.org".to_string(),
            chain_id: 84532,
            block_gas_limit: 30_000_000,
            default_gas_price_wei: 1_000_000_000,
            parallel_check: false,
            x402_gate: false,
            usdc_native_gas: false,
            ai_brief_default: true,
            notes: "Base Sepolia: self-custodial agent wallet firewall testbed.",
        }
    }

    pub fn from_flag(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "ethereum" | "eth" | "mainnet" => Some(Self::ethereum()),
            "monad" => Some(Self::monad()),
            "arc" => Some(Self::arc()),
            "base" | "base-sepolia" | "sepolia" => Some(Self::base()),
            _ => None,
        }
    }

    pub fn with_rpc_url(mut self, url: String) -> Self {
        self.rpc_url = url;
        self
    }

    pub fn with_chain_id(mut self, id: u64) -> Self {
        self.chain_id = id;
        self
    }
}

impl Default for ChainProfile {
    fn default() -> Self {
        Self::ethereum()
    }
}

/// Shared HTTP client with a hard timeout. A hanging RPC must fail the
/// gate fast — never hang it (a stalled upstream is a denial of service
/// on the firewall itself).
pub fn http_client() -> reqwest::Client {
    let secs: u64 = std::env::var("RPC_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(25);
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(secs))
        .build()
        .unwrap_or_else(|_| reqwest::Client::new())
}

/// Parsed CLI options (manual parsing — no new dependencies).
#[derive(Debug, Clone)]
pub struct CliOptions {
    pub profile: ChainProfile,
    /// Read tx JSON (object or array) from file instead of stdin.
    pub sim_file: Option<String>,
    /// Attach local-LLM brief to batch output.
    pub ai_brief: bool,
    /// Force concurrent simulation in batch mode.
    pub monad_parallel_check: bool,
    /// Legacy single-tx CI mode.
    pub ci: bool,
}

impl Default for CliOptions {
    fn default() -> Self {
        Self {
            profile: ChainProfile::default(),
            sim_file: None,
            ai_brief: false,
            monad_parallel_check: false,
            ci: false,
        }
    }
}

fn next_value(args: &[String], i: &mut usize, flag: &str) -> Option<String> {
    *i += 1;
    if *i < args.len() {
        Some(args[*i].clone())
    } else {
        eprintln!("warning: {} expects a value; ignoring", flag);
        None
    }
}

/// Parse CLI args. Unknown flags are ignored with a warning (never fatal).
pub fn parse_args(args: &[String]) -> CliOptions {
    let mut opts = CliOptions::default();
    let mut rpc_override: Option<String> = None;
    let mut chain_id_override: Option<u64> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--chain" => {
                if let Some(v) = next_value(args, &mut i, "--chain") {
                    match ChainProfile::from_flag(&v) {
                        Some(p) => opts.profile = p,
                        None => eprintln!(
                            "warning: unknown --chain '{}'; expected ethereum|monad|arc|base",
                            v
                        ),
                    }
                }
            }
            "--rpc-url" => {
                if let Some(v) = next_value(args, &mut i, "--rpc-url") {
                    rpc_override = Some(v);
                }
            }
            "--chain-id" => {
                if let Some(v) = next_value(args, &mut i, "--chain-id") {
                    match v.parse::<u64>() {
                        Ok(id) => chain_id_override = Some(id),
                        Err(_) => eprintln!("warning: invalid --chain-id '{}'", v),
                    }
                }
            }
            "--sim-file" => {
                if let Some(v) = next_value(args, &mut i, "--sim-file") {
                    opts.sim_file = Some(v);
                }
            }
            "--ai-brief" => opts.ai_brief = true,
            "--monad-parallel-check" => opts.monad_parallel_check = true,
            "--ci" => opts.ci = true,
            _ => {}
        }
        i += 1;
    }

    // Profile default for --ai-brief: agent chains brief by default.
    if opts.profile.ai_brief_default && opts.sim_file.is_some() {
        opts.ai_brief = true;
    }
    // Monad profile implies parallel batch checks unless explicitly serial?
    // Keep explicit: flag OR profile enables concurrency in batch mode.
    if opts.profile.parallel_check {
        opts.monad_parallel_check = true;
    }

    // RPC precedence: --rpc-url > RPC_URL env > profile default.
    if let Some(url) = rpc_override {
        opts.profile = opts.profile.with_rpc_url(url);
    } else if let Ok(env_url) = std::env::var("RPC_URL") {
        if !env_url.trim().is_empty() {
            opts.profile = opts.profile.with_rpc_url(env_url);
        }
    }
    if let Some(id) = chain_id_override {
        opts.profile = opts.profile.with_chain_id(id);
    }

    opts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn chain_flags_resolve() {
        assert_eq!(parse_args(&argv(&["--chain", "monad"])).profile.key, "monad");
        assert_eq!(parse_args(&argv(&["--chain", "arc"])).profile.key, "arc");
        assert_eq!(parse_args(&argv(&["--chain", "base"])).profile.key, "base");
        assert_eq!(
            parse_args(&argv(&["--chain", "ethereum"])).profile.key,
            "ethereum"
        );
    }

    #[test]
    fn unknown_chain_falls_back_to_ethereum() {
        let o = parse_args(&argv(&["--chain", "solana"]));
        assert_eq!(o.profile.key, "ethereum");
    }

    #[test]
    fn monad_profile_enables_parallel() {
        let o = parse_args(&argv(&["--chain", "monad"]));
        assert!(o.profile.parallel_check);
        assert!(o.monad_parallel_check);
        assert_eq!(o.profile.chain_id, 143);
    }

    #[test]
    fn arc_profile_flags() {
        let o = parse_args(&argv(&["--chain", "arc"]));
        assert!(o.profile.x402_gate);
        assert!(o.profile.usdc_native_gas);
    }

    #[test]
    fn rpc_precedence_flag_over_profile() {
        let o = parse_args(&argv(&["--chain", "monad", "--rpc-url", "http://x:8545"]));
        assert_eq!(o.profile.rpc_url, "http://x:8545");
        assert_eq!(o.profile.key, "monad");
    }

    #[test]
    fn sim_file_and_brief_flags() {
        let o = parse_args(&argv(&["--sim-file", "tx.json", "--ai-brief"]));
        assert_eq!(o.sim_file.as_deref(), Some("tx.json"));
        assert!(o.ai_brief);
    }

    #[test]
    fn base_sepolia_chain_id() {
        let o = parse_args(&argv(&["--chain", "base"]));
        assert_eq!(o.profile.chain_id, 84532);
    }

    #[test]
    fn chain_id_override() {
        let o = parse_args(&argv(&["--chain", "arc", "--chain-id", "9999"]));
        assert_eq!(o.profile.chain_id, 9999);
    }
}
