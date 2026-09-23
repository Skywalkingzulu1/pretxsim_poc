use revm::{
    primitives::{Address, U256, B256, Bytecode, AccountInfo},
    Database, DatabaseRef,
};
use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct RpcCacheDB {
    client: Client,
    rpc_url: String,
    block_number: u64,
    cache: HashMap<Address, Option<AccountInfo>>,
    code_cache: HashMap<B256, Bytecode>,
    storage_cache: HashMap<(Address, U256), U256>,
}

impl RpcCacheDB {
    pub fn new(rpc_url: String) -> Self {
        Self {
            client: Client::new(),
            rpc_url,
            block_number: 0,
            cache: HashMap::new(),
            code_cache: HashMap::new(),
            storage_cache: HashMap::new(),
        }
    }

    pub fn set_block_number(&mut self, block_number: u64) {
        self.block_number = block_number;
    }

    async fn fetch_rpc_value(&self, method: &str, params: Vec<Value>) -> Result<Value, String> {
        let payload = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        });

        let res = self.client
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

    fn block_on<F>(&self, f: F) -> F::Output
    where
        F: std::future::Future + Send,
        F::Output: Send,
    {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(f)
        })
    }
}

impl DatabaseRef for RpcCacheDB {
    type Error = String;

    fn basic_ref(&self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        if let Some(cached) = self.cache.get(&address) {
            return Ok(cached.clone());
        }

        let block_tag = format!("0x{:x}", self.block_number);
        let result = self.block_on(self.fetch_rpc_value("eth_getAccount", vec![
            json!(format!("{:#x}", address)),
            json!(block_tag),
        ]))?;

        if result.is_null() {
            return Ok(None);
        }

        let balance = U256::from_str_radix(
            result["balance"].as_str().unwrap_or("0x0").trim_start_matches("0x"),
            16
        ).unwrap_or_default();

        let nonce = u64::from_str_radix(
            result["nonce"].as_str().unwrap_or("0x0").trim_start_matches("0x"),
            16
        ).unwrap_or_default();

        let code_hash_str = result["codeHash"].as_str().unwrap_or("");
        let code_hash = if code_hash_str.is_empty() || code_hash_str == "0x" {
            B256::ZERO
        } else {
            B256::from_str(code_hash_str).unwrap_or_default()
        };

        let code = if code_hash != B256::ZERO {
            if let Some(cached_code) = self.code_cache.get(&code_hash) {
                cached_code.clone()
            } else {
                let code_result = self.block_on(self.fetch_rpc_value("eth_getCodeByHash", vec![
                    json!(format!("{:#x}", code_hash)),
                ]))?;
                let code_bytes = hex::decode(code_result.as_str().unwrap_or("0x").trim_start_matches("0x")).unwrap_or_default();
                let bytecode = Bytecode::new_raw(code_bytes.into());
                bytecode
            }
        } else {
            Bytecode::default()
        };

        let code_hash = code.hash_slow();
        let account_info = AccountInfo::new(balance, nonce, code_hash, code);
        Ok(Some(account_info))
    }

    fn code_by_hash_ref(&self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        if let Some(cached) = self.code_cache.get(&code_hash) {
            return Ok(cached.clone());
        }

        let result = self.block_on(self.fetch_rpc_value("eth_getCodeByHash", vec![
            json!(format!("{:#x}", code_hash)),
        ]))?;

        let code_bytes = hex::decode(result.as_str().unwrap_or("0x").trim_start_matches("0x")).unwrap_or_default();
        let bytecode = Bytecode::new_raw(code_bytes.into());
        Ok(bytecode)
    }

    fn storage_ref(&self, address: Address, index: U256) -> Result<U256, Self::Error> {
        let key = (address, index);
        if let Some(cached) = self.storage_cache.get(&key) {
            return Ok(*cached);
        }

        let block_tag = format!("0x{:x}", self.block_number);
        let result = self.block_on(self.fetch_rpc_value("eth_getStorageAt", vec![
            json!(format!("{:#x}", address)),
            json!(format!("0x{:x}", index)),
            json!(block_tag),
        ]))?;

        let value = U256::from_str_radix(
            result.as_str().unwrap_or("0x0").trim_start_matches("0x"),
            16
        ).unwrap_or_default();

        Ok(value)
    }

    fn block_hash_ref(&self, number: u64) -> Result<B256, Self::Error> {
        let result = self.block_on(self.fetch_rpc_value("eth_getBlockByNumber", vec![
            json!(format!("0x{:x}", number)),
            json!(false),
        ]))?;

        let hash_str = result["hash"].as_str().unwrap_or("");
        let hash = B256::from_str(hash_str).unwrap_or_default();
        Ok(hash)
    }
}

impl Database for RpcCacheDB {
    type Error = String;

    #[inline]
    fn basic(&mut self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        <Self as DatabaseRef>::basic_ref(self, address)
    }

    #[inline]
    fn code_by_hash(&mut self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        <Self as DatabaseRef>::code_by_hash_ref(self, code_hash)
    }

    #[inline]
    fn storage(&mut self, address: Address, index: U256) -> Result<U256, Self::Error> {
        <Self as DatabaseRef>::storage_ref(self, address, index)
    }

    #[inline]
    fn block_hash(&mut self, number: u64) -> Result<B256, Self::Error> {
        <Self as DatabaseRef>::block_hash_ref(self, number)
    }
}