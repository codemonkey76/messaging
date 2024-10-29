pub mod client;
pub mod mock;
use crate::error::AppResult;

#[async_trait::async_trait]
pub trait ClickSendApi {
    async fn fetch_verified_numbers(&self) -> AppResult<Vec<String>>;
    async fn fetch_dedicated_numbers(&self) -> AppResult<Vec<String>>;
    async fn fetch_alpha_tags(&self) -> AppResult<Vec<String>>;
    async fn send_single_sms(&self, recipient: &str, sender: &str, message: &str) -> AppResult<()>;
    async fn validate_sender(&self, sender: &str) -> AppResult<()>;
}
