use base64::{engine::general_purpose, Engine};
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};
use serde::Deserialize;

use super::ClickSendApi;
use crate::{
    error::{AppError, AppResult},
    validators::{self, validate_sender_logic},
};
use serde_json;

pub struct ClickSendClient {
    client: Client,
    base_url: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct OwnNumber {
    phone_number: String,
}

#[derive(Debug, Deserialize)]
struct OwnNumbersResponse {
    own_numbers: Vec<OwnNumber>,
}

#[derive(Debug, Deserialize)]
struct DedicatedNumber {
    dedicated_number: String,
}

#[derive(Debug, Deserialize)]
struct DedicatedNumbersData {
    data: Vec<DedicatedNumber>,
}

#[derive(Debug, Deserialize)]
struct DedicatedNumbersResponse {
    data: DedicatedNumbersData,
}

impl ClickSendClient {
    pub fn new(api_key: &str, username: &str, base_url: &str, version: &str) -> AppResult<Self> {
        // Construct basic auth credentials and encode them
        let credentials = format!("{}:{}", username, api_key);
        let encoded_creds = general_purpose::STANDARD.encode(credentials);

        // Create default headers
        let mut headers = HeaderMap::new();
        headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        // Construct the Auth header
        let auth_header_value = format!("Basic {}", encoded_creds);
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&auth_header_value).map_err(|_| {
                AppError::ClickSendApiError("Unable to construct authorization header".into())
            })?,
        );

        // Build the reqwest client with the default headers
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|_| {
                AppError::ClickSendApiError("Unable to construct request client".into())
            })?;

        Ok(Self {
            client,
            base_url: base_url.to_string(),
            version: version.to_string(),
        })
    }

    fn construct_url(&self, endpoint: &str) -> String {
        let url = format!("{}/{}/{}", self.base_url, self.version, endpoint);

        url
    }
}

#[async_trait::async_trait]
impl ClickSendApi for ClickSendClient {
    async fn validate_sender(&self, sender: &str) -> AppResult<()> {
        validate_sender_logic(
            sender,
            validators::validate_e164,
            || Box::pin(self.fetch_verified_numbers()),
            || Box::pin(self.fetch_dedicated_numbers()),
            || Box::pin(self.fetch_alpha_tags()),
        )
        .await
    }

    async fn send_single_sms(&self, recipient: &str, sender: &str, message: &str) -> AppResult<()> {
        // 1. Validate recipient number (must be in E.164 format)
        validators::validate_e164(recipient)?;

        // 2. Validate the sender (either own number, dedicated number or alpha tag)
        self.validate_sender(sender).await?;

        // 3. build the request URL for sending SMS
        let url = self.construct_url("sms/send");

        // 4. Prepare the payload for the SMS request
        let payload = serde_json::json!({
            "messages": [
                {
                    "body": message,
                    "to": recipient,
                    "from": sender,
                    "source": "api",
                }
            ]
        });

        // 5. Make the API request
        let response = self.client.post(&url).json(&payload).send().await;

        // 6. Handle the API response
        match response {
            Ok(res) if res.status().is_success() => Ok(()), // SMS sent successfully
            Ok(res) => {
                let status = res.status();
                let error_message = res
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(AppError::MessageSendFailed(format!(
                    "Failed with status {}: {}",
                    status, error_message
                )))
            }
            Err(err) => Err(AppError::MessageSendFailed(err.to_string())),
        }
    }

    async fn fetch_verified_numbers(&self) -> AppResult<Vec<String>> {
        let url = self.construct_url("own-numbers");
        let response = self.client.get(&url).send().await;

        match response {
            Ok(res) if res.status().is_success() => {
                let body_text = res
                    .text()
                    .await
                    .unwrap_or_else(|_| "Failed to get response text".to_string());

                let own_numbers_response: OwnNumbersResponse = serde_json::from_str(&body_text)
                    .unwrap_or_else(|err| {
                        println!("Failed to deserialize JSON: {}", err);
                        OwnNumbersResponse {
                            own_numbers: Vec::new(),
                        }
                    });

                let phone_numbers: Vec<String> = own_numbers_response
                    .own_numbers
                    .into_iter()
                    .map(|own_number| own_number.phone_number)
                    .collect();
                Ok(phone_numbers)
            }
            Ok(res) => {
                let status = res.status();
                let error_message = res
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(AppError::MessageSendFailed(format!(
                    "Failed with status {}: {}",
                    status, error_message
                )))
            }
            Err(err) => Err(AppError::MessageSendFailed(err.to_string())),
        }
    }

    async fn fetch_dedicated_numbers(&self) -> AppResult<Vec<String>> {
        let url = self.construct_url("numbers");
        let response = self.client.get(&url).send().await;

        match response {
            Ok(res) if res.status().is_success() => {
                let body_text = res
                    .text()
                    .await
                    .unwrap_or_else(|_| "Failed to get response text".to_string());

                let dedicated_numbers: DedicatedNumbersResponse = serde_json::from_str(&body_text)
                    .unwrap_or_else(|err| {
                        println!("Failed to deserialize JSON: {}", err);
                        DedicatedNumbersResponse {
                            data: DedicatedNumbersData { data: Vec::new() },
                        }
                    });

                let phone_numbers: Vec<String> = dedicated_numbers
                    .data
                    .data
                    .into_iter()
                    .map(|dedicated_number| dedicated_number.dedicated_number)
                    .collect();
                Ok(phone_numbers)
            }
            Ok(res) => {
                let status = res.status();
                let error_message = res
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(AppError::MessageSendFailed(format!(
                    "Failed to fetch verified numbers {}: {}",
                    status, error_message
                )))
            }
            Err(err) => Err(AppError::MessageSendFailed(err.to_string())),
        }
    }

    async fn fetch_alpha_tags(&self) -> AppResult<Vec<String>> {
        Ok([].to_vec())
    }
}
