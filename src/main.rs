use clap;
// use serde_json::{self};
use ureq::{self};

fn main() {
    let amount: f64 = 30.0;
    let currency: &str = "EUR";
    let rate = get_rate(currency).unwrap();
    println!("{:#?}", rate);

    let mid = rate["rates"][0]["mid"].as_f64();

    match mid {
        Some(x) => println!("{currency} {amount} is PLN {}", x * amount),
        None => println!("Error - rate not a valid u64"),
    }
    // println!("1{code} is {}", mid * amount);
}

fn get_rate(currency: &str) -> Result<serde_json::Value, ureq::Error> {
    let url = format!("https://api.nbp.pl/api/exchangerates/rates/c/{currency}/?format=json");
    let rate: serde_json::Value = ureq::get(&url).call()?.into_json()?;
    Ok(rate)
}
