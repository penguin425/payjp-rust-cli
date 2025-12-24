use crate::error::{AppError, Result};
use crate::models::{CreateTokenParams, PayjpApiError, Token};
use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use std::time::Duration;

const BASE_URL: &str = "https://api.pay.jp/v1";

/// Token API client (uses public key)
pub struct TokenClient {
    client: Client,
    public_key: String,
    verbose: bool,
}

impl TokenClient {
    /// Create a new Token client with public key
    pub fn new(public_key: &str) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            public_key: public_key.to_string(),
            verbose: false,
        })
    }

    /// Enable verbose logging
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Create Basic Authentication header
    fn create_auth_header(&self) -> String {
        let credentials = format!("{}:", self.public_key);
        let encoded = STANDARD.encode(credentials);
        format!("Basic {}", encoded)
    }

    /// Build common headers
    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&self.create_auth_header()).unwrap(),
        );
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("payjp-cli/0.1.0 (Rust)"),
        );
        headers
    }

    /// Create a token from card details
    pub fn create_token(&self, params: &CreateTokenParams) -> Result<Token> {
        let url = format!("{}/tokens", BASE_URL);
        let headers = self.build_headers();

        if self.verbose {
            eprintln!("POST {} (using public key)", url);
        }

        let response = self
            .client
            .post(&url)
            .headers(headers)
            .form(&params.to_form_params())
            .send()?;

        let status = response.status();
        let body = response.text()?;

        if self.verbose {
            eprintln!("Response status: {}", status);
            eprintln!("Response body: {}", body);
        }

        if status.is_success() {
            let token: Token = serde_json::from_str(&body)?;
            Ok(token)
        } else {
            match serde_json::from_str::<PayjpApiError>(&body) {
                Ok(api_error) => Err(AppError::Api(api_error.error)),
                Err(_) => Err(AppError::Api(crate::models::PayjpErrorDetail {
                    status: status.as_u16(),
                    error_type: "unknown_error".to_string(),
                    code: None,
                    message: format!("Unknown error: {}", body),
                    param: None,
                })),
            }
        }
    }

    /// Get a token by ID
    pub fn get_token(&self, id: &str) -> Result<Token> {
        let url = format!("{}/tokens/{}", BASE_URL, id);
        let headers = self.build_headers();

        if self.verbose {
            eprintln!("GET {}", url);
        }

        let response = self.client.get(&url).headers(headers).send()?;

        let status = response.status();
        let body = response.text()?;

        if self.verbose {
            eprintln!("Response status: {}", status);
            eprintln!("Response body: {}", body);
        }

        if status.is_success() {
            let token: Token = serde_json::from_str(&body)?;
            Ok(token)
        } else {
            match serde_json::from_str::<PayjpApiError>(&body) {
                Ok(api_error) => Err(AppError::Api(api_error.error)),
                Err(_) => Err(AppError::Api(crate::models::PayjpErrorDetail {
                    status: status.as_u16(),
                    error_type: "unknown_error".to_string(),
                    code: None,
                    message: format!("Unknown error: {}", body),
                    param: None,
                })),
            }
        }
    }
}
