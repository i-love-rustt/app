use axum::{ Router ,routing::post};
use sqlx::PgPool;
use crate::handlers;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
    .route("/register", post(handlers::register))
    .with_state(pool)
    
}