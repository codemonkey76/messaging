use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SmsRequest {
    pub phone_number: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct ApiResponse {
    pub status: u32,
    pub message: String,
}
