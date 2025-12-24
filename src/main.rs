mod api;
mod cli;
mod config;
mod error;
mod models;
mod output;

use api::PayjpClient;
use clap::{Parser, Subcommand};
use cli::{CardArgs, ChargeArgs, ConfigArgs, CustomerArgs, TokenArgs};
use config::Config;
use std::process;

#[derive(Parser)]
#[command(
    name = "payjp",
    about = "PAY.JP CLI - Command line interface for PAY.JP payment service",
    version,
    author
)]
struct Cli {
    /// Secret API key (overrides PAYJP_SECRET_KEY environment variable)
    #[arg(short = 'k', long, global = true)]
    api_key: Option<String>,

    /// Public API key (overrides PAYJP_PUBLIC_KEY environment variable)
    #[arg(long, global = true)]
    public_key: Option<String>,

    /// Output format [json|table]
    #[arg(short, long, global = true)]
    output: Option<String>,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Profile to use
    #[arg(short, long, global = true)]
    profile: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Charge operations (create, get, list, refund, capture)
    Charge(ChargeArgs),

    /// Customer operations (create, get, list, update, delete)
    Customer(CustomerArgs),

    /// Card operations (create, get, list, update, delete)
    Card(CardArgs),

    /// Token operations (create, get) - uses public key
    Token(TokenArgs),

    /// Configuration management
    Config(ConfigArgs),
}

fn main() {
    let cli = Cli::parse();

    let result = run(cli);

    if let Err(e) = result {
        eprintln!("{}", e.format_error());
        process::exit(1);
    }
}

fn run(cli: Cli) -> error::Result<()> {
    // Handle config command separately (doesn't need API key)
    if let Commands::Config(args) = cli.command {
        return args.run();
    }

    // Build configuration
    let config = Config::build(
        cli.api_key,
        cli.public_key,
        cli.output,
        cli.verbose,
        cli.profile,
    )?;

    // Execute command
    match cli.command {
        Commands::Token(args) => {
            // Token uses public key
            let public_key = config.require_public_key()?;
            args.run(public_key, &config)
        }
        Commands::Charge(args) => {
            let api_key = config.require_api_key()?;
            let client = PayjpClient::new(api_key)?.with_verbose(config.verbose);
            args.run(&client, &config)
        }
        Commands::Customer(args) => {
            let api_key = config.require_api_key()?;
            let client = PayjpClient::new(api_key)?.with_verbose(config.verbose);
            args.run(&client, &config)
        }
        Commands::Card(args) => {
            let api_key = config.require_api_key()?;
            let client = PayjpClient::new(api_key)?.with_verbose(config.verbose);
            args.run(&client, &config)
        }
        Commands::Config(_) => unreachable!(),
    }
}
