use lazy_static::lazy_static;
use magic_crypt::{new_magic_crypt, MagicCrypt256};
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub encryption_key: String,
    #[serde(default = "default_port")]
    pub port: u16,
    pub redirect_uri: String,
    pub database_url: String,
    pub sentry_dsn: String,
    pub smtp_username: String,
    pub smtp_password: String,
}

fn default_port() -> u16 {
    8080
}

// Throw the Config struct into a CONFIG lazy_static to avoid multiple processing
lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

// MC for encrypting/decrypting stuff
lazy_static! {
    pub static ref MC: MagicCrypt256 = new_magic_crypt!(&CONFIG.encryption_key, 256);
}

/// Use envy to inject dotenv and env vars into the Config struct
fn get_config() -> Config {
    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Configuration Error: {:#?}", error),
    }
}
