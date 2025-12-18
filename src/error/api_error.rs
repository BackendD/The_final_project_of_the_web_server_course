use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiError {
    pub message:String,
}
impl ApiError {
    pub fn unauthorized(msg:&str) -> Self {
        Self {message:msg.to_string()}
    }
    pub fn not_found(msg:&str) -> Self {
        Self{message:msg.to_string()}
    }
    pub fn internal(msg:&str) -> Self {
        Self {message:msg.to_string()}
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body=serde_json::to_string(&self).unwrap();
        (StatusCode::BAD_REQUEST,body).into_response()
    }
}
