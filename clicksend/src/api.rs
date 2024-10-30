use crate::{clicksend::ClickSendApi, AppResult};

pub struct MessageService<T: ClickSendApi> {
    client: T,
}

impl<T: ClickSendApi> MessageService<T> {
    pub fn new(client: T) -> Self {
        Self { client }
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
