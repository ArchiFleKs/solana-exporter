use crate::account::Account;
use anyhow::Result;
use serde::Deserialize;
use solana_client::{
    rpc_client::RpcClient, rpc_config::RpcBlockProductionConfig,
    rpc_config::RpcGetVoteAccountsConfig, rpc_request::RpcError,
};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Validator {
    pub description: Option<String>,
    pub validator_identity_account: Account,
    pub validator_vote_account: Account,
}

struct ValidatorInfo {
    commission: u8,
    last_vote: u64,
    root_slot: u64,
    credits: u32,
    epoch_credits: u64,
    activated_state: u64,
    version: u32,
    delinquent: bool,
    skip_rate: f64,
}

impl Validator {
    pub fn get_balance(&self, rpc_client: &RpcClient) -> Result<HashMap<String, f64>> {
        let mut balances = HashMap::new();
        balances.insert(
            String::from("vote_account_balance"),
            self.validator_vote_account
                .get_account_balance_as_sol(&rpc_client)?,
        );
        balances.insert(
            String::from("identity_account_balance"),
            self.validator_identity_account
                .get_account_balance_as_sol(&rpc_client)?,
        );
        Ok(balances)
    }

    // pub fn get_validator_info(&self, rpc_client: &RpcClient) -> ValidatorInfo {
    //     let vote_account = rpc_client.get_vote_accounts_with_config(
    //         RpcGetVoteAccountsConfig {
    //             vote_pubkey: Some(self.validator_vote_account.pubkey),
    //             ..RpcGetVoteAccountsConfig::default()
    //         }
    //     ).unwrap().current[0];
    //     let epoch_info = rpc_client.get_epoch_info().unwrap();
    //     let skip_rate = rpc_client.get_block_production_with_config(
    //         RpcBlockProductionConfig {
    //             identity: Some(self.validator_identity_account.pubkey),
    //             ..RpcBlockProductionConfig::default()
    //         }
    //     ).ok().map(|result| {
    //         result
    //             .value
    //             .by_identity
    //             .into_iter()
    //             .map(|(identity, (leader_slots, blocks_produced))| {
    //                 (
    //                     identity,
    //                     100. * (leader_slots.saturating_sub(blocks_produced)) as f64
    //                         / leader_slots as f64,
    //                 )
    //             })
    //             .collect()
    //     })
    //     .unwrap_or_default();
    //     let node_version = rpc_client
    //         .get_cluster_nodes().unwrap()
    //         .into_iter()
    //         .filter(|contact_info| contact_info.pubkey == self.validator_identity_account.pubkey)
    //         .map(|contact_info| {
    //             (
    //                 contact_info.pubkey,
    //                 contact_info
    //                     .version
    //             )
    //         })
    //         .next().unwrap();
    //     ValidatorInfo {
    //         commission: vote_account.commission,
    //         last_vote: vote_account.last_vote,
    //         root_slot: vote_account.root_slot,
    //         credits: vote_account,
    //         epoch_credits: vote_account.epoch_credits.iter().filter(|&x| x.0 == epoch_info.epoch).map(|&x| x.1).next().unwrap(),
    //         activated_state: vote_account.activated_stake,
    //         version: node_version,
    //         skip_rate: skip_rate
    //     }
}
