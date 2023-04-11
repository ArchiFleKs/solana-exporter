use anyhow::{anyhow, Result};
use clap::Parser;
use metrics::{AccountExporter, ValidatorExporter};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcGetVoteAccountsConfig;
use std::process::exit;

mod account;
mod cli;
mod config;
mod metrics;
mod validator;

use crate::cli::Cli;
use crate::config::Config;

fn main() {
    let args = match Cli::try_parse() {
        Ok(args) => args,
        Err(error) => {
            eprintln!("Error parsing command line arguments: {}", error);
            exit(1);
        }
    };

    let config = match Config::new(&args) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Error generating config: {}", error);
            exit(2);
        }
    };

    let rpc_client = RpcClient::new(&config.rpc_url);

    let duration = std::time::Duration::from_millis(config.duration_ms);

    let binding = match Config::binding(&config).parse() {
        Ok(binding) => binding,
        Err(error) => {
            eprintln!("There was an issue parsing the binding: {}", error);
            exit(3);
        }
    };

    let exporter = match prometheus_exporter::start(binding) {
        Ok(exporter) => exporter,
        Err(error) => {
            eprintln!("There was an issue starting the exporter: {}", error);
            exit(4);
        }
    };

    let accounts_exporter = match AccountExporter::new() {
        Ok(accounts_exporter) => accounts_exporter,
        Err(error) => {
            eprintln!("There was an issue starting the exporter: {}", error);
            exit(5);
        }
    };
    loop {
        for account in &config.accounts {
            match account.get_account_balance_as_sol(&rpc_client) {
                Ok(balance) => {
                    accounts_exporter
                        .solana_account_balance
                        .with_label_values(&[
                            &account
                                .description
                                .as_ref()
                                .unwrap_or(&"unspecified".to_string()),
                            &account.pubkey,
                        ])
                        .set(balance);
                }
                Err(error) => {
                    eprintln!("There was an issue querying the balance: {}", error);
                    exit(6);
                }
            }
        }
        let _guard = exporter.wait_duration(duration);
    }

    //let validators_exporter = ValidatorExporter::new();
    //let validators_info = rpc_client.get_vote_accounts_with_config(RpcGetVoteAccountsConfig { vote_pubkey: Some(config.validators[0].validator_vote_account.pubkey.clone()), ..RpcGetVoteAccountsConfig::default() });

    //println!("{:?}", validators_info);
}
