use clicksend::{api::MessageService, clicksend::mock::MockClickSendClient};

#[tokio::test]
async fn test_send_single_message() {
    let client = MockClickSendClient;
    let service = MessageService::new(client);

    let result = service
        .send_single_sms("+123456789", "+1234567890", "Test message")
        .await;

    if result.is_err() {
        println!("Test failed with error: {}", result.as_ref().unwrap_err());
    }

    assert!(result.is_ok());
}
