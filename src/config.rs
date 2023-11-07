use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
}

impl Config {
    pub fn init() -> Config {
        let database_url =
            env::var("DATABASE_URL").expect("Failed to find DATABASE_URL environment variable.");
        let jwt_secret =
            env::var("JWT_SECRET").expect("Failed to find JWT_SECRET environment variable.");
        let jwt_expires_in = env::var("JWT_EXPIRED_IN")
            .expect("Failed to find JWT_EXPIRED_IN environment variable.");
        let jwt_maxage =
            env::var("JWT_MAXAGE").expect("Failed to find JWT_MAXAGE environment variable.");

        Config {
            database_url,
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
        }
    }
}
