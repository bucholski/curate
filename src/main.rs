use clap::Parser;
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

    let amount: f64 = cli.amount.unwrap_or(1.0);
    let currency: &str = &cli.currency.unwrap_or(String::from("EUR"));

    match get_rate(currency) {
        Ok(rate) => print_output(rate, amount, currency),
        Err(e) => println!("Error when contacting the API: {e}"),
    }
}

fn get_rate(currency: &str) -> Result<serde_json::Value, ureq::Error> {
    let url = format!("https://api.nbp.pl/api/exchangerates/rates/a/{currency}/?format=json");
    let rate: serde_json::Value = ureq::get(&url).call()?.into_json()?;
    // println!("{:#?}", rate); //see what you get from the api
    Ok(rate)
}

fn print_output(rate: serde_json::Value, amount: f64, currency: &str) {
    match rate["rates"][0]["mid"].as_f64() {
        Some(mid) => println!("{currency} {amount} is PLN {:.2}", (mid * amount)),
        None => println!("Error - rate not a valid f64"),
    }
}

fn iterate_response(response: serde_json::Value) {
    println!("{:#?}", response)
}
