use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
    pub user_group: String,
}
