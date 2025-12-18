use axum::{
    middleware::Next,
    response::Response,
    http::Request,
    body::Body,
};

use crate::error::api_error::ApiError;

pub async fn auth(req:Request<Body>,next:Next)->Result<Response,ApiError>{
    let token=req.headers().get("X_Auth");

    match token{
        Some(value) if value=="SECRET123"
        =>Ok(next.run(req).await),

        _ =>Err(ApiError::unauthorized("Invalid or missing token")),

    }
}

    