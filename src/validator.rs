use std::collections::HashMap;

use serde::Deserialize;
use solana_client::rpc_client::RpcClient;
use crate::account::Account;

#[derive(Deserialize)]
pub struct Validator {
    pub description: Option<String>,
    pub validator_identity_account: Account,
    pub validator_vote_account: Account
}

struct ValidatorInfo {
    commission: u32,
    last_vote: u64,
    root_slot: u64,
    credits: u32,
    epoch_credits: u32,
    activated_state: u32,
    version: u32,
    delinquent: bool,
    skip_rate: f64
}

impl Validator {
    pub fn get_balance(&self, rpc_client: &RpcClient) -> HashMap<String, f64> {
        let mut balances = HashMap::new();
        balances.insert(String::from("vote_account_balance"), self.validator_vote_account.get_account_balance_as_sol(&rpc_client));
        balances.insert(String::from("identity_account_balance"), self.validator_identity_account.get_account_balance_as_sol(&rpc_client));
        balances
    }
 //    pub fn get_validator_info(&self, rpc_client: &RpcClient) -> ValidatorInfo {
 //        ValidatorInfo {
 // HH           rcp_client.get_validator_info

 //        }
 //    }
}    
