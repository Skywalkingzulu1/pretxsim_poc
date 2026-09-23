mod inspector;
mod agent;
mod rpc_db;

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

fn simulate_transaction(rpc_url: String, tx_env: TxEnv, inspector: &mut inspector::RiskInspector) -> Result<(inspector::EVMStructuralAnalysis, ExecutionResult), Box<dyn Error + Send + Sync>> {
    let db = rpc_db::RpcCacheDB::new(rpc_url);
    let mut cached_db = CacheDB::new(db);

    let block_env = BlockEnv {
        number: U256::from(20_000_000),
        timestamp: U256::from(1700000000),
        gas_limit: U256::from(30_000_000),
        ..Default::default()
    };

    let mut cfg_env = CfgEnv::default();
    cfg_env.chain_id = 1;

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
}

impl GatekeeperServer {
    pub fn new(rpc_url: String) -> Self {
        Self {
            rpc_url,
            http_client: Client::new(),
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
}

#[async_trait]
impl LocalGatekeeperServer for GatekeeperServer {
    async fn send_transaction(&self, tx: serde_json::Value) -> Result<String, ErrorObjectOwned> {
        println!("\n[⚡ Intercepted Transaction Request]");

        let from = tx.get("from").and_then(|v| v.as_str()).unwrap_or("");
        let to = tx.get("to").and_then(|v| v.as_str()).unwrap_or("");
        let value = tx.get("value").and_then(|v| v.as_str()).unwrap_or("0x0");
        let data = tx.get("data").and_then(|v| v.as_str()).unwrap_or("0x");
        let gas = tx.get("gas").and_then(|v| v.as_str()).unwrap_or("0x5208");
        let gas_price = tx.get("gasPrice").and_then(|v| v.as_str()).unwrap_or("0x3b9aca00");
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
            chain_id: Some(1),
            ..Default::default()
        };

        let rpc_url = self.rpc_url.clone();
        let mut inspector = inspector::RiskInspector::default();

        let (analysis, exec_result) = tokio::task::spawn_blocking(move || {
            simulate_transaction(rpc_url, tx_env, &mut inspector)
        }).await
        .map_err(|e| ErrorObjectOwned::owned(-32603, format!("Task join error: {}", e), None::<()>))?
        .map_err(|e| ErrorObjectOwned::owned(-32603, format!("Simulation failed: {}", e), None::<()>))?;

        let summary = serde_json::to_string_pretty(&analysis).unwrap();
        println!("[*] Structural Analysis Extracted:\n{}", summary);

        println!("\n[*] Consulting Local LLM Agent...");
        let advice = agent::query_local_agent(&summary).await.unwrap_or_else(|_| "Agent offline".to_string());

        println!("\n================ SECURITY AGENT BRIEF ================");
        println!("{}", advice);
        println!("======================================================");

        if exec_result.is_success() {
            println!("\n[✓] Simulation succeeded");
        } else {
            println!("\n[✗] Simulation reverted/failed");
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let rpc_url = std::env::var("RPC_URL").unwrap_or_else(|_| "https://eth.llamarpc.com".to_string());

    println!("======================================================");
    println!("  PreTxSim Micro-Agent Proxy Live on 127.0.0.1:8545");
    println!("  Upstream RPC: {}", rpc_url);
    println!("  RAM Usage target: < 800 MB");
    println!("======================================================");

    let server = ServerBuilder::default().build("127.0.0.1:8545").await?;
    let handle = server.start(GatekeeperServer::new(rpc_url).into_rpc());

    handle.stopped().await;
    Ok(())
}