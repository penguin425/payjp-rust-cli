use crate::api::PayjpClient;
use crate::config::Config;
use crate::error::Result;
use crate::models::{CreateCardParams, ListParams, UpdateCardParams};
use crate::output::Output;
use clap::{Args, Subcommand};
use std::collections::HashMap;

#[derive(Debug, Args)]
pub struct CardArgs {
    #[command(subcommand)]
    pub command: CardCommand,
}

#[derive(Debug, Subcommand)]
pub enum CardCommand {
    /// Create a new card for a customer
    Create {
        /// Customer ID
        customer_id: String,

        /// Card token (tok_xxx)
        #[arg(short, long)]
        card: String,

        /// Metadata (key=value format, can be specified multiple times)
        #[arg(short, long, value_parser = parse_metadata)]
        metadata: Vec<(String, String)>,
    },

    /// Get a card by ID
    Get {
        /// Customer ID
        customer_id: String,

        /// Card ID
        card_id: String,
    },

    /// Update a card
    Update {
        /// Customer ID
        customer_id: String,

        /// Card ID
        card_id: String,

        /// Cardholder name
        #[arg(short, long)]
        name: Option<String>,

        /// Address city
        #[arg(long)]
        address_city: Option<String>,

        /// Address line 1
        #[arg(long)]
        address_line1: Option<String>,

        /// Address line 2
        #[arg(long)]
        address_line2: Option<String>,

        /// Address state/prefecture
        #[arg(long)]
        address_state: Option<String>,

        /// Address ZIP/postal code
        #[arg(long)]
        address_zip: Option<String>,

        /// Country
        #[arg(long)]
        country: Option<String>,

        /// Metadata (key=value format, can be specified multiple times)
        #[arg(short, long, value_parser = parse_metadata)]
        metadata: Vec<(String, String)>,
    },

    /// Delete a card
    Delete {
        /// Customer ID
        customer_id: String,

        /// Card ID
        card_id: String,
    },

    /// List cards for a customer
    List {
        /// Customer ID
        customer_id: String,

        /// Maximum number of cards to return (1-100)
        #[arg(short, long)]
        limit: Option<u8>,

        /// Number of cards to skip
        #[arg(long)]
        offset: Option<u64>,

        /// Return cards created after this date (YYYY-MM-DD or Unix timestamp)
        #[arg(long, value_parser = parse_timestamp)]
        since: Option<i64>,

        /// Return cards created before this date (YYYY-MM-DD or Unix timestamp)
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

impl CardArgs {
    pub fn run(&self, client: &PayjpClient, config: &Config) -> Result<()> {
        let output = Output::new(config.output);

        match &self.command {
            CardCommand::Create {
                customer_id,
                card,
                metadata,
            } => {
                let params = CreateCardParams {
                    card: card.clone(),
                    metadata: metadata.iter().cloned().collect::<HashMap<_, _>>(),
                };

                let card = client.create_card(customer_id, &params)?;
                output.print(&card);
            }

            CardCommand::Get {
                customer_id,
                card_id,
            } => {
                let card = client.get_card(customer_id, card_id)?;
                output.print(&card);
            }

            CardCommand::Update {
                customer_id,
                card_id,
                name,
                address_city,
                address_line1,
                address_line2,
                address_state,
                address_zip,
                country,
                metadata,
            } => {
                let params = UpdateCardParams {
                    name: name.clone(),
                    address_city: address_city.clone(),
                    address_line1: address_line1.clone(),
                    address_line2: address_line2.clone(),
                    address_state: address_state.clone(),
                    address_zip: address_zip.clone(),
                    country: country.clone(),
                    metadata: metadata.iter().cloned().collect::<HashMap<_, _>>(),
                };

                let card = client.update_card(customer_id, card_id, &params)?;
                output.print(&card);
            }

            CardCommand::Delete {
                customer_id,
                card_id,
            } => {
                let response = client.delete_card(customer_id, card_id)?;
                output.print_delete(&response);
            }

            CardCommand::List {
                customer_id,
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

                let cards = client.list_cards(customer_id, &params)?;
                output.print_list(&cards);
            }
        }

        Ok(())
    }
}
