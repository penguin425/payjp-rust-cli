use crate::api::client::PayjpClient;
use crate::error::Result;
use crate::models::{
    CaptureChargeParams, Charge, CreateChargeParams, List, ListParams, RefundChargeParams,
    UpdateChargeParams,
};

impl PayjpClient {
    /// Create a new charge
    pub fn create_charge(&self, params: &CreateChargeParams) -> Result<Charge> {
        self.post("/charges", &params.to_form_params())
    }

    /// Get a charge by ID
    pub fn get_charge(&self, id: &str) -> Result<Charge> {
        self.get(&format!("/charges/{}", id))
    }

    /// Update a charge
    pub fn update_charge(&self, id: &str, params: &UpdateChargeParams) -> Result<Charge> {
        self.post(&format!("/charges/{}", id), &params.to_form_params())
    }

    /// Refund a charge
    pub fn refund_charge(&self, id: &str, params: &RefundChargeParams) -> Result<Charge> {
        self.post(&format!("/charges/{}/refund", id), &params.to_form_params())
    }

    /// Capture a charge
    pub fn capture_charge(&self, id: &str, params: &CaptureChargeParams) -> Result<Charge> {
        self.post(&format!("/charges/{}/capture", id), &params.to_form_params())
    }

    /// Reauthorize a charge
    pub fn reauth_charge(&self, id: &str) -> Result<Charge> {
        self.post(&format!("/charges/{}/reauth", id), &[])
    }

    /// Finish 3D Secure for a charge
    pub fn tds_finish_charge(&self, id: &str) -> Result<Charge> {
        self.post(&format!("/charges/{}/tds_finish", id), &[])
    }

    /// List charges
    pub fn list_charges(&self, params: &ListParams) -> Result<List<Charge>> {
        self.get_with_params("/charges", &params.to_query_params())
    }
}
