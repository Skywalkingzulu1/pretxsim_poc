mod inspector;
mod agent;
mod rpc_db;
mod config;
mod arc_x402_gate;
mod ollama_risk_brief;

use jsonrpsee::core::async_trait;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::ServerBuilder;
use jsonrpsee::types::ErrorObjectOwned;
use revm::{
    db::CacheDB,
    primitives::{Address, TxKind, U256, Bytes, BlockEnv, CfgEnv, TxEnv, Env, ExecutionResult},
    Evm,
    inspector_handle_register,
};
use reqwest::Client;
use serde_json::{json, Value};
use std::str::FromStr;
use std::error::Error;
use std::env;

#[rpc(server)]
pub trait LocalGatekeeper {
    #[method(name = "eth_sendTransaction")]
    async fn send_transaction(&self, tx: serde_json::Value) -> Result<String, ErrorObjectOwned>;

    #[method(name = "eth_blockNumber")]
    async fn block_number(&self) -> Result<String, ErrorObjectOwned>;

    #[method(name = "eth_getBalance")]
    async fn get_balance(&self, address: String, block: String) -> Result<String, ErrorObjectOwned>;
}

#[derive(Debug)]
struct SimError(String);

impl std::fmt::Display for SimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for SimError {}

fn simulate_transaction(
    rpc_url: String,
    tx_env: TxEnv,
    inspector: &mut inspector::RiskInspector,
    profile: config::ChainProfile,
) -> Result<(inspector::EVMStructuralAnalysis, ExecutionResult), Box<dyn Error + Send + Sync>> {
    let db = rpc_db::RpcCacheDB::new(rpc_url);
    let mut cached_db = CacheDB::new(db);

    let block_env = BlockEnv {
        number: U256::from(20_000_000),
        timestamp: U256::from(1700000000),
        gas_limit: U256::from(profile.block_gas_limit),
        ..Default::default()
    };

    let mut cfg_env = CfgEnv::default();
    cfg_env.chain_id = profile.chain_id;

    let env = Box::new(Env {
        cfg: cfg_env,
        block: block_env,
        tx: tx_env,
    });

    let mut evm = Evm::builder()
        .with_db(cached_db)
        .with_external_context(inspector)
        .append_handler_register(inspector_handle_register)
        .with_env(env)
        .build();

    let result = evm.transact().map_err(|e| SimError(format!("{}", e)))?;

    // Capture post-execution state using the evm context
    let context = &mut evm.context.evm;
    let inspector = &mut evm.context.external;
    inspector.capture_post_state(context);
    inspector.detect_delegations(context);

    Ok((inspector.analysis.clone(), result.result))
}

pub struct GatekeeperServer {
    rpc_url: String,
    http_client: Client,
    profile: config::ChainProfile,
}

impl GatekeeperServer {
    pub fn new(rpc_url: String) -> Self {
        let profile = config::ChainProfile::ethereum().with_rpc_url(rpc_url.clone());
        Self {
            rpc_url,
            http_client: config::http_client(),
            profile,
        }
    }

    pub fn with_profile(profile: config::ChainProfile) -> Self {
        let rpc_url = profile.rpc_url.clone();
        Self {
            rpc_url,
            http_client: config::http_client(),
            profile,
        }
    }

