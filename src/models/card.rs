use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Card object from PAY.JP API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub livemode: bool,
    #[serde(default)]
    pub name: Option<String>,
    pub last4: String,
    pub exp_month: u8,
    pub exp_year: u16,
    pub brand: String,
    pub fingerprint: String,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub address_city: Option<String>,
    #[serde(default)]
    pub address_line1: Option<String>,
    #[serde(default)]
    pub address_line2: Option<String>,
    #[serde(default)]
    pub address_state: Option<String>,
    #[serde(default)]
    pub address_zip: Option<String>,
    #[serde(default)]
    pub address_zip_check: Option<String>,
    #[serde(default)]
    pub cvc_check: Option<String>,
    #[serde(default)]
    pub customer: Option<String>,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    #[serde(default)]
    pub three_d_secure_status: Option<String>,
}

/// Card list (embedded in Customer)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardList {
    pub object: String,
    pub count: u64,
    pub has_more: bool,
    pub url: String,
    pub data: Vec<Card>,
}

impl Default for CardList {
    fn default() -> Self {
        Self {
            object: "list".to_string(),
            count: 0,
            has_more: false,
            url: String::new(),
            data: Vec::new(),
        }
    }
}

/// Parameters for creating a card
#[derive(Debug, Clone, Default)]
pub struct CreateCardParams {
    pub card: String,
    pub metadata: HashMap<String, String>,
}

impl CreateCardParams {
    pub fn to_form_params(&self) -> Vec<(String, String)> {
        let mut params = vec![("card".to_string(), self.card.clone())];

        for (key, value) in &self.metadata {
            params.push((format!("metadata[{}]", key), value.clone()));
        }

        params
    }
}

/// Parameters for updating a card
#[derive(Debug, Clone, Default)]
pub struct UpdateCardParams {
    pub name: Option<String>,
    pub address_city: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub address_state: Option<String>,
    pub address_zip: Option<String>,
    pub country: Option<String>,
    pub metadata: HashMap<String, String>,
}

impl UpdateCardParams {
    pub fn to_form_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(ref name) = self.name {
            params.push(("name".to_string(), name.clone()));
        }
        if let Some(ref address_city) = self.address_city {
            params.push(("address_city".to_string(), address_city.clone()));
        }
        if let Some(ref address_line1) = self.address_line1 {
            params.push(("address_line1".to_string(), address_line1.clone()));
        }
        if let Some(ref address_line2) = self.address_line2 {
            params.push(("address_line2".to_string(), address_line2.clone()));
        }
        if let Some(ref address_state) = self.address_state {
            params.push(("address_state".to_string(), address_state.clone()));
        }
        if let Some(ref address_zip) = self.address_zip {
            params.push(("address_zip".to_string(), address_zip.clone()));
        }
        if let Some(ref country) = self.country {
            params.push(("country".to_string(), country.clone()));
        }

        for (key, value) in &self.metadata {
            params.push((format!("metadata[{}]", key), value.clone()));
        }

        params
    }
}
