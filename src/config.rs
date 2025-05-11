use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub api_key: String,
    pub database_url: String,
    pub environment: String,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        // .envファイルを読み込む
        dotenv::dotenv().ok();

        let settings = config::Config::builder()
            // デフォルトの設定
            .add_source(config::Environment::default())
            .build()?;

        settings.try_deserialize()
    }

    pub fn get_api_key() -> String {
        env::var("API_KEY").expect("API_KEY must be set")
    }
} 