    async fn fetch_rpc_value(&self, method: &str, params: Vec<Value>) -> Result<Value, String> {
        let payload = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        });

        let res = self.http_client
            .post(&self.rpc_url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json::<Value>()
            .await
            .map_err(|e| e.to_string())?;

        Ok(res["result"].clone())
    }

    /// Parse an `eth_sendTransaction`-style JSON value, run the revm
    /// simulation with the RiskInspector, and consult the local LLM agent.
    /// Shared by the interactive proxy path and `--ci` mode.
    pub async fn simulate_from_value(&self, tx: serde_json::Value) -> Result<(inspector::EVMStructuralAnalysis, ExecutionResult, String), ErrorObjectOwned> {
        self.simulate_inner(tx, true).await
    }

    /// Core simulation. `call_agent=false` skips the raw Ollama call
    /// (batch mode substitutes the structured brief instead).
    pub async fn simulate_inner(&self, tx: serde_json::Value, call_agent: bool) -> Result<(inspector::EVMStructuralAnalysis, ExecutionResult, String), ErrorObjectOwned> {
        let from = tx.get("from").and_then(|v| v.as_str()).unwrap_or("");
        let to = tx.get("to").and_then(|v| v.as_str()).unwrap_or("");
        let value = tx.get("value").and_then(|v| v.as_str()).unwrap_or("0x0");
        let data = tx.get("data").and_then(|v| v.as_str()).unwrap_or("0x");
        let gas = tx.get("gas").and_then(|v| v.as_str()).unwrap_or("0x5208");
        let default_price = format!("0x{:x}", self.profile.default_gas_price_wei);
        let gas_price = tx.get("gasPrice").and_then(|v| v.as_str()).unwrap_or(default_price.as_str());
        let nonce = tx.get("nonce").and_then(|v| v.as_str()).unwrap_or("0x0");

        let from_addr = Address::from_str(from)
            .unwrap_or_else(|_| Address::from_str("0x1111111111111111111111111111111111111111").unwrap());

        let to_addr = if to.is_empty() {
            TxKind::Create
        } else {
            TxKind::Call(Address::from_str(to).unwrap_or(Address::ZERO))
        };

        let value_u256 = U256::from_str_radix(value.trim_start_matches("0x"), 16).unwrap_or(U256::ZERO);
        let data_bytes = Bytes::from_str(data).unwrap_or_default();
        let gas_u64 = u64::from_str_radix(gas.trim_start_matches("0x"), 16).unwrap_or(21000);
        let gas_price_u256 = U256::from_str_radix(gas_price.trim_start_matches("0x"), 16).unwrap_or(U256::from(1_000_000_000));
        let nonce_u64 = u64::from_str_radix(nonce.trim_start_matches("0x"), 16).unwrap_or(0);

        let tx_env = TxEnv {
            caller: from_addr,
            transact_to: to_addr,
            value: value_u256,
            data: data_bytes,
            gas_limit: gas_u64,
            gas_price: gas_price_u256,
            nonce: Some(nonce_u64),
            chain_id: Some(self.profile.chain_id),
            ..Default::default()
        };

        let rpc_url = self.rpc_url.clone();
        let profile = self.profile.clone();
        let mut inspector = inspector::RiskInspector::default();

        let (analysis, exec_result) = tokio::task::spawn_blocking(move || {
            simulate_transaction(rpc_url, tx_env, &mut inspector, profile)
        }).await
        .map_err(|e| ErrorObjectOwned::owned(-32603, format!("Task join error: {}", e), None::<()>))?
        .map_err(|e| ErrorObjectOwned::owned(-32603, format!("Simulation failed: {}", e), None::<()>))?;

        let advice = if call_agent {
            let summary = serde_json::to_string_pretty(&analysis).unwrap_or_else(|_| "{}".to_string());
            agent::query_local_agent(&summary).await.unwrap_or_else(|_| "Agent offline".to_string())
        } else {
            String::new()
        };

        Ok((analysis, exec_result, advice))
    }
}

/// Deterministic BLOCK heuristics shared by --ci and --sim-file modes.
pub fn is_structural_block(a: &inspector::EVMStructuralAnalysis) -> bool {
    a.delegatecalls_detected > 0
        || a.selfdestruct_triggered
        || !a.delegation_changes.is_empty()
        || a.storage_slots_written > 10
}

