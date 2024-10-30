use crate::{
    error::AppResult,
    validators::{self, validate_sender_logic},
};

use super::ClickSendApi;

pub struct MockClickSendClient;

#[async_trait::async_trait]
impl ClickSendApi for MockClickSendClient {
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
        // Validate recipient number
        validators::validate_e164(recipient)?;
        self.validate_sender(sender).await?;

        println!(
            "Sending message from '{}' to '{}' - {}",
            recipient, sender, message
        );
        Ok(())
    }

    async fn fetch_verified_numbers(&self) -> AppResult<Vec<String>> {
        Ok(vec!["+1234567890".to_string(), "+1987654321".to_string()])
    }

    async fn fetch_dedicated_numbers(&self) -> AppResult<Vec<String>> {
        Ok(vec!["+11234567890".to_string()])
    }

    async fn fetch_alpha_tags(&self) -> AppResult<Vec<String>> {
        Ok(vec!["MYBUSINESS".to_string(), "ALPHAEXAMPLE".to_string()])
    }
}
