use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::models::Claims;

pub async fn auth(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {

    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    let token = match auth_header {
        Some(t) => t,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let secret = std::env::var("JWT_SECRET").unwrap();

    let result = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    );

    if result.is_err() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}