use revm::{
    interpreter::Interpreter,
    EvmContext, Inspector, Database, DatabaseRef,
    primitives::{Address, U256, B256, TxKind, Log},
};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct BalanceChange {
    pub address: String,
    pub pre_balance: String,
    pub post_balance: String,
    pub delta: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct AllowanceChange {
    pub owner: String,
    pub spender: String,
    pub token: String,
    pub pre_allowance: String,
    pub post_allowance: String,
    pub delta: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct TokenTransfer {
    pub token: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub token_type: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct DelegationChange {
    pub address: String,
    pub old_code_hash: Option<String>,
    pub new_code_hash: Option<String>,
    pub delegated_to: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct StorageDiff {
    pub address: String,
    pub slot: String,
    pub pre_value: String,
    pub post_value: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct EmittedLog {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct EVMStructuralAnalysis {
    pub delegatecalls_detected: usize,
    pub storage_slots_written: usize,
    pub selfdestruct_triggered: bool,
    pub eth_transferred_wei: String,
    pub balance_changes: Vec<BalanceChange>,
    pub allowance_changes: Vec<AllowanceChange>,
    pub token_transfers: Vec<TokenTransfer>,
    pub delegation_changes: Vec<DelegationChange>,
    pub storage_diffs: Vec<StorageDiff>,
    pub logs_emitted: Vec<EmittedLog>,
}

pub struct RiskInspector {
    pub analysis: EVMStructuralAnalysis,
    pre_balances: HashMap<Address, U256>,
    pre_code_hashes: HashMap<Address, B256>,
    pre_storage: HashMap<(Address, U256), U256>,
    pre_allowances: HashMap<(Address, Address, Address), U256>,
}

impl Default for RiskInspector {
    fn default() -> Self {
        Self {
            analysis: EVMStructuralAnalysis::default(),
            pre_balances: HashMap::new(),
            pre_code_hashes: HashMap::new(),
            pre_storage: HashMap::new(),
            pre_allowances: HashMap::new(),
        }
    }
}

impl<DB: Database + DatabaseRef> Inspector<DB> for RiskInspector {
    fn initialize_interp(&mut self, _interp: &mut Interpreter, context: &mut EvmContext<DB>) {
        let tx = &context.env.tx;
        let from = tx.caller;
        let to = match tx.transact_to {
            TxKind::Call(addr) => addr,
            TxKind::Create => Address::ZERO,
        };

        let db = &context.db;
        if let Ok(Some(acc)) = db.basic_ref(from) {
            self.pre_balances.insert(from, acc.balance);
        }
        if to != Address::ZERO {
            if let Ok(Some(acc)) = db.basic_ref(to) {
                self.pre_balances.insert(to, acc.balance);
                // Store code hash for delegation detection
                self.pre_code_hashes.insert(to, acc.code_hash);
            }
        }
    }

    fn step(&mut self, interp: &mut Interpreter, _context: &mut EvmContext<DB>) {
        let opcode = interp.current_opcode();
        match opcode {
            0xF4 => self.analysis.delegatecalls_detected += 1,
            0x55 => self.analysis.storage_slots_written += 1,
            0xFF => self.analysis.selfdestruct_triggered = true,
            _ => {}
        }
    }

    fn log(&mut self, _interp: &mut Interpreter, _context: &mut EvmContext<DB>, log: &Log) {
        self.analysis.logs_emitted.push(EmittedLog {
            address: format!("{:#x}", log.address),
            topics: log.topics().iter().map(|t| format!("{:#x}", t)).collect(),
            data: format!("0x{}", hex::encode(log.data.data.as_ref())),
        });
    }
}

impl RiskInspector {
    pub fn capture_post_state<DB: Database + DatabaseRef>(&mut self, context: &mut EvmContext<DB>) {
        let tx = &context.env.tx;
        let from = tx.caller;
        let to = match tx.transact_to {
            TxKind::Call(addr) => addr,
            _ => Address::ZERO,
        };

        let db = &context.db;
        for addr in [from, to] {
            if addr == Address::ZERO { continue; }
            if let Ok(Some(acc)) = db.basic_ref(addr) {
                let post = acc.balance;
                let pre = self.pre_balances.remove(&addr).unwrap_or_default();
                if pre != post {
                    let delta = if post > pre { format!("+{}", post - pre) } else { format!("-{}", pre - post) };
                    self.analysis.balance_changes.push(BalanceChange {
                        address: format!("{:#x}", addr),
                        pre_balance: pre.to_string(),
                        post_balance: post.to_string(),
                        delta,
                    });
                }
            }
        }

        // Allowance tracking requires pre-populating storage slots - simplified for now
        // let allowances_to_check: Vec<_> = self.pre_allowances.keys().cloned().collect();
        // for (token, owner, spender) in allowances_to_check {
        //     if let Ok(storage) = db.storage_ref(token, spender) {
        //         let post: U256 = storage;
        //         let pre = self.pre_allowances.remove(&(token, owner, spender)).unwrap_or_default();
        //         if pre != post {
        //             let delta = if post > pre { format!("+{}", post - pre) } else { format!("-{}", pre - post) };
        //             self.analysis.allowance_changes.push(AllowanceChange {
        //                 owner: format!("{:#x}", owner),
        //                 spender: format!("{:#x}", spender),
        //                 token: format!("{:#x}", token),
        //                 pre_allowance: pre.to_string(),
        //                 post_allowance: post.to_string(),
        //                 delta,
        //             });
        //         }
        //     }
        // }
    }

    pub fn detect_delegations<DB: Database + DatabaseRef>(&mut self, context: &mut EvmContext<DB>) {
        let tx = &context.env.tx;
        let to = match tx.transact_to {
            TxKind::Call(addr) => addr,
            _ => return,
        };

        let db = &context.db;
        if let Ok(Some(acc)) = db.basic_ref(to) {
            let new_hash = acc.code_hash;
            if let Some(old_hash) = self.pre_code_hashes.get(&to) {
                if *old_hash != new_hash {
                    // Fetch the new code to check for EIP-7702 delegation
                    let delegated_to = if let Ok(code) = db.code_by_hash_ref(new_hash) {
                        let bytes = code.original_bytes();
                        if bytes.len() >= 22 && bytes[0] == 0xEF && bytes[1] == 0x01 {
                            Some(format!("{:#x}", Address::from_slice(&bytes[2..22])))
                        } else { None }
                    } else { None };

                    self.analysis.delegation_changes.push(DelegationChange {
                        address: format!("{:#x}", to),
                        old_code_hash: Some(format!("{:#x}", old_hash)),
                        new_code_hash: Some(format!("{:#x}", new_hash)),
                        delegated_to,
                    });
                }
            }
        }
    }
}

const ERC20_TRANSFER: &[u8] = &[0xdd, 0xf2, 0x52, 0xad];
const ERC20_APPROVAL: &[u8] = &[0x8c, 0x5b, 0xe1, 0xa5];
const ERC721_TRANSFER: &[u8] = &[0xdd, 0xf2, 0x52, 0xad];
const ERC1155_SINGLE: &[u8] = &[0xc3, 0xd5, 0x81, 0xb9];
const ERC1155_BATCH: &[u8] = &[0x4a, 0x39, 0x3e, 0xe0];