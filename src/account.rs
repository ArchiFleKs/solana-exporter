use anyhow::{anyhow, Result};
use serde::Deserialize;
use solana_client::rpc_client::RpcClient;
use solana_program::{native_token::lamports_to_sol, pubkey::Pubkey};
use std::str::FromStr;

#[derive(Deserialize)]
pub struct Account {
    pub description: Option<String>,
    pub pubkey: String,
}

impl Account {
    pub fn get_account_balance_as_sol(&self, rpc_client: &RpcClient) -> Result<f64> {
        match rpc_client.get_balance(&Pubkey::from_str(&self.pubkey)?) {
            Ok(balance) => Ok(lamports_to_sol(balance)),
            Err(error) => Err(anyhow!(
                "There was an issue querying the balance: {}",
                error
            )),
        }
    }
    pub fn get_account_balance_as_lamport(&self, rpc_client: &RpcClient) -> Result<u64> {
        match rpc_client.get_balance(&Pubkey::from_str(&self.pubkey)?) {
            Ok(balance) => Ok(balance),
            Err(error) => Err(anyhow!(
                "There was an issue querying the balance: {}",
                error
            )),
        }
    }
}
