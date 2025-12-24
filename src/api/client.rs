use crate::error::{AppError, Result};
use crate::models::PayjpApiError;
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::Rng;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use std::thread;
use std::time::Duration;

const BASE_URL: &str = "https://api.pay.jp/v1";
const MAX_RETRIES: u32 = 5;
const INITIAL_BACKOFF_MS: u64 = 2000;
const MAX_BACKOFF_MS: u64 = 32000;

/// PAY.JP API client
#[derive(Debug, Clone)]
pub struct PayjpClient {
    client: Client,
    secret_key: String,
    verbose: bool,
}

impl PayjpClient {
    /// Create a new PAY.JP client
    pub fn new(secret_key: &str) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client,
            secret_key: secret_key.to_string(),
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
        let credentials = format!("{}:", self.secret_key);
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

    /// Build URL for an endpoint
    fn build_url(&self, endpoint: &str) -> String {
        format!("{}{}", BASE_URL, endpoint)
    }

    /// Calculate backoff duration with jitter
    fn calculate_backoff(&self, retry_count: u32) -> Duration {
        let base_ms = INITIAL_BACKOFF_MS * 2u64.pow(retry_count);
        let capped_ms = base_ms.min(MAX_BACKOFF_MS);

        // Add jitter: ±50%
        let mut rng = rand::thread_rng();
        let jitter_factor = 0.5 + rng.gen::<f64>();
        let final_ms = (capped_ms as f64 * jitter_factor) as u64;

        Duration::from_millis(final_ms)
    }

    /// Execute a request with retry logic
    fn execute_with_retry<F>(&self, request_fn: F) -> Result<Response>
    where
        F: Fn() -> std::result::Result<Response, reqwest::Error>,
    {
        let mut retry_count = 0;

        loop {
            match request_fn() {
                Ok(response) => {
                    if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                        if retry_count >= MAX_RETRIES {
                            return Err(AppError::MaxRetriesExceeded);
                        }

                        let backoff = self.calculate_backoff(retry_count);
                        if self.verbose {
                            eprintln!(
                                "Rate limited. Retrying in {:?} (attempt {}/{})",
                                backoff,
                                retry_count + 1,
                                MAX_RETRIES
                            );
                        }

                        thread::sleep(backoff);
                        retry_count += 1;
                        continue;
                    }

                    return Ok(response);
                }
                Err(e) => {
                    if e.is_timeout() || e.is_connect() {
                        if retry_count >= MAX_RETRIES {
                            return Err(AppError::MaxRetriesExceeded);
                        }

                        let backoff = self.calculate_backoff(retry_count);
                        if self.verbose {
                            eprintln!(
                                "Network error. Retrying in {:?} (attempt {}/{}): {}",
                                backoff,
                                retry_count + 1,
                                MAX_RETRIES,
                                e
                            );
                        }

                        thread::sleep(backoff);
                        retry_count += 1;
                        continue;
                    }

                    return Err(AppError::Http(e));
                }
            }
        }
    }

    /// Parse response and handle errors
    fn parse_response<T: serde::de::DeserializeOwned>(&self, response: Response) -> Result<T> {
        let status = response.status();
        let body = response.text()?;

        if self.verbose {
            eprintln!("Response status: {}", status);
            eprintln!("Response body: {}", body);
        }

        if status.is_success() {
            let result: T = serde_json::from_str(&body)?;
            Ok(result)
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

    /// GET request
    pub fn get<T: serde::de::DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let url = self.build_url(endpoint);
        let headers = self.build_headers();

        if self.verbose {
            eprintln!("GET {}", url);
        }

        let response = self.execute_with_retry(|| {
            self.client.get(&url).headers(headers.clone()).send()
        })?;

        self.parse_response(response)
    }

    /// GET request with query parameters
    pub fn get_with_params<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &[(String, String)],
    ) -> Result<T> {
        let url = self.build_url(endpoint);
        let headers = self.build_headers();

        if self.verbose {
            eprintln!("GET {} with params: {:?}", url, params);
        }

        let response = self.execute_with_retry(|| {
            self.client
                .get(&url)
                .headers(headers.clone())
                .query(params)
                .send()
        })?;

        self.parse_response(response)
    }

    /// POST request with form body
    pub fn post<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &[(String, String)],
    ) -> Result<T> {
        let url = self.build_url(endpoint);
        let headers = self.build_headers();

        if self.verbose {
            eprintln!("POST {} with body: {:?}", url, params);
        }

        let response = self.execute_with_retry(|| {
            self.client
                .post(&url)
                .headers(headers.clone())
                .form(params)
                .send()
        })?;

        self.parse_response(response)
    }

    /// DELETE request
    pub fn delete<T: serde::de::DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let url = self.build_url(endpoint);
        let headers = self.build_headers();

        if self.verbose {
            eprintln!("DELETE {}", url);
        }

        let response = self.execute_with_retry(|| {
            self.client.delete(&url).headers(headers.clone()).send()
        })?;

        self.parse_response(response)
    }
}
