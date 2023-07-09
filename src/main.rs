pub mod api_actions;
pub mod config_actions;
use api_actions::*;
use clap::{arg, Parser};
use config_actions::Config;

#[derive(Parser, Debug)]
#[command(
    name = "curate",
    author = "Misza B. <bucholskim@gmail.com>",
    version = "0.1.0",
    about = "Simple configurable currency rate lookup"
)]

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

    let (config_dir, config_file) = config_actions::determine_path();
    if let Err(e) = config_actions::verify_config(&config_dir, &config_file) {
        println!("Config file error: {e}");
    };

    let config: Config = config_actions::get_config(config_file).unwrap();
    let amount: f64 = cli.amount.unwrap_or(config.default_amount);
    let currency: &str = &cli.currency.unwrap_or(config.default_currency);
    match fetch_data(currency, &config.api_url) {
        Ok(data) => try_print(get_rate(&data, &config.rate_path), amount, currency),
        Err(e) => println!("API error: {e}"),
    }
}

fn try_print(rate: Option<f64>, amount: f64, currency: &str) {
    match rate {
        Some(rate) => println!("{currency} {amount} is PLN {:.2}", (rate * amount)),
        None => println!("Error - rate not a valid f64"),
    }
}
