use axum::{Router, routing::{post, get}, middleware};
use sqlx::PgPool;
use crate::handlers;
use crate::middleware::auth;

pub fn create_router(pool: PgPool) -> Router {
    let protected = Router::new()
        .route("/categories", post(handlers::create_category))
        .route_layer(middleware::from_fn(auth));

    let public = Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login));

    Router::new()
        .merge(public)
        .merge(protected)
        .with_state(pool)
}