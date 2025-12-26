use serde::{Deserialize, Serialize};

/// Generic list response from PAY.JP API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List<T> {
    pub object: String,
    pub count: u64,
    pub has_more: bool,
    pub url: String,
    pub data: Vec<T>,
}

/// Delete response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
    pub livemode: bool,
}

/// List parameters for pagination
#[derive(Debug, Clone, Default)]
pub struct ListParams {
    pub limit: Option<u8>,
    pub offset: Option<u64>,
    pub since: Option<i64>,
    pub until: Option<i64>,
}

impl ListParams {
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(limit) = self.limit {
            params.push(("limit".to_string(), limit.to_string()));
        }
        if let Some(offset) = self.offset {
            params.push(("offset".to_string(), offset.to_string()));
        }
        if let Some(since) = self.since {
            params.push(("since".to_string(), since.to_string()));
        }
        if let Some(until) = self.until {
            params.push(("until".to_string(), until.to_string()));
        }

        params
    }
}
