use clicksend::clicksend::mock;

#[tokio::test]
async fn test_send_message() {
    let mock_api = mock::MockClickSendApi;

    let result = clicksend::send_single_sms("+123456789", "+1234567890", "Test message").await;

    if result.is_err() {
        println!("Test failed with error: {}", result.as_ref().unwrap_err());
    }

    assert!(result.is_ok());
}
