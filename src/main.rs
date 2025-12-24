mod api;
mod cli;
mod config;
mod error;
mod models;
mod output;

use api::PayjpClient;
use clap::{Parser, Subcommand};
use cli::{CardArgs, ChargeArgs, ConfigArgs, CustomerArgs};
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
    /// API key (overrides PAYJP_SECRET_KEY environment variable)
    #[arg(short = 'k', long, global = true)]
    api_key: Option<String>,

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
    let config = Config::build(cli.api_key, cli.output, cli.verbose, cli.profile)?;

    // Require API key for other commands
    let api_key = config.require_api_key()?;

    // Create API client
    let client = PayjpClient::new(api_key)?.with_verbose(config.verbose);

    // Execute command
    match cli.command {
        Commands::Charge(args) => args.run(&client, &config),
        Commands::Customer(args) => args.run(&client, &config),
        Commands::Card(args) => args.run(&client, &config),
        Commands::Config(_) => unreachable!(),
    }
}
