use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::card::Card;

/// Charge object from PAY.JP API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Charge {
    pub id: String,
    pub object: String,
    pub livemode: bool,
    pub created: i64,
    pub amount: u64,
    pub currency: String,
    pub paid: bool,
    pub captured: bool,
    #[serde(default)]
    pub captured_at: Option<i64>,
    pub refunded: bool,
    #[serde(default)]
    pub amount_refunded: u64,
    #[serde(default)]
    pub refund_reason: Option<String>,
    #[serde(default)]
    pub card: Option<Card>,
    #[serde(default)]
    pub customer: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub failure_code: Option<String>,
    #[serde(default)]
    pub failure_message: Option<String>,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    #[serde(default)]
    pub three_d_secure_status: Option<String>,
    #[serde(default)]
    pub expired_at: Option<i64>,
}

/// Parameters for creating a charge
#[derive(Debug, Clone)]
pub struct CreateChargeParams {
    pub amount: u64,
    pub currency: String,
    pub card: Option<String>,
    pub customer: Option<String>,
    pub capture: Option<bool>,
    pub description: Option<String>,
    pub expiry_days: Option<u32>,
    pub three_d_secure: Option<bool>,
    pub metadata: HashMap<String, String>,
}

impl Default for CreateChargeParams {
    fn default() -> Self {
        Self {
            amount: 0,
            currency: "jpy".to_string(),
            card: None,
            customer: None,
            capture: None,
            description: None,
            expiry_days: None,
            three_d_secure: None,
            metadata: HashMap::new(),
        }
    }
}

impl CreateChargeParams {
    pub fn to_form_params(&self) -> Vec<(String, String)> {
        let mut params = vec![
            ("amount".to_string(), self.amount.to_string()),
            ("currency".to_string(), self.currency.clone()),
        ];

        if let Some(ref card) = self.card {
            params.push(("card".to_string(), card.clone()));
        }
        if let Some(ref customer) = self.customer {
            params.push(("customer".to_string(), customer.clone()));
        }
        if let Some(capture) = self.capture {
            params.push(("capture".to_string(), capture.to_string()));
        }
        if let Some(ref description) = self.description {
            params.push(("description".to_string(), description.clone()));
        }
        if let Some(expiry_days) = self.expiry_days {
            params.push(("expiry_days".to_string(), expiry_days.to_string()));
        }
        if let Some(three_d_secure) = self.three_d_secure {
            params.push(("three_d_secure".to_string(), three_d_secure.to_string()));
        }

        for (key, value) in &self.metadata {
            params.push((format!("metadata[{}]", key), value.clone()));
        }

        params
    }
}

/// Parameters for updating a charge
#[derive(Debug, Clone, Default)]
pub struct UpdateChargeParams {
    pub description: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl UpdateChargeParams {
    pub fn to_form_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(ref description) = self.description {
            params.push(("description".to_string(), description.clone()));
        }

        for (key, value) in &self.metadata {
            params.push((format!("metadata[{}]", key), value.clone()));
        }

        params
    }
}

/// Parameters for refunding a charge
#[derive(Debug, Clone, Default)]
pub struct RefundChargeParams {
    pub amount: Option<u64>,
    pub refund_reason: Option<String>,
}

impl RefundChargeParams {
    pub fn to_form_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(amount) = self.amount {
            params.push(("amount".to_string(), amount.to_string()));
        }
        if let Some(ref refund_reason) = self.refund_reason {
            params.push(("refund_reason".to_string(), refund_reason.clone()));
        }

        params
    }
}

/// Parameters for capturing a charge
#[derive(Debug, Clone, Default)]
pub struct CaptureChargeParams {
    pub amount: Option<u64>,
}

impl CaptureChargeParams {
    pub fn to_form_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(amount) = self.amount {
            params.push(("amount".to_string(), amount.to_string()));
        }

        params
    }
}
