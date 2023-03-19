use metrics::{AccountExporter, ValidatorExporter};
use solana_client::rpc_client::RpcClient;
use clap::Parser;
use solana_client::rpc_config::RpcGetVoteAccountsConfig;

mod cli;
mod config;
mod validator;
mod account;
mod metrics;

use crate::cli::Cli;
use crate::config::Config;

fn main() {
    let args: Cli = Cli::parse();
    let config: Config = Config::new(&args);
    let rpc_client = RpcClient::new(&config.rpc_url);
    let binding = Config::binding(&config).parse().unwrap();
    let _exporter = prometheus_exporter::start(binding).unwrap();

    let accounts_exporter: AccountExporter = AccountExporter::new();
    let validators_exporter: ValidatorExporter = ValidatorExporter::new();

    let validators_info = rpc_client.get_vote_accounts_with_config(RpcGetVoteAccountsConfig { vote_pubkey: Some(config.validators[0].validator_vote_account.pubkey.clone()), ..RpcGetVoteAccountsConfig::default() });

    println!("{:?}", validators_info);
}

