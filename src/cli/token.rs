use crate::api::TokenClient;
use crate::config::Config;
use crate::error::Result;
use crate::models::CreateTokenParams;
use crate::output::Output;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct TokenArgs {
    #[command(subcommand)]
    pub command: TokenCommand,
}

#[derive(Debug, Subcommand)]
pub enum TokenCommand {
    /// Create a token from card details (uses public key)
    Create {
        /// Card number
        #[arg(short, long)]
        number: String,

        /// Expiration month (1-12)
        #[arg(long)]
        exp_month: u8,

        /// Expiration year (e.g., 2025)
        #[arg(long)]
        exp_year: u16,

        /// CVC/CVV code
        #[arg(long)]
        cvc: String,

        /// Cardholder name
        #[arg(long)]
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
    },

    /// Get a token by ID (uses public key)
    Get {
        /// Token ID
        id: String,
    },
}

impl TokenArgs {
    pub fn run(&self, public_key: &str, config: &Config) -> Result<()> {
        let client = TokenClient::new(public_key)?.with_verbose(config.verbose);
        let output = Output::new(config.output);

        match &self.command {
            TokenCommand::Create {
                number,
                exp_month,
                exp_year,
                cvc,
                name,
                address_city,
                address_line1,
                address_line2,
                address_state,
                address_zip,
                country,
            } => {
                let params = CreateTokenParams {
                    number: number.clone(),
                    exp_month: *exp_month,
                    exp_year: *exp_year,
                    cvc: cvc.clone(),
                    name: name.clone(),
                    address_city: address_city.clone(),
                    address_line1: address_line1.clone(),
                    address_line2: address_line2.clone(),
                    address_state: address_state.clone(),
                    address_zip: address_zip.clone(),
                    country: country.clone(),
                };

                let token = client.create_token(&params)?;
                output.print(&token);
            }

            TokenCommand::Get { id } => {
                let token = client.get_token(id)?;
                output.print(&token);
            }
        }

        Ok(())
    }
}
