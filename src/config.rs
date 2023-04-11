use crate::account::Account;
use crate::cli::Cli;
use crate::validator::Validator;
use anyhow::{anyhow, Result};
use figment::{
    providers::{Format, Yaml},
    Figment,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub rpc_url: String,
    pub accounts: Vec<Account>,
    pub validators: Vec<Validator>,
    http: HttpConfig,
    pub duration_ms: u64,
}

#[derive(Deserialize)]
struct HttpConfig {
    listen_address: String,
    listen_port: u16,
}

impl Config {
    pub fn new(args: &Cli) -> Result<Self> {
        let config: Config = Figment::new().merge(Yaml::file(&args.config)).extract()?;
        Ok(config)
    }
    pub fn binding(&self) -> String {
        format!("{}:{}", self.http.listen_address, self.http.listen_port)
    }
}
