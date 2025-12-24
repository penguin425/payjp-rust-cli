use crate::api::PayjpClient;
use crate::config::Config;
use crate::error::Result;
use crate::models::{CreateCustomerParams, ListParams, UpdateCustomerParams};
use crate::output::Output;
use clap::{Args, Subcommand};
use std::collections::HashMap;

#[derive(Debug, Args)]
pub struct CustomerArgs {
    #[command(subcommand)]
    pub command: CustomerCommand,
}

#[derive(Debug, Subcommand)]
pub enum CustomerCommand {
    /// Create a new customer
    Create {
        /// Email address
        #[arg(short, long)]
        email: Option<String>,

        /// Description
        #[arg(short, long)]
        description: Option<String>,

        /// Card token to attach (tok_xxx)
        #[arg(short, long)]
        card: Option<String>,

        /// Metadata (key=value format, can be specified multiple times)
        #[arg(short, long, value_parser = parse_metadata)]
        metadata: Vec<(String, String)>,
    },

    /// Get a customer by ID
    Get {
        /// Customer ID
        id: String,
    },

    /// Update a customer
    Update {
        /// Customer ID
        id: String,

        /// Email address
        #[arg(short, long)]
        email: Option<String>,

        /// Description
        #[arg(short, long)]
        description: Option<String>,

        /// Default card ID
        #[arg(long)]
        default_card: Option<String>,

        /// New card token to add (tok_xxx)
        #[arg(short, long)]
        card: Option<String>,

        /// Metadata (key=value format, can be specified multiple times)
        #[arg(short, long, value_parser = parse_metadata)]
        metadata: Vec<(String, String)>,
    },

    /// Delete a customer
    Delete {
        /// Customer ID
        id: String,
    },

    /// List customers
    List {
        /// Maximum number of customers to return (1-100)
        #[arg(short, long)]
        limit: Option<u8>,

        /// Number of customers to skip
        #[arg(long)]
        offset: Option<u64>,

        /// Return customers created after this date (YYYY-MM-DD or Unix timestamp)
        #[arg(long, value_parser = parse_timestamp)]
        since: Option<i64>,

        /// Return customers created before this date (YYYY-MM-DD or Unix timestamp)
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

impl CustomerArgs {
    pub fn run(&self, client: &PayjpClient, config: &Config) -> Result<()> {
        let output = Output::new(config.output);

        match &self.command {
            CustomerCommand::Create {
                email,
                description,
                card,
                metadata,
            } => {
                let params = CreateCustomerParams {
                    email: email.clone(),
                    description: description.clone(),
                    card: card.clone(),
                    metadata: metadata.iter().cloned().collect::<HashMap<_, _>>(),
                };

                let customer = client.create_customer(&params)?;
                output.print(&customer);
            }

            CustomerCommand::Get { id } => {
                let customer = client.get_customer(id)?;
                output.print(&customer);
            }

            CustomerCommand::Update {
                id,
                email,
                description,
                default_card,
                card,
                metadata,
            } => {
                let params = UpdateCustomerParams {
                    email: email.clone(),
                    description: description.clone(),
                    default_card: default_card.clone(),
                    card: card.clone(),
                    metadata: metadata.iter().cloned().collect::<HashMap<_, _>>(),
                };

                let customer = client.update_customer(id, &params)?;
                output.print(&customer);
            }

            CustomerCommand::Delete { id } => {
                let response = client.delete_customer(id)?;
                output.print_delete(&response);
            }

            CustomerCommand::List {
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

                let customers = client.list_customers(&params)?;
                output.print_list(&customers);
            }
        }

        Ok(())
    }
}
