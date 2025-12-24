use serde::{Deserialize, Serialize};
use std::fmt;

/// PAY.JP API error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayjpApiError {
    pub error: PayjpErrorDetail,
}

/// PAY.JP error detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayjpErrorDetail {
    pub status: u16,
    #[serde(rename = "type")]
    pub error_type: String,
    #[serde(default)]
    pub code: Option<String>,
    pub message: String,
    #[serde(default)]
    pub param: Option<String>,
}

impl fmt::Display for PayjpErrorDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl PayjpErrorDetail {
    /// Get a human-readable hint message for the error
    pub fn get_hint(&self) -> Option<String> {
        match self.code.as_deref() {
            Some("invalid_number") => {
                Some("カード番号が正しいかご確認ください。".to_string())
            }
            Some("invalid_cvc") => {
                Some("セキュリティコード（CVC）が正しいかご確認ください。".to_string())
            }
            Some("invalid_expiry_month") => {
                Some("有効期限の月（1-12）が正しいかご確認ください。".to_string())
            }
            Some("invalid_expiry_year") => {
                Some("有効期限の年が正しいかご確認ください。".to_string())
            }
            Some("expired_card") => {
                Some("カードの有効期限が切れています。別のカードをお試しください。".to_string())
            }
            Some("card_declined") => {
                Some("カード会社にお問い合わせいただくか、別のカードをお試しください。".to_string())
            }
            Some("processing_error") => {
                Some("一時的なエラーです。しばらく待ってから再度お試しください。".to_string())
            }
            Some("invalid_api_key") => {
                Some("APIキーが正しいかご確認ください。".to_string())
            }
            Some("over_capacity") => {
                Some("レート制限に達しました。しばらく待ってから再度お試しください。".to_string())
            }
            Some("three_d_secure_failed") => {
                Some("3Dセキュア認証に失敗しました。再度お試しください。".to_string())
            }
            Some("invalid_id") => {
                Some("指定されたIDが見つかりません。".to_string())
            }
            Some("already_captured") => {
                Some("この支払いは既に確定済みです。".to_string())
            }
            Some("already_refunded") => {
                Some("この支払いは既に返金済みです。".to_string())
            }
            Some("invalid_amount") => {
                Some("金額は50〜9,999,999の範囲で指定してください。".to_string())
            }
            _ => None,
        }
    }

    /// Get the error type description in Japanese
    pub fn get_type_description(&self) -> &str {
        match self.error_type.as_str() {
            "client_error" => "クライアントエラー",
            "card_error" => "カードエラー",
            "server_error" => "サーバーエラー",
            "auth_error" => "認証エラー",
            _ => "不明なエラー",
        }
    }
}
