use serde::Deserialize;
use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey::Pubkey,native_token::lamports_to_sol};
use std::str::FromStr;

#[derive(Deserialize)]
pub struct Account {
    pub description: Option<String>,
    pub pubkey: String,
}

impl Account {
    pub fn get_account_balance_as_sol(&self, rpc_client: &RpcClient) -> f64 {
        match rpc_client.get_balance(&Pubkey::from_str(&self.pubkey).unwrap()) {
            Ok(balance) => lamports_to_sol(balance),
            Err(error) => {
                panic!("There was an issue querying the balance for {}", error)
            }
        }
    }
    pub fn get_account_balance_as_lamport(&self, rpc_client: &RpcClient) -> u64 {
        match rpc_client.get_balance(&Pubkey::from_str(&self.pubkey).unwrap()) {
            Ok(balance) => balance,
            Err(error) => {
                panic!("There was an issue querying the balance for {}", error)
            }
        }
    }
}