/// Run one tx end-to-end for batch mode: simulate, optionally brief with
/// the local LLM, optionally inspect x402 (Arc). Returns a JSON object
/// with `blocked` set. The LLM brief is advisory only — it never gates.
pub async fn run_single_tx(
    profile: config::ChainProfile,
    tx: serde_json::Value,
    want_brief: bool,
) -> serde_json::Value {
    let t0 = std::time::Instant::now();
    let server = GatekeeperServer::with_profile(profile.clone());
    let mut out = serde_json::Map::new();

    match server.simulate_inner(tx.clone(), !want_brief).await {
        Ok((analysis, exec_result, advice)) => {
            let structural_block = is_structural_block(&analysis);

            // Arc x402 gate: machine-to-machine payment inspection.
            let (x402_val, x402_block) = if profile.x402_gate {
                match arc_x402_gate::extract(&tx) {
                    Some(p) => {
                        let insp = arc_x402_gate::inspect(&p, &arc_x402_gate::GateLimits::default());
                        let b = insp.verdict == arc_x402_gate::X402Verdict::Block;
                        (serde_json::json!(insp), b)
                    }
                    None => (serde_json::Value::Null, false),
                }
            } else {
                (serde_json::Value::Null, false)
            };

            // Structured local-LLM brief (advisory).
            let brief_val = if want_brief {
                let summary =
                    serde_json::to_string(&analysis).unwrap_or_else(|_| "{}".to_string());
                let ollama_url = std::env::var("OLLAMA_URL")
                    .unwrap_or_else(|_| ollama_risk_brief::DEFAULT_OLLAMA_URL.to_string());
                let model = std::env::var("OLLAMA_MODEL")
                    .unwrap_or_else(|_| ollama_risk_brief::DEFAULT_MODEL.to_string());
                let b = ollama_risk_brief::brief_from_analysis_json(&summary, &ollama_url, &model)
                    .await;
                serde_json::json!({
                    "verdict": b.verdict.as_str(),
                    "summary": b.summary,
                    "model": b.model,
                })
            } else {
                serde_json::Value::Null
            };

            let blocked = structural_block || x402_block;
            out.insert("chain".to_string(), serde_json::json!(profile.key));
            out.insert("analysis".to_string(), serde_json::json!(analysis));
            out.insert(
                "execution_success".to_string(),
                serde_json::json!(exec_result.is_success()),
            );
            out.insert("gas_used".to_string(), serde_json::json!(exec_result.gas_used()));
            if profile.usdc_native_gas {
                // Arc prices gas in native USDC; units below are gas units.
                out.insert("gas_asset".to_string(), serde_json::json!("USDC"));
            }
            if !want_brief {
                out.insert("advice".to_string(), serde_json::json!(advice));
            }
            out.insert("brief".to_string(), brief_val);
            out.insert("x402".to_string(), x402_val);
            out.insert("blocked".to_string(), serde_json::json!(blocked));
        }
        Err(e) => {
            out.insert("chain".to_string(), serde_json::json!(profile.key));
            out.insert(
                "error".to_string(),
                serde_json::json!({ "error": e.message(), "code": e.code() }),
            );
            out.insert("blocked".to_string(), serde_json::json!(true));
        }
    }
    out.insert(
        "elapsed_ms".to_string(),
        serde_json::json!(t0.elapsed().as_millis() as u64),
    );
    serde_json::Value::Object(out)
}

#[async_trait]
impl LocalGatekeeperServer for GatekeeperServer {
    async fn send_transaction(&self, tx: serde_json::Value) -> Result<String, ErrorObjectOwned> {
        println!("\n[*] Intercepted Transaction Request");

        let (analysis, exec_result, advice) = self.simulate_from_value(tx).await?;

        let summary = serde_json::to_string_pretty(&analysis).unwrap_or_else(|_| "{}".to_string());
        println!("[*] Structural Analysis Extracted:\n{}", summary);

        println!("\n================ SECURITY AGENT BRIEF ================");
        println!("{}", advice);
        println!("======================================================");

        if exec_result.is_success() {
            println!("\n[+] Simulation succeeded");
        } else {
            println!("\n[-] Simulation reverted/failed");
        }

        println!("\nProceed with broadcast to network? [y/N]: ");
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);

        if input.trim().eq_ignore_ascii_case("y") {
            Ok(format!("0x{:x}", exec_result.gas_used()))
        } else {
            Err(ErrorObjectOwned::owned(-32603, "Transaction aborted by user.", None::<()>))
        }
    }

    async fn block_number(&self) -> Result<String, ErrorObjectOwned> {
        match self.fetch_rpc_value("eth_blockNumber", vec![]).await {
            Ok(val) => Ok(val.as_str().unwrap_or("0x0").to_string()),
            Err(e) => Err(ErrorObjectOwned::owned(-32603, e, None::<()>)),
        }
    }

    async fn get_balance(&self, address: String, _block: String) -> Result<String, ErrorObjectOwned> {
        match self.fetch_rpc_value("eth_getBalance", vec![json!(address), json!("latest")]).await {
            Ok(val) => Ok(val.as_str().unwrap_or("0x0").to_string()),
            Err(e) => Err(ErrorObjectOwned::owned(-32603, e, None::<()>)),
        }
    }
}

