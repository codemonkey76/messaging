use crate::{clicksend::ClickSendApi, AppResult, ClickSendClient};

pub struct MessageService {
    client: ClickSendClient,
}

impl MessageService {
    pub fn new(api_key: &str, username: &str, base_url: &str, version: &str) -> AppResult<Self> {
        Ok(Self {
            client: ClickSendClient::new(api_key, username, base_url, version)?,
        })
    }

    pub async fn send_single_sms(
        &self,
        recipient: &str,
        sender: &str,
        message: &str,
    ) -> AppResult<()> {
        self.client
            .send_single_sms(recipient, sender, message)
            .await
    }
}
