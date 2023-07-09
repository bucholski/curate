use std::error::Error;

pub fn fetch_data(currency: &str, api_url: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let url = api_url.replace("{currency}", currency);
    let data = ureq::get(&url).call()?.into_json()?;
    // println!("{:#?}", data); //see what you get from the api
    Ok(data)
}

pub fn get_rate(data: &serde_json::Value, api_path: &str) -> Option<f64> {
    data.pointer(api_path)?.as_f64()
}
