use crate::constants::{APP_NAME, ORG_NAME};
use anyhow::{Context, Result};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default, ValueEnum)]
pub enum OrderBy {
    #[default]
    None, // Random
    Name,
    CreatedAt,
    ModifiedAt,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub images_dir: PathBuf,
    #[serde(default)]
    pub order_by: OrderBy,
    #[serde(default)]
    pub reverse: bool,
    #[serde(default)]
    pub external_args: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            images_dir: PathBuf::new(),
            order_by: OrderBy::None,
            reverse: false,
            external_args: vec![
                "--transition-type".to_string(),
                "wipe".to_string(),
                "--transition-step".to_string(),
                "10".to_string(),
            ],
        }
    }
}

pub struct ConfigManager {
    config_dir: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap())
            .join(ORG_NAME)
            .join(APP_NAME);

        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)
                .with_context(|| format!("Error making config dir: {:?}", config_dir))?;
        }

        Ok(Self { config_dir })
    }

    pub fn load(&self) -> Result<Config> {
        let config_file = self.config_dir.join("config.toml");

        if !config_file.exists() {
            let default_config = Config::default();
            let toml_str = toml::to_string_pretty(&default_config)?;
            fs::write(&config_file, toml_str)?;
            return Ok(default_config);
        }

        let content = fs::read_to_string(config_file)?;
        let config: Config = toml::from_str(&content)?;

        Ok(config)
    }
}
