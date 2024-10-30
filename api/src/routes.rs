use std::sync::Arc;

use axum::{
    body::Body,
    extract::{rejection::JsonRejection, State},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing, Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use shared::{ApiResponse, SmsRequest};

use crate::AppState;

pub async fn send_sms(
    State(app_state): State<AppState>,
    result: Result<Json<SmsRequest>, JsonRejection>,
) -> impl IntoResponse {
    if let Err(_) = result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                status: 500,
                message: "Malformed request".to_string(),
            }),
        );
    }

    let Json(payload) = result.unwrap();

    let sms_message = SmsRequest {
        phone_number: payload.phone_number,
        message: payload.message,
    };

    match app_state.rabbitmq.publish_message(sms_message).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse {
                status: 200,
                message: "Message queued".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                status: 500,
                message: "Failed to queue the message".to_string(),
            }),
        ),
    }
}

pub fn app(app_state: AppState) -> axum::Router {
    axum::Router::new()
        .route("/send_sms", routing::post(send_sms))
        .route_layer(middleware::from_fn(auth_middleware))
        .with_state(app_state)
}

async fn auth_middleware<B>(
    req: Request<Body>,
    next: Next,
) -> Response {
    State(state): State<Arc<AppState>>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    let token = bearer.token();

    if !state.valid_api_keys.contains(token) {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    next.run(req).await
}
