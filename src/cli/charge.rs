use crate::api::PayjpClient;
use crate::config::Config;
use crate::error::Result;
use crate::models::{
    CaptureChargeParams, CreateChargeParams, ListParams, RefundChargeParams, UpdateChargeParams,
};
use crate::output::Output;
use clap::{Args, Subcommand};
use std::collections::HashMap;

#[derive(Debug, Args)]
pub struct ChargeArgs {
    #[command(subcommand)]
    pub command: ChargeCommand,
}

#[derive(Debug, Subcommand)]
pub enum ChargeCommand {
    /// Create a new charge
    Create {
        /// Amount (50-9,999,999)
        #[arg(short, long)]
        amount: u64,

        /// Card token (tok_xxx)
        #[arg(short = 'c', long)]
        card: Option<String>,

        /// Customer ID (cus_xxx)
        #[arg(long)]
        customer: Option<String>,

        /// Whether to capture immediately (default: true)
        #[arg(long)]
        capture: Option<bool>,

        /// Currency (default: jpy)
        #[arg(long, default_value = "jpy")]
        currency: String,

        /// Description
        #[arg(short, long)]
        description: Option<String>,

        /// Enable 3D Secure
        #[arg(long)]
        three_d_secure: Option<bool>,

        /// Expiry days for uncaptured charge
        #[arg(long)]
        expiry_days: Option<u32>,

        /// Metadata (key=value format, can be specified multiple times)
        #[arg(short, long, value_parser = parse_metadata)]
        metadata: Vec<(String, String)>,
    },

    /// Get a charge by ID
    Get {
        /// Charge ID
        id: String,
    },

    /// Update a charge
    Update {
        /// Charge ID
        id: String,

        /// Description
        #[arg(short, long)]
        description: Option<String>,

        /// Metadata (key=value format, can be specified multiple times)
        #[arg(short, long, value_parser = parse_metadata)]
        metadata: Vec<(String, String)>,
    },

    /// Refund a charge
    Refund {
        /// Charge ID
        id: String,

        /// Amount to refund (partial refund)
        #[arg(short, long)]
        amount: Option<u64>,

        /// Refund reason
        #[arg(short, long)]
        reason: Option<String>,
    },

    /// Capture an authorized charge
    Capture {
        /// Charge ID
        id: String,

        /// Amount to capture (partial capture)
        #[arg(short, long)]
        amount: Option<u64>,
    },

    /// Reauthorize a charge
    Reauth {
        /// Charge ID
        id: String,
    },

    /// Complete 3D Secure verification
    TdsFinish {
        /// Charge ID
        id: String,
    },

    /// List charges
    List {
        /// Maximum number of charges to return (1-100)
        #[arg(short, long)]
        limit: Option<u8>,

        /// Number of charges to skip
        #[arg(long)]
        offset: Option<u64>,

        /// Return charges created after this date (YYYY-MM-DD or Unix timestamp)
        #[arg(long, value_parser = parse_timestamp)]
        since: Option<i64>,

        /// Return charges created before this date (YYYY-MM-DD or Unix timestamp)
        #[arg(long, value_parser = parse_timestamp)]
        until: Option<i64>,
    },
}

fn parse_metadata(s: &str) -> std::result::Result<(String, String), String> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err("Metadata must be in key=value format".to_string());
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

fn parse_timestamp(s: &str) -> std::result::Result<i64, String> {
    // Try parsing as Unix timestamp first
    if let Ok(ts) = s.parse::<i64>() {
        return Ok(ts);
    }

    // Try parsing as date (YYYY-MM-DD)
    if let Ok(date) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        let datetime = date.and_hms_opt(0, 0, 0).unwrap();
        return Ok(datetime.and_utc().timestamp());
    }

    Err("Invalid date format. Use YYYY-MM-DD or Unix timestamp".to_string())
}

impl ChargeArgs {
    pub fn run(&self, client: &PayjpClient, config: &Config) -> Result<()> {
        let output = Output::new(config.output);

        match &self.command {
            ChargeCommand::Create {
                amount,
                card,
                customer,
                capture,
                currency,
                description,
                three_d_secure,
                expiry_days,
                metadata,
            } => {
                let params = CreateChargeParams {
                    amount: *amount,
                    currency: currency.clone(),
                    card: card.clone(),
                    customer: customer.clone(),
                    capture: *capture,
                    description: description.clone(),
                    three_d_secure: *three_d_secure,
                    expiry_days: *expiry_days,
                    metadata: metadata.iter().cloned().collect::<HashMap<_, _>>(),
                };

                let charge = client.create_charge(&params)?;
                output.print(&charge);
            }

            ChargeCommand::Get { id } => {
                let charge = client.get_charge(id)?;
                output.print(&charge);
            }

            ChargeCommand::Update {
                id,
                description,
                metadata,
            } => {
                let params = UpdateChargeParams {
                    description: description.clone(),
                    metadata: metadata.iter().cloned().collect::<HashMap<_, _>>(),
                };

                let charge = client.update_charge(id, &params)?;
                output.print(&charge);
            }

            ChargeCommand::Refund { id, amount, reason } => {
                let params = RefundChargeParams {
                    amount: *amount,
                    refund_reason: reason.clone(),
                };

                let charge = client.refund_charge(id, &params)?;
                output.print(&charge);
            }

            ChargeCommand::Capture { id, amount } => {
                let params = CaptureChargeParams { amount: *amount };

                let charge = client.capture_charge(id, &params)?;
                output.print(&charge);
            }

            ChargeCommand::Reauth { id } => {
                let charge = client.reauth_charge(id)?;
                output.print(&charge);
            }

            ChargeCommand::TdsFinish { id } => {
                let charge = client.tds_finish_charge(id)?;
                output.print(&charge);
            }

            ChargeCommand::List {
                limit,
                offset,
                since,
                until,
            } => {
                let params = ListParams {
                    limit: *limit,
                    offset: *offset,
                    since: *since,
                    until: *until,
                };

                let charges = client.list_charges(&params)?;
                output.print_list(&charges);
            }
        }

        Ok(())
    }
}
