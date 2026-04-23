use serde::{Deserialize ,Serialize };
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug ,Serialize ,sqlx::FromRow)]

pub struct User{
    pub id: Uuid ,
    pub name : String ,
    pub email : String ,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]

pub struct CreateUser {
    pub name : String ,
    pub email: String ,
    pub  password : String ,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize , Serialize)]

pub struct Claims{
    pub sub : String,
    pub exp : usize ,
}

#[derive(Serialize)]
pub struct AuthResponse{
    pub token : String ,
}
#[derive(Debug, sqlx::FromRow)]
pub struct UserWithPassword {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<NaiveDateTime>,
}