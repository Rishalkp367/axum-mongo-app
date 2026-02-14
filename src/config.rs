use std::env;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub mongodb_uri: String,
    pub mongodb_db: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into()),
            port: env
                ::var("PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse()
                .expect("PORT must be number"),
            mongodb_uri: env::var("MONGODB_URI").expect("MONGODB_URI not set"),
            mongodb_db: env::var("MONGODB_DB").expect("MONGODB_DB not set"),
        }
    }
}
