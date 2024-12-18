use anyhow::Result;
use std::{env, str::FromStr, sync::Arc};
use tokio::{fs, sync::OnceCell};
use validator::Validate;

#[derive(Clone, Debug, Validate, serde::Deserialize)]
pub struct Config {
    #[validate(length(min = 1))]
    pub database_url: String, // database url
    #[validate(length(min = 1))]
    pub host_uri: String, // host url
    #[validate(range(min = 1))]
    pub solana_rpc_curl_interval: u64, // solana rpc curl interval, eg 60 -> 60s
}

impl FromStr for Config {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}

pub static GLOBAL_CONFIG: OnceCell<Arc<Config>> = OnceCell::const_new();

pub async fn get_global_config() -> &'static Arc<Config> {
    let config_url = env::var("ANGEL_CONFIG").expect("ANGEL_CONFIG is not set env");

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
