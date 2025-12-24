use crate::api::client::PayjpClient;
use crate::error::Result;
use crate::models::{
    CreateCustomerParams, Customer, DeleteResponse, List, ListParams, UpdateCustomerParams,
};

impl PayjpClient {
    /// Create a new customer
    pub fn create_customer(&self, params: &CreateCustomerParams) -> Result<Customer> {
        self.post("/customers", &params.to_form_params())
    }

    /// Get a customer by ID
    pub fn get_customer(&self, id: &str) -> Result<Customer> {
        self.get(&format!("/customers/{}", id))
    }

    /// Update a customer
    pub fn update_customer(&self, id: &str, params: &UpdateCustomerParams) -> Result<Customer> {
        self.post(&format!("/customers/{}", id), &params.to_form_params())
    }

    /// Delete a customer
    pub fn delete_customer(&self, id: &str) -> Result<DeleteResponse> {
        self.delete(&format!("/customers/{}", id))
    }

    /// List customers
    pub fn list_customers(&self, params: &ListParams) -> Result<List<Customer>> {
        self.get_with_params("/customers", &params.to_query_params())
    }
}
