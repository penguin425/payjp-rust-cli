use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = "config.toml";
const CONFIG_DIR_NAME: &str = "payjp";
const ENV_API_KEY: &str = "PAYJP_SECRET_KEY";
const ENV_PUBLIC_KEY: &str = "PAYJP_PUBLIC_KEY";
const ENV_OUTPUT: &str = "PAYJP_OUTPUT";

/// Output format
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum OutputFormat {
    #[default]
    Table,
    Json,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "table" => Ok(OutputFormat::Table),
            "json" => Ok(OutputFormat::Json),
            _ => Err(format!("Unknown output format: {}", s)),
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Table => write!(f, "table"),
            OutputFormat::Json => write!(f, "json"),
        }
    }
}

/// Configuration file structure
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigFile {
    #[serde(default)]
    pub default: ProfileConfig,
    #[serde(flatten)]
    pub profiles: HashMap<String, ProfileConfig>,
}

/// Profile configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileConfig {
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub public_key: Option<String>,
    #[serde(default)]
    pub output: Option<String>,
}

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: Option<String>,
    pub public_key: Option<String>,
    pub output: OutputFormat,
    pub verbose: bool,
    pub profile: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: None,
            public_key: None,
            output: OutputFormat::Table,
            verbose: false,
            profile: "default".to_string(),
        }
    }
}

impl Config {
    /// Get config directory path
    pub fn config_dir() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join(CONFIG_DIR_NAME))
    }

    /// Get config file path
    pub fn config_file_path() -> Option<PathBuf> {
        Self::config_dir().map(|p| p.join(CONFIG_FILE_NAME))
    }

    /// Load configuration from file
    pub fn load_from_file() -> Result<ConfigFile> {
        let path = match Self::config_file_path() {
            Some(p) => p,
            None => return Ok(ConfigFile::default()),
        };

        if !path.exists() {
            return Ok(ConfigFile::default());
        }

        let content = fs::read_to_string(&path).map_err(|e| {
            AppError::Config(format!("Failed to read config file: {}", e))
        })?;

        toml::from_str(&content)
            .map_err(|e| AppError::Config(format!("Failed to parse config file: {}", e)))
    }

    /// Save configuration to file
    pub fn save_to_file(config: &ConfigFile) -> Result<()> {
        let dir = match Self::config_dir() {
            Some(d) => d,
            None => return Err(AppError::Config("Cannot determine config directory".to_string())),
        };

        fs::create_dir_all(&dir).map_err(|e| {
            AppError::Config(format!("Failed to create config directory: {}", e))
        })?;

        let path = dir.join(CONFIG_FILE_NAME);
        let content = toml::to_string_pretty(config)
            .map_err(|e| AppError::Config(format!("Failed to serialize config: {}", e)))?;

        fs::write(&path, content).map_err(|e| {
            AppError::Config(format!("Failed to write config file: {}", e))
        })?;

        Ok(())
    }

    /// Build configuration from all sources
    /// Priority: CLI args > Environment variables > Config file
    pub fn build(
        cli_api_key: Option<String>,
        cli_public_key: Option<String>,
        cli_output: Option<String>,
        cli_verbose: bool,
        profile: Option<String>,
    ) -> Result<Self> {
        let profile_name = profile.unwrap_or_else(|| "default".to_string());

        // Load from file
        let file_config = Self::load_from_file()?;
        let profile_config = if profile_name == "default" {
            file_config.default.clone()
        } else {
            file_config.profiles.get(&profile_name).cloned().unwrap_or_default()
        };

        // Environment variables
        let env_api_key = std::env::var(ENV_API_KEY).ok();
        let env_public_key = std::env::var(ENV_PUBLIC_KEY).ok();
        let env_output = std::env::var(ENV_OUTPUT).ok();

        // Resolve API key (CLI > ENV > File)
        let api_key = cli_api_key
            .or(env_api_key)
            .or(profile_config.api_key);

        // Resolve public key (CLI > ENV > File)
        let public_key = cli_public_key
            .or(env_public_key)
            .or(profile_config.public_key);

        // Resolve output format (CLI > ENV > File > Default)
        let output_str = cli_output
            .or(env_output)
            .or(profile_config.output)
            .unwrap_or_else(|| "table".to_string());

        let output = output_str.parse().unwrap_or(OutputFormat::Table);

        Ok(Self {
            api_key,
            public_key,
            output,
            verbose: cli_verbose,
            profile: profile_name,
        })
    }

    /// Get API key or return error
    pub fn require_api_key(&self) -> Result<&str> {
        self.api_key.as_deref().ok_or_else(|| {
            AppError::Config("API key is not set".to_string())
        })
    }

    /// Get public key or return error
    pub fn require_public_key(&self) -> Result<&str> {
        self.public_key.as_deref().ok_or_else(|| {
            AppError::Config("Public key is not set".to_string())
        })
    }
}
