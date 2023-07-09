// pub mod config_actions {
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{error::Error, fs, io::Write, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_url: String,
    pub rate_path: String,
    #[serde(default = "get_default_currency")]
    pub default_currency: String,
    #[serde(default = "get_default_amount")]
    pub default_amount: f64,
}

pub fn determine_path() -> (PathBuf, PathBuf) {
    //returns (dir,file)
    let binding = match BaseDirs::new() {
        Some(dir) => dir,
        None => panic!("Couldn't find a viable home directory to put config in"),
    };

    let config_dir: PathBuf = [binding.config_dir().to_path_buf(), "curate".into()]
        .iter()
        .collect();

    let config_file: PathBuf = [&config_dir, &"config".into()].iter().collect();
    (config_dir, config_file)
}

pub fn verify_config(config_dir: &PathBuf, config_file: &PathBuf) -> Result<(), Box<dyn Error>> {
    //no config dir found
    if !config_dir.is_dir() {
        fs::DirBuilder::new().create(config_dir)?;
    }
    //no config file found
    if !config_file.is_file() {
        let default = serde_json::json!({
          "api_url":"https://api.nbp.pl/api/exchangerates/rates/a/{currency}/?format=json",
          "rate_path":"/rates/0/mid",
          "default_currency":"EUR",
          "default_amount":1_f64
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

fn get_default_currency() -> String {
    let path = determine_path();
    println!("{:?}", path);
    let config = get_config(path.1);
    if let Ok(config_struct) = config {
        config_struct.default_currency
    } else {
        "EUR".into()
    }
}

fn get_default_amount() -> f64 {
    let path = determine_path();
    let config = get_config(path.1);
    if let Ok(config_struct) = config {
        config_struct.default_amount
    } else {
        1_f64
    }
}

fn set_api_url() -> String {
    todo!()
}

fn set_value_path() -> String {
    todo!()
}

fn set_default_currency() -> String {
    todo!()
}

fn set_default_amount() -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn defaults() {
        assert!(!get_default_currency().is_empty());
        assert_eq!(get_default_currency().len(), 3);
        assert!(get_default_amount() > 0_f64)
    }
}
