use crate::config::{Config, ProfileConfig};
use crate::error::Result;
use clap::{Args, Subcommand};
use colored::Colorize;

#[derive(Debug, Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommand,
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommand {
    /// Set configuration values
    Set {
        /// Secret API key (sk_xxx)
        #[arg(short = 'k', long)]
        api_key: Option<String>,

        /// Public API key (pk_xxx)
        #[arg(long)]
        public_key: Option<String>,

        /// Output format (json or table)
        #[arg(short, long)]
        output: Option<String>,

        /// Profile name (default: default)
        #[arg(short, long, default_value = "default")]
        profile: String,
    },

    /// Show current configuration
    Show {
        /// Profile name (default: default)
        #[arg(short, long, default_value = "default")]
        profile: String,
    },

    /// List all profiles
    List,

    /// Show config file path
    Path,
}

fn mask_key(key: &str) -> String {
    if key.len() > 8 {
        format!("{}...{}", &key[..4], &key[key.len() - 4..])
    } else {
        "****".to_string()
    }
}

impl ConfigArgs {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            ConfigCommand::Set {
                api_key,
                public_key,
                output,
                profile,
            } => {
                let mut config = Config::load_from_file()?;

                let profile_config = if profile == "default" {
                    &mut config.default
                } else {
                    config
                        .profiles
                        .entry(profile.clone())
                        .or_insert_with(ProfileConfig::default)
                };

                if let Some(key) = api_key {
                    profile_config.api_key = Some(key.clone());
                    println!("{} Secret key set to: {}", "✓".green(), mask_key(key));
                }

                if let Some(key) = public_key {
                    profile_config.public_key = Some(key.clone());
                    println!("{} Public key set to: {}", "✓".green(), mask_key(key));
                }

                if let Some(out) = output {
                    if out != "json" && out != "table" {
                        println!("{} Output format must be 'json' or 'table'", "✗".red());
                        return Ok(());
                    }
                    profile_config.output = Some(out.clone());
                    println!("{} Output format set to: {}", "✓".green(), out);
                }

                Config::save_to_file(&config)?;
                println!(
                    "\n{} Configuration saved to profile: {}",
                    "✓".green(),
                    profile
                );
            }

            ConfigCommand::Show { profile } => {
                let config = Config::load_from_file()?;
                let default_profile = ProfileConfig::default();

                let profile_config = if profile == "default" {
                    &config.default
                } else {
                    config.profiles.get(profile).unwrap_or(&default_profile)
                };

                println!("{}", format!("Profile: {}", profile).bold());
                println!("{}", "─".repeat(40));

                if let Some(ref key) = profile_config.api_key {
                    println!("  {:15} {}", "Secret Key:".dimmed(), mask_key(key));
                } else {
                    println!("  {:15} {}", "Secret Key:".dimmed(), "(not set)".yellow());
                }

                if let Some(ref key) = profile_config.public_key {
                    println!("  {:15} {}", "Public Key:".dimmed(), mask_key(key));
                } else {
                    println!("  {:15} {}", "Public Key:".dimmed(), "(not set)".yellow());
                }

                println!(
                    "  {:15} {}",
                    "Output:".dimmed(),
                    profile_config.output.as_deref().unwrap_or("table")
                );

                // Check environment variables
                println!();
                println!("{}", "Environment Variables:".bold());
                println!("{}", "─".repeat(40));

                if let Ok(env_key) = std::env::var("PAYJP_SECRET_KEY") {
                    println!("  {:20} {}", "PAYJP_SECRET_KEY:".dimmed(), mask_key(&env_key));
                } else {
                    println!(
                        "  {:20} {}",
                        "PAYJP_SECRET_KEY:".dimmed(),
                        "(not set)".dimmed()
                    );
                }

                if let Ok(env_key) = std::env::var("PAYJP_PUBLIC_KEY") {
                    println!("  {:20} {}", "PAYJP_PUBLIC_KEY:".dimmed(), mask_key(&env_key));
                } else {
                    println!(
                        "  {:20} {}",
                        "PAYJP_PUBLIC_KEY:".dimmed(),
                        "(not set)".dimmed()
                    );
                }

                if let Ok(env_output) = std::env::var("PAYJP_OUTPUT") {
                    println!("  {:20} {}", "PAYJP_OUTPUT:".dimmed(), env_output);
                } else {
                    println!("  {:20} {}", "PAYJP_OUTPUT:".dimmed(), "(not set)".dimmed());
                }
            }

            ConfigCommand::List => {
                let config = Config::load_from_file()?;

                println!("{}", "Profiles:".bold());
                println!("{}", "─".repeat(40));

                // Default profile
                let has_default =
                    config.default.api_key.is_some() || config.default.public_key.is_some();
                println!(
                    "  {} {}",
                    "default".cyan(),
                    if has_default { "" } else { "(empty)" }
                );

                // Other profiles
                for (name, profile) in &config.profiles {
                    let has_key = profile.api_key.is_some() || profile.public_key.is_some();
                    println!(
                        "  {} {}",
                        name.cyan(),
                        if has_key { "" } else { "(empty)" }
                    );
                }
            }

            ConfigCommand::Path => {
                if let Some(path) = Config::config_file_path() {
                    println!("{}", path.display());
                } else {
                    println!("Could not determine config file path");
                }
            }
        }

        Ok(())
    }
}
