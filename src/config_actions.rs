// pub mod config_actions {
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{error::Error, fs, io::Write, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_url: String,
    pub rate_path: String,
    // TODO - implement the fields below
    #[serde(default = "default_currency")]
    pub default_currency: String,
    #[serde(default = "default_amount")]
    pub default_amount: f64,
}

pub fn verify_config(config_dir: &PathBuf, config_file: &PathBuf) -> Result<(), Box<dyn Error>> {
    //no config dir
    if !config_dir.is_dir() {
        fs::DirBuilder::new().create(config_dir)?;
    }
    //no config file
    if !config_file.is_file() {
        let default = serde_json::json!({
          "api_url":"https://api.nbp.pl/api/exchangerates/rates/a/{currency}/?format=json",
          "rate_path":"/rates/0/mid"
        });
        let mut config: std::fs::File = fs::File::create(config_file)?;
        config.write_all(serde_json::ser::to_string_pretty(&default)?.as_bytes())?;
    }
    Ok(())
}

pub fn get_config(config_path: PathBuf) -> Result<Config, Box<dyn Error>> {
    let config_string = std::fs::read_to_string(config_path)?;
    let config_struct: Config = serde_json::from_str(&config_string)?;
    Ok(config_struct)
}

fn default_currency() -> String {
    //TODO - Pull from config file
    "EUR".into()
}

fn default_amount() -> f64 {
    //TODO - Pull from config file
    1_f64
}
// }
