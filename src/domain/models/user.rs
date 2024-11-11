use serde::Serialize;
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct User {
    pub id: Option<Uuid>, // Its optional as we may get it only after creation in DB
    pub email: String,
    pub hashed_password: String,
}
