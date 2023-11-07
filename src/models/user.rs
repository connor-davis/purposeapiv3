use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum UserType {
    STANDARD,
    ADMIN,
}

trait UserTypeAsString {
    fn as_str(&self) -> &'static str;
}

impl UserTypeAsString for UserType {
    fn as_str(&self) -> &'static str {
        match self {
            UserType::STANDARD => "standard",
            UserType::ADMIN => "admin",
        }
    }
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub user_type: String,
    pub user_group: String,
    pub user_profile: Option<Uuid>,
}
