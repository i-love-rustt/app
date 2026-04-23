use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;


use crate::models::{User, CreateUser, LoginUser, Claims, AuthResponse, UserWithPassword, Category, CreateCategory};
use bcrypt;
use jsonwebtoken::{encode, EncodingKey, Header};
use std::time::{SystemTime, UNIX_EPOCH};


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
pub async fn login(
    State(pool): State<PgPool>,
    Json(body): Json<LoginUser>,
) -> (StatusCode, Json<AuthResponse>) {

    let user = sqlx::query_as!(
    UserWithPassword,
    "SELECT id, name, email, password_hash, created_at FROM users WHERE email = $1",
    body.email
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    let user = match user {
    Some(u) => u ,
    None => return (StatusCode::UNAUTHORIZED , Json(AuthResponse {token:"كلمه السر او الايميل خظا".to_string()})),
    } ;

    let valid  = bcrypt::verify(&body.password , &user.password_hash).unwrap() ;
    if !valid {
        return (StatusCode::UNAUTHORIZED , Json(AuthResponse {token:"كلمه السر او الايميل خظا".to_string()}));
    }


    let secret  = std::env::var("JWT_SECRET").unwrap() ;


    let exp  = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs()as usize + 604800;
      
      let claims = Claims{
        sub: user.id.to_string(),
        exp,
      } ;

      let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()) ,

    )

    .unwrap();
    (StatusCode::OK ,Json(AuthResponse {token}))





}

pub async fn   create_category(
    State(pool): State<PgPool>,
    Json(body) : Json<CreateCategory>,
)-> (StatusCode ,Json<Category>){

    let category = sqlx::query_as!(
        Category,
        "INSERT INTO categories (name, parent_id) VALUES ($1, $2) RETURNING id, name, parent_id, created_at",
        body.name,
        body.parent_id

    )
    .fetch_one(&pool)
    .await
    .unwrap();
    (StatusCode::CREATED , Json(category))

    
} 