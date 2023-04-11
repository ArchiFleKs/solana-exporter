use anyhow::Result;
use prometheus_exporter::{
    self,
    prometheus::register_gauge,
    prometheus::register_gauge_vec,
    prometheus::register_int_gauge,
    prometheus::register_int_gauge_vec,
    prometheus::Opts,
    prometheus::{Gauge, GaugeVec, IntGauge, IntGaugeVec},
};

pub struct AccountExporter {
    pub solana_account_balance: GaugeVec,
}

impl AccountExporter {
    pub fn new() -> Result<Self> {
        let solana_account_balance = register_gauge_vec!(
            Opts::new("solana_account_balance", "Balance of Solana accounts"),
            &["description", "pubkey"]
        )?;

        Ok(Self {
            solana_account_balance,
        })
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
    solana_validator_activated_stake: GaugeVec,
}

impl ValidatorExporter {
    pub fn new() -> Result<Self> {
        static LABELS: &[&str] = &[
            "name",
            "description",
            "identity_pubkey",
            "vote_account_pubkey",
        ];

        let solana_validator_version = register_gauge_vec!(
            Opts::new("solana_validator_version", "Version of solana on validator"),
            LABELS
        )?;

        let solana_validator_vote_account_balance = register_gauge_vec!(
            Opts::new(
                "solana_vote_account_balance",
                "Balance on validator vote account"
            ),
            LABELS
        )?;

        let solana_validator_credits_current = register_gauge_vec!(
            Opts::new(
                "solana_validator_credits_current",
                "Current credits on validator"
            ),
            LABELS
        )?;

        let solana_validator_identity_balance = register_gauge_vec!(
            Opts::new(
                "solana_validator_identity_balance",
                "Current credits on validator identity account"
            ),
            LABELS
        )?;

        let solana_transaction_per_seconds = register_gauge!(Opts::new(
            "solana_transaction_per_seconds",
            "Cluster transaction per second"
        ))?;

        let solana_validator_status = register_int_gauge_vec!(
            Opts::new("solana_validator_status", "Validator status"),
            LABELS
        )?;

        let solana_validator_root_slot = register_int_gauge_vec!(
            Opts::new("solana_validator_root_slot", "Validator root slot"),
            LABELS
        )?;

        let solana_validator_vote_percent = register_gauge_vec!(
            Opts::new(
                "solana_validator_vote_percent",
                "Validator correct votes percent"
            ),
            LABELS
        )?;

        let solana_validator_epoch_elapsed_percent = register_gauge_vec!(
            Opts::new(
                "solana_validator_epoch_elapsed_percent",
                "Validator epoch elapsed percent"
            ),
            LABELS
        )?;

        let solana_validator_open_files = register_int_gauge_vec!(
            Opts::new("solana_validator_open_files", "Validator open files"),
            LABELS
        )?;

        let solana_validators_count = register_int_gauge!(Opts::new(
            "solana_validator_count",
            "Count of solana validator on the network"
        ))?;

        let solana_validator_last_vote = register_int_gauge_vec!(
            Opts::new("solana_validator_last_vote", "Validator last vote"),
            LABELS
        )?;

        let solana_validator_epoch = register_int_gauge_vec!(
            Opts::new("solana_validator_epoch", "Current epoch on validator"),
            LABELS
        )?;

        let solana_validator_credits = register_int_gauge_vec!(
            Opts::new("solana_validator_credits", "Validator vote credits"),
            LABELS
        )?;

        let solana_validator_commission_percent = register_gauge_vec!(
            Opts::new(
                "solana_validator_commission_percent",
                "Validator commission rate"
            ),
            LABELS
        )?;

        let solana_validator_activated_stake = register_gauge_vec!(
            Opts::new(
                "solana_validator_activated_stake",
                "Validator activated stake"
            ),
            LABELS
        )?;

        Ok(Self {
            solana_validator_version,
            solana_validator_vote_account_balance,
            solana_validator_credits_current,
            solana_validator_identity_balance,
            solana_transaction_per_seconds,
            solana_validator_status,
            solana_validator_root_slot,
            solana_validator_vote_percent,
            solana_validator_epoch_elapsed_percent,
            solana_validator_open_files,
            solana_validators_count,
            solana_validator_last_vote,
            solana_validator_epoch,
            solana_validator_credits,
            solana_validator_commission_percent,
            solana_validator_activated_stake,
        })
    }
}
