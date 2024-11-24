use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct UserCreateDTO {
    pub username: String,
    pub password: String,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct UserOutDTO {
    pub id: Option<Uuid>,
    pub username: String,
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct SignInData {
    pub username: String,
    pub password: String,
}
