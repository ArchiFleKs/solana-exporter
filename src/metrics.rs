use prometheus_exporter::{
    self,
    prometheus::register_gauge_vec,
    prometheus::register_int_gauge_vec,
    prometheus::register_gauge,
    prometheus::register_int_gauge,
    prometheus::Opts,
    prometheus::{GaugeVec, Gauge, IntGaugeVec, IntGauge}
};

pub struct AccountExporter {
    solana_account_balance: GaugeVec
}

impl AccountExporter {
    pub fn new() -> Self {
        Self {
            solana_account_balance: register_gauge_vec!(
                Opts::new("solana_wallet_balance", "Balance of Solana wallets"),
                &["description","pubkey"]).unwrap()
        }
    }
}

pub struct ValidatorExporter {
    solana_validator_version: GaugeVec,
    solana_validator_vote_account_balance: GaugeVec,
    solana_validator_credits_current: GaugeVec,
    solana_validator_identity_balance: GaugeVec,
    solana_transaction_per_seconds: Gauge,
    solana_validator_status: IntGaugeVec,
    solana_validator_root_slot: IntGaugeVec,
    solana_validator_vote_percent: GaugeVec,
    solana_validator_epoch_elapsed_percent: GaugeVec,
    solana_validator_open_files: IntGaugeVec,
    solana_validators_count: IntGauge,
    solana_validator_last_vote: IntGaugeVec,
    solana_validator_epoch: IntGaugeVec,
    solana_validator_credits: IntGaugeVec,
    solana_validator_commission_percent: GaugeVec,
    solana_validator_activated_stake: GaugeVec
}

impl ValidatorExporter {
    pub fn new() -> Self {
        Self {
            solana_validator_version: register_gauge_vec!(
                Opts::new("solana_validator_version", "Version of solana on validator"),
                &["name","description","identity_pubkey","vote_account_pubkey"]).unwrap(),
            solana_validator_vote_account_balance: register_gauge_vec!(
                Opts::new("solana_vote_account_balance", "Balance on validator vote account"),
                &["name","description","identity_pubkey","vote_account_pubkey"]).unwrap(),
            solana_validator_credits_current: register_gauge_vec!(
                Opts::new("solana_validator_credits_current", "Current credits on validator vote account"),
                &["name","description","identity_pubkey","vote_account_pubkey"]).unwrap(),
            solana_validator_identity_balance: register_gauge_vec!(
                Opts::new("solana_validator_identity_balance", "Current credits on validator identity account"),
                &["name","description","identity_pubkey","vote_account_pubkey"]).unwrap(),
            solana_transaction_per_seconds: register_gauge!(
                Opts::new("solana_transaction_per_seconds", "Cluster transaction per second")).unwrap(),
            solana_validator_status: register_int_gauge_vec!(
                Opts::new("solana_validator_status", "Validator status"),
                &["name","description","identity_pubkey","vote_account_pubkey"]).unwrap(),
            solana_validator_root_slot: register_int_gauge_vec!(
                Opts::new("solana_validator_root_slot", "Validator root slot"),
                &["name","description","identity_pubkey","vote_account_pubkey"]).unwrap(),
            solana_validator_vote_percent: register_gauge_vec!(
                Opts::new("solana_validator_vote_percent", "Validator correct votes percent"),
                &["name","description","identity_pubkey","vote_account_pubkey"]).unwrap(),
            solana_validator_epoch_elapsed_percent: register_gauge_vec!(
                Opts::new("solana_validator_epoch_elapsed_percent", "Validator epoch elapsed percent"),
                &["name","description","identity_pubkey","vote_account_pubkey"]).unwrap(),
            solana_validator_open_files: register_int_gauge_vec!(
                Opts::new("solana_validator_open_files", "Validator open files"),
                &["name","description","identity_pubkey","vote_account_pubkey"]).unwrap(),
            solana_validators_count: register_int_gauge!(
                Opts::new("solana_validator_count", "Count of solana validator on the network")).unwrap(),
            solana_validator_last_vote: register_int_gauge_vec!(
                Opts::new("solana_validator_last_vote", "Validator last vote"),
                &["name","description","identity_pubkey","vote_account_pubkey"]).unwrap(),
            solana_validator_epoch: register_int_gauge_vec!(
                Opts::new("solana_validator_epoch", "Current epoch on validator"),
                &["name","description","identity_pubkey","vote_account_pubkey"]).unwrap(),
            solana_validator_credits: register_int_gauge_vec!(
                Opts::new("solana_validator_credits", "Validator vote credits"),
                &["name","description","identity_pubkey","vote_account_pubkey"]).unwrap(),
            solana_validator_commission_percent: register_gauge_vec!(
                Opts::new("solana_validator_commission_percent", "Validator commission rate"),
                &["name","description","identity_pubkey","vote_account_pubkey"]).unwrap(),
            solana_validator_activated_stake: register_gauge_vec!(
                Opts::new("solana_validator_activated_stake", "Validator activated stake"),
                &["name","description","identity_pubkey","vote_account_pubkey"]).unwrap(),
        }
    }
}

