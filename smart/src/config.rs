use anyhow::Result;
use serde::Deserialize;
use std::{env, str::FromStr, sync::Arc};
use tokio::{fs, sync::OnceCell};
use validator::Validate;

#[derive(Clone, Debug, Validate, Deserialize)]
pub struct Config {
    #[validate(length(min = 1))]
    pub database_url: String, // database url
    #[validate(length(min = 1))]
    pub web_host_uri: String, // host url
    #[validate(length(min = 1))]
    pub solana_rpc_url: String, // solana rpc url
    #[validate(length(min = 1))]
    pub solana_wss_url: String, // solana wss url
    #[validate(length(min = 1))]
    pub strategies_file_path: String, // strategies file path
    #[validate(range(min = 1))]
    pub upload_strategy_file_interval: u32, // upload strategy file interval seconds
}

impl FromStr for Config {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}

pub static GLOBAL_CONFIG: OnceCell<Arc<Config>> = OnceCell::const_new();

pub async fn get_global_config() -> &'static Arc<Config> {
    let config_url = env::var("SMART_CONFIG").expect("SMART_CONFIG is not set env");

    GLOBAL_CONFIG
        .get_or_init(|| async {
            Arc::new(
                fs::read_to_string(config_url)
                    .await
                    .expect("Failed to read config file")
                    .parse::<Config>()
                    .expect("Failed to parse config"),
            )
        })
        .await
}
