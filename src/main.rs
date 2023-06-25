pub mod api_actions;
pub mod config_actions;
use clap::{arg, Parser};
use config_actions::Config;
use directories::BaseDirs;
// use serde::{Deserialize, Serialize};
use serde_json;
use std::{error::Error, path::PathBuf};
use ureq::{self};

// #[derive(Debug, Serialize, Deserialize)]
// struct Config {
//     api_url: String,
//     rate_path: String,
//     // TODO - implement the fields below
//     #[serde(default = "default_currency")]
//     default_currency: String,
//     #[serde(default = "default_amount")]
//     default_amount: f64,
// }

#[derive(Parser, Debug)]
#[command(name = "curate")]
#[command(author = "Misza B. <bucholskim@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Simple currency rate lookup")]

struct Cli {
    ///ISO 4217 currency code
    currency: Option<String>,

    //TODO change amount to something more precise than f64 (currency crate?)
    amount: Option<f64>,
    ///set your own api URL with "{currency}" in place where currency code goes
    #[arg(short, long, value_name = "URL")]
    url: Option<String>,
    ///set path to value of currency in returned JSON as in "/Object/Array/0/value_key"
    #[arg(short, long, value_name = "PATH")]
    pointer: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    //config location //TODO - Should I use statics?
    let binding = match BaseDirs::new() {
        Some(dir) => dir,
        None => panic!("Couldn't find a viable home directory to put config in"),
    };

    let config_dir: PathBuf = [binding.config_dir().to_path_buf(), "curate".into()]
        .iter()
        .collect();
    // config_dir.push("curate");
    let config_file: PathBuf = [&config_dir, &"config".into()].iter().collect();

    if let Err(e) = config_actions::verify_config(&config_dir, &config_file) {
        println!("Config file error: {e}");
    };
    let config: Config = config_actions::get_config(config_file).unwrap();

    let amount: f64 = cli.amount.unwrap_or(config.default_amount);
    let currency: &str = &cli.currency.unwrap_or(config.default_currency);

    match fetch_data(currency, &config.api_url) {
        Ok(data) => print_output(get_rate(&data, &config.rate_path), amount, currency),
        Err(e) => println!("API error: {e}"),
    }
}

fn fetch_data(currency: &str, api_url: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let url = api_url.replace("{currency}", currency);
    let data = ureq::get(&url).call()?.into_json()?;

    // println!("{:#?}", data); //see what you get from the api
    Ok(data)
}

fn get_rate(data: &serde_json::Value, api_path: &str) -> Option<f64> {
    data.pointer(api_path)?.as_f64()
}

fn print_output(rate: Option<f64>, amount: f64, currency: &str) {
    match rate {
        Some(rate) => println!("{currency} {amount} is PLN {:.2}", (rate * amount)),
        None => println!("Error - rate not a valid f64"),
    }
}
