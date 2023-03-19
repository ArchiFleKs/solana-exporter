use serde::Deserialize;
use figment::{Figment, providers::{Format, Yaml}};
use crate::validator::Validator;
use crate::account::Account;
use crate::cli::Cli;

#[derive(Deserialize)]
pub struct Config {
    pub rpc_url: String,
    pub accounts: Vec<Account>,
    pub validators: Vec<Validator>,
    http: HttpConfig
}

#[derive(Deserialize)]
struct HttpConfig {
    listen_address: String,
    listen_port: u16,
}

impl Config {
    pub fn new(args: &Cli) -> Self {
        Figment::new()
            .merge(Yaml::file(args.config.as_deref().expect("Could not load configuration from parser")))
            .extract().expect("Error loading config file")
    }
    pub fn binding(&self) -> String {
        format!("{}:{}", self.http.listen_address, self.http.listen_port)
    }
}
