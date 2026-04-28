use std::fs;
use std::sync::OnceLock;

use crate::models::configuration::MainConfig;

static CONFIG_CACHE: OnceLock<MainConfig> = OnceLock::new();
pub fn get_config() -> &'static MainConfig {
    CONFIG_CACHE.get_or_init(|| {
        let toml_content = fs::read_to_string("./config/configuration.toml")
            .expect("Failed reading configuration.toml");
        toml::from_str(&toml_content).expect("Toml format Incorrect !")
    })
}