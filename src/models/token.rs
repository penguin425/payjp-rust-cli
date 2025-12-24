use serde::{Deserialize, Serialize};

use super::card::Card;

/// Token object from PAY.JP API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub id: String,
    pub object: String,
    pub livemode: bool,
    pub created: i64,
    pub used: bool,
    pub card: Card,
}

/// Parameters for creating a token
#[derive(Debug, Clone)]
pub struct CreateTokenParams {
    pub number: String,
    pub exp_month: u8,
    pub exp_year: u16,
    pub cvc: String,
    pub name: Option<String>,
    pub address_city: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub address_state: Option<String>,
    pub address_zip: Option<String>,
    pub country: Option<String>,
}

impl CreateTokenParams {
    pub fn to_form_params(&self) -> Vec<(String, String)> {
        let mut params = vec![
            ("card[number]".to_string(), self.number.clone()),
            ("card[exp_month]".to_string(), self.exp_month.to_string()),
            ("card[exp_year]".to_string(), self.exp_year.to_string()),
            ("card[cvc]".to_string(), self.cvc.clone()),
        ];

        if let Some(ref name) = self.name {
            params.push(("card[name]".to_string(), name.clone()));
        }
        if let Some(ref address_city) = self.address_city {
            params.push(("card[address_city]".to_string(), address_city.clone()));
        }
        if let Some(ref address_line1) = self.address_line1 {
            params.push(("card[address_line1]".to_string(), address_line1.clone()));
        }
        if let Some(ref address_line2) = self.address_line2 {
            params.push(("card[address_line2]".to_string(), address_line2.clone()));
        }
        if let Some(ref address_state) = self.address_state {
            params.push(("card[address_state]".to_string(), address_state.clone()));
        }
        if let Some(ref address_zip) = self.address_zip {
            params.push(("card[address_zip]".to_string(), address_zip.clone()));
        }
        if let Some(ref country) = self.country {
            params.push(("card[country]".to_string(), country.clone()));
        }

        params
    }
}
