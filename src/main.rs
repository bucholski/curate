use clap::Parser;
use directories::BaseDirs;
use serde_json;
use std::{fs, io::Write, path::PathBuf};
use ureq::{self};

#[derive(Parser, Debug)]
#[command(name = "curate")]
#[command(author = "Misza B. <bucholskim@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Simple currency rate lookup")]

struct Cli {
    currency: Option<String>,
    amount: Option<f64>,
}

fn main() {
    let cli = Cli::parse();

    //THIS WILL BE INPUT EITHER FROM ARGUMENTS OR A CONFIG FILE
    let api_path = "/rates/0/mid";
    let amount: f64 = cli.amount.unwrap_or(1.0);
    let currency: &str = &cli.currency.unwrap_or(String::from("EUR"));
    check_config();
    //THIS WILL BE INPUT EITHER FROM ARGUMENTS OR A CONFIG FILE

    match fetch_data(currency) {
        // Ok(rate) => print_output(rate, amount, currency),
        Ok(data) => print_output(get_rate(&data, api_path), amount, currency),

        Err(e) => println!("Error when contacting the API: {e}"),
    }
}

fn fetch_data(currency: &str) -> Result<serde_json::Value, ureq::Error> {
    let url = format!("https://api.nbp.pl/api/exchangerates/rates/a/{currency}/?format=json");
    let data: serde_json::Value = ureq::get(&url).call()?.into_json()?;
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

//Volatile stuff below

fn check_config() {
    let binding = BaseDirs::new().unwrap();
    let mut config_dir: PathBuf = binding.config_dir().to_path_buf();
    config_dir.push("curate");
    let mut config_file = config_dir.clone();
    config_file.push("config");
    if !config_dir.is_dir() {
        fs::DirBuilder::new().create(config_dir).unwrap();
    }
    if !config_file.is_file() {
        let default = serde_json::json!({
          "api_url":"https://api.nbp.pl/api/exchangerates/rates/a/{currency}/?format=json",
          "rate_path":"/rates/0/mid"
        });
        let mut config = fs::File::create(config_file).unwrap();
        config
            .write_all(
                serde_json::ser::to_string_pretty(&default)
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
    }
}
// todo!("BLOOMBERG API");
