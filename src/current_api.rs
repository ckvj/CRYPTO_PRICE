use std::error::Error;
use reqwest::{Url, Error as ApiError, blocking};
use serde_json::{Map, Value};


pub fn coingecko_get(url: Url) -> Result<blocking::Response, ApiError> {
    // dbg!(reqwest::blocking::get(url.clone()).unwrap());
    reqwest::blocking::get(url)
}

/// Parse CoinGecko API response for price endpoint. 
pub fn parse_api_response(response: blocking::Response) -> Result<(f64, f64), Box<dyn Error>> {
    //  Since Coingecko returns the asset name as a field (eg solana), we cannot deserialize into a struct
    // directly and instead need unpack values.
    
    let data: Map<String, Value> = serde_json::from_str(&response.text()?)?;
    
    let (price, change_24hr) = data
        .values()
        .flat_map(|value| {
            let price = value["usd"].as_f64();
            let change_24hr = value["usd_24h_change"].as_f64();
            price.zip(change_24hr)
        })
        .next()
        .unwrap_or((0.0, 0.0));

    Ok((price, change_24hr))
}

pub fn build_url(asset: &str) -> reqwest::Url {
    let base_url = "https://api.coingecko.com/api/v3/simple/price";
    let params = build_params(asset); // Question: Why is this not borrowed?
    Url::parse_with_params(base_url, params).unwrap() // Question: Why is this not borrowed?
}

pub fn build_params(asset: &str) -> Vec<(String, String)> {
    let currency = "USD".to_string();

    vec![
        ("ids".to_string(), asset.to_string().to_lowercase()),
        ("vs_currencies".to_string(), currency),
        ("include_24hr_change".to_string(), "true".to_string()),
    ]
}
