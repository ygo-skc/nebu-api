use std::env::var;
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

pub struct Config {
    pub tcg_price_api_host: String,
}

impl Config {
    pub fn load() -> &'static Config {
        CONFIG.get_or_init(|| {
            let env_file_name = var("NEBU_API_DOT_ENV_FILE").unwrap_or(".env".to_string());
            dotenvy::from_filename(env_file_name).ok();
            Config {
                tcg_price_api_host: var("TCG_PRICE_API_HOST").expect("Missing TCG Pricing API url in env file"),
            }
        })
    }
}
