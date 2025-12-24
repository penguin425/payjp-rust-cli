use crate::api::client::PayjpClient;
use crate::error::Result;
use crate::models::{Card, CreateCardParams, DeleteResponse, List, ListParams, UpdateCardParams};

impl PayjpClient {
    /// Create a new card for a customer
    pub fn create_card(&self, customer_id: &str, params: &CreateCardParams) -> Result<Card> {
        self.post(
            &format!("/customers/{}/cards", customer_id),
            &params.to_form_params(),
        )
    }

    /// Get a card by ID
    pub fn get_card(&self, customer_id: &str, card_id: &str) -> Result<Card> {
        self.get(&format!("/customers/{}/cards/{}", customer_id, card_id))
    }

    /// Update a card
    pub fn update_card(
        &self,
        customer_id: &str,
        card_id: &str,
        params: &UpdateCardParams,
    ) -> Result<Card> {
        self.post(
            &format!("/customers/{}/cards/{}", customer_id, card_id),
            &params.to_form_params(),
        )
    }

    /// Delete a card
    pub fn delete_card(&self, customer_id: &str, card_id: &str) -> Result<DeleteResponse> {
        self.delete(&format!("/customers/{}/cards/{}", customer_id, card_id))
    }

    /// List cards for a customer
    pub fn list_cards(&self, customer_id: &str, params: &ListParams) -> Result<List<Card>> {
        self.get_with_params(
            &format!("/customers/{}/cards", customer_id),
            &params.to_query_params(),
        )
    }
}
