use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenClaims {
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
}