/// Batch mode: read tx JSON (object or array) from --sim-file, simulate
/// each, print a JSON array. Concurrent when the profile/flag requests it
/// (--monad-parallel-check). Exit 1 if any tx is blocked. Never prompts.
///
/// Example: cargo run -- --chain monad --sim-file tx_payload.json --ai-brief
async fn run_batch(opts: config::CliOptions) -> Result<(), Box<dyn Error>> {
    let path = opts.sim_file.clone().unwrap_or_default();
    let data = std::fs::read_to_string(&path)
        .map_err(|e| format!("cannot read sim file '{}': {}", path, e))?;
    // Tolerate UTF-8 BOM (e.g. files saved on Windows).
    let data = data.strip_prefix('\u{FEFF}').unwrap_or(&data);
    let parsed: serde_json::Value =
        serde_json::from_str(data).map_err(|e| format!("sim file is not JSON: {}", e))?;
    let txs: Vec<serde_json::Value> = match parsed {
        serde_json::Value::Array(a) => a,
        single => vec![single],
    };
    if txs.is_empty() {
        println!("[]");
        return Ok(());
    }

    let t_all = std::time::Instant::now();
    let mut results: Vec<serde_json::Value> = Vec::with_capacity(txs.len());

    if opts.monad_parallel_check {
        // Concurrent pre-flight checks (Monad high-throughput mode).
        let mut handles = Vec::with_capacity(txs.len());
        for (idx, tx) in txs.into_iter().enumerate() {
            let profile = opts.profile.clone();
            let want_brief = opts.ai_brief;
            handles.push(tokio::spawn(async move {
                let mut r = run_single_tx(profile, tx, want_brief).await;
                if let serde_json::Value::Object(map) = &mut r {
                    map.insert("index".to_string(), serde_json::json!(idx));
                }
                r
            }));
        }
        for h in handles {
            results.push(h.await.map_err(|e| format!("batch task failed: {}", e))?);
        }
        // Restore input order (tasks may finish out of order).
        results.sort_by_key(|r| r.get("index").and_then(|v| v.as_u64()).unwrap_or(0));
    } else {
        for (idx, tx) in txs.into_iter().enumerate() {
            let mut r = run_single_tx(opts.profile.clone(), tx, opts.ai_brief).await;
            if let serde_json::Value::Object(map) = &mut r {
                map.insert("index".to_string(), serde_json::json!(idx));
            }
            results.push(r);
        }
    }

    let blocked = results
        .iter()
        .filter(|r| r.get("blocked").and_then(|v| v.as_bool()).unwrap_or(false))
        .count();
    println!("{}", serde_json::to_string_pretty(&results).unwrap());
    eprintln!(
        "pretxsim batch: {} txs, {} blocked, {}ms total (chain: {})",
        results.len(),
        blocked,
        t_all.elapsed().as_millis(),
        opts.profile.key,
    );
    if blocked > 0 {
        std::process::exit(1);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let raw_args: Vec<String> = env::args().skip(1).collect();
    let opts = config::parse_args(&raw_args);
    let profile = opts.profile.clone();
    let rpc_url = profile.rpc_url.clone();

    // Batch file mode (Monad parallel checks, Arc x402, briefs).
    if opts.sim_file.is_some() {
        return run_batch(opts).await;
    }

    // CI mode: read a single tx JSON from stdin, simulate, print structured
    // JSON to stdout, exit 1 when BLOCK heuristics trigger. Never prompts.
    // Example: type examples/demo_txs.json | pretxsim_poc --ci
    if opts.ci {
        let mut input = String::new();
        use std::io::Read;
        std::io::stdin().read_to_string(&mut input).unwrap_or_default();
        let trimmed = input.trim();
        let tx: serde_json::Value = if trimmed.is_empty() {
            serde_json::json!({})
        } else {
            serde_json::from_str(trimmed).unwrap_or_else(|_| serde_json::json!({}))
        };
        let server = GatekeeperServer::with_profile(profile);
        match server.simulate_from_value(tx).await {
            Ok((analysis, exec_result, advice)) => {
                let out = serde_json::json!({
                    "analysis": analysis,
                    "execution_success": exec_result.is_success(),
                    "advice": advice,
                });
                println!("{}", serde_json::to_string_pretty(&out).unwrap());
                if is_structural_block(&analysis) {
                    std::process::exit(1);
                }
                return Ok(());
            }
            Err(e) => {
                let out = serde_json::json!({ "error": e.message(), "code": e.code() });
                eprintln!("{}", serde_json::to_string(&out).unwrap());
                std::process::exit(1);
            }
        }
    }

    println!("======================================================");
    println!("  PreTxSim Micro-Agent Proxy Live on 127.0.0.1:8545");
    println!("  Chain profile: {} (chain id {})", profile.key, profile.chain_id);
    println!("  Upstream RPC: {}", rpc_url);
    if profile.x402_gate {
        println!("  x402 gate: ON (USDC-native gas)");
    }
    if profile.parallel_check {
        println!("  Parallel pre-flight checks: ON");
    }
    println!("  RAM Usage target: < 800 MB");
    println!("  Note: {}", profile.notes);
    println!("======================================================");

    let server = ServerBuilder::default().build("127.0.0.1:8545").await?;
    let handle = server.start(GatekeeperServer::with_profile(profile).into_rpc());

    handle.stopped().await;
    Ok(())
}
