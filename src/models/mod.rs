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