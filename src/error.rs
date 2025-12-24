use crate::models::PayjpErrorDetail;
use colored::Colorize;
use thiserror::Error;

/// Application error types
#[derive(Error, Debug)]
pub enum AppError {
    #[error("API error: {0}")]
    Api(PayjpErrorDetail),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Rate limited")]
    RateLimited,

    #[error("Max retries exceeded")]
    MaxRetriesExceeded,
}

impl AppError {
    /// Format the error for display to the user
    pub fn format_error(&self) -> String {
        match self {
            AppError::Api(detail) => {
                let mut output = format!(
                    "{} {}\n\n",
                    "Error:".red().bold(),
                    detail.get_type_description()
                );

                if let Some(ref code) = detail.code {
                    output.push_str(&format!("  {}    {}\n", "Code:".dimmed(), code));
                }
                output.push_str(&format!("  {} {}\n", "Message:".dimmed(), detail.message));
                if let Some(ref param) = detail.param {
                    output.push_str(&format!("  {}   {}\n", "Param:".dimmed(), param));
                }

                if let Some(hint) = detail.get_hint() {
                    output.push_str(&format!("\n{} {}\n", "Hint:".yellow().bold(), hint));
                }

                output
            }
            AppError::Http(e) => {
                let mut output = format!("{} HTTPエラー\n\n", "Error:".red().bold());
                output.push_str(&format!("  {} {}\n", "Details:".dimmed(), e));

                if e.is_timeout() {
                    output.push_str(&format!(
                        "\n{} リクエストがタイムアウトしました。ネットワーク接続を確認してください。\n",
                        "Hint:".yellow().bold()
                    ));
                } else if e.is_connect() {
                    output.push_str(&format!(
                        "\n{} サーバーに接続できません。ネットワーク接続を確認してください。\n",
                        "Hint:".yellow().bold()
                    ));
                }

                output
            }
            AppError::Config(msg) => {
                format!(
                    "{} 設定エラー\n\n  {} {}\n\n{} APIキーを設定してください:\n  payjp config set --api-key YOUR_SECRET_KEY\n  または環境変数 PAYJP_SECRET_KEY を設定してください。\n",
                    "Error:".red().bold(),
                    "Details:".dimmed(),
                    msg,
                    "Hint:".yellow().bold()
                )
            }
            AppError::InvalidArgument(msg) => {
                format!(
                    "{} 引数エラー\n\n  {} {}\n",
                    "Error:".red().bold(),
                    "Details:".dimmed(),
                    msg
                )
            }
            AppError::RateLimited => {
                format!(
                    "{} レート制限\n\n  {} APIリクエストの制限に達しました。\n\n{} しばらく待ってから再度お試しください。\n",
                    "Error:".red().bold(),
                    "Details:".dimmed(),
                    "Hint:".yellow().bold()
                )
            }
            AppError::MaxRetriesExceeded => {
                format!(
                    "{} リトライ上限\n\n  {} 最大リトライ回数を超えました。\n\n{} しばらく待ってから再度お試しください。\n",
                    "Error:".red().bold(),
                    "Details:".dimmed(),
                    "Hint:".yellow().bold()
                )
            }
            _ => format!("{} {}\n", "Error:".red().bold(), self),
        }
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
