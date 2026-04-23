use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;
use crate::models::{User, CreateUser};
use bcrypt;

pub async fn register(
    State(pool): State<PgPool>,
    Json(body): Json<CreateUser>,
) -> (StatusCode, Json<User>) {

    let password_hash = bcrypt::hash(&body.password, 12).unwrap();

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING id, name, email, created_at",
        body.name,
        body.email,
        password_hash
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    (StatusCode::CREATED, Json(user))
}