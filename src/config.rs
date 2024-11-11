use once_cell::sync::Lazy;

pub struct Config {
    pub database_uri: String,
    pub jwt_secret: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    jwt_secret: dotenvy::var("JWT_SECRET")
        .expect("JWT_SECRET env var should be set")
        .to_string(),
    database_uri: dotenvy::var("DATABASE_URL").expect("DATABASE_URL env var should be set"),
});
