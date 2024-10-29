use regex::Regex;

use crate::error::{AppError, AppResult};

pub fn validate_e164(phone_number: &str) -> AppResult<()> {
    let re = Regex::new(r"^\+[1-9]\d{1,14}$").expect("Invalid regex for E.164 format");

    if re.is_match(phone_number) {
        Ok(())
    } else {
        Err(AppError::InvalidPhoneNumber(phone_number.into()))
    }
}

pub async fn validate_sender_logic<'a, F, G, H, I>(
    sender: &str,
    validate_e164: F,
    fetch_verified_numbers: G,
    fetch_dedicated_numbers: H,
    fetch_alpha_tags: I,
) -> AppResult<()>
where
    F: Fn(&str) -> AppResult<()>,
    G: Fn() -> std::pin::Pin<
        Box<dyn std::future::Future<Output = AppResult<Vec<String>>> + Send + 'a>,
    >,
    H: Fn() -> std::pin::Pin<
        Box<dyn std::future::Future<Output = AppResult<Vec<String>>> + Send + 'a>,
    >,
    I: Fn() -> std::pin::Pin<
        Box<dyn std::future::Future<Output = AppResult<Vec<String>>> + Send + 'a>,
    >,
{
    if sender.starts_with('+') {
        // Validate E.164 format
        validate_e164(sender)?;

        // Call the ClickSend API to get a list of verified own numbers and dedicated numbers
        let verified_numbers = fetch_verified_numbers().await?;
        let dedicated_numbers = fetch_dedicated_numbers().await?;

        // Check if the sender is in either list
        if !verified_numbers.contains(&sender.to_string())
            && !dedicated_numbers.contains(&sender.to_string())
        {
            return Err(AppError::InvalidSender(sender.to_string()));
        }
    } else {
        // Check if the sender is a registered Alpha Tag
        let alpha_tags = fetch_alpha_tags().await?;

        if !alpha_tags.contains(&sender.to_string()) {
            return Err(AppError::InvalidSender(sender.to_string()));
        }
    }

    Ok(())
}
