use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct UserCreateDTO {
    pub email: String,
    pub password: String,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct UserOutDTO {
    pub id: Option<Uuid>,
    pub email: String,
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct SignInData {
    pub email: String,
    pub password: String,
}
