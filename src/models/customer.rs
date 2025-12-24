use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::card::CardList;

/// Customer object from PAY.JP API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: String,
    pub object: String,
    pub livemode: bool,
    pub created: i64,
    #[serde(default)]
    pub default_card: Option<String>,
    #[serde(default)]
    pub cards: CardList,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// Parameters for creating a customer
#[derive(Debug, Clone, Default)]
pub struct CreateCustomerParams {
    pub email: Option<String>,
    pub description: Option<String>,
    pub card: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl CreateCustomerParams {
    pub fn to_form_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(ref email) = self.email {
            params.push(("email".to_string(), email.clone()));
        }
        if let Some(ref description) = self.description {
            params.push(("description".to_string(), description.clone()));
        }
        if let Some(ref card) = self.card {
            params.push(("card".to_string(), card.clone()));
        }

        for (key, value) in &self.metadata {
            params.push((format!("metadata[{}]", key), value.clone()));
        }

        params
    }
}

/// Parameters for updating a customer
#[derive(Debug, Clone, Default)]
pub struct UpdateCustomerParams {
    pub email: Option<String>,
    pub description: Option<String>,
    pub default_card: Option<String>,
    pub card: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl UpdateCustomerParams {
    pub fn to_form_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(ref email) = self.email {
            params.push(("email".to_string(), email.clone()));
        }
        if let Some(ref description) = self.description {
            params.push(("description".to_string(), description.clone()));
        }
        if let Some(ref default_card) = self.default_card {
            params.push(("default_card".to_string(), default_card.clone()));
        }
        if let Some(ref card) = self.card {
            params.push(("card".to_string(), card.clone()));
        }

        for (key, value) in &self.metadata {
            params.push((format!("metadata[{}]", key), value.clone()));
        }

        params
    }
}
