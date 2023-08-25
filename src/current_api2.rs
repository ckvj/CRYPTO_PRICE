// /// Built but then got 403 Errors. Waiting to talk to CoinGecko

// use std::error::Error;

// use reqwest::{Url, Error as ApiError, blocking};
// use serde_json::{Map, Value};
// use serde::Deserialize;


// #[derive(Debug, Deserialize)]
// pub struct ApiResponse {
//     pub current_price: f64,
//     pub price_change_percentage_24h_in_currency: f64,
//     pub price_change_percentage_7d_in_currency:f64,
//     pub ath_change_percentage: f64,
// }


// pub fn coingecko_get(url: Url) -> Result<blocking::Response, ApiError> {
//     dbg!(reqwest::blocking::get(url.clone()).unwrap());
//     reqwest::blocking::get(url)
// }

// /// Parse CoinGecko API response for price endpoint. 
// pub fn parse_api_response(response: blocking::Response) -> Result<(f64, f64), Box<dyn Error>> {
    
//     println!("{}", response.status());
//     let data: Map<String, Value> = serde_json::from_str(&response.text()?)?;
//     println!("{:?}", &data);

//     // let body = response.text()?;
//     // println!("{:?}", &body);
//     // let resp: Vec<ApiResponse> = serde_json::from_str(&body).unwrap();

//     let price = 55.5;
//     let change_24hr = 66.6;
//     Ok((price, change_24hr))
// }

// pub fn build_url(asset: &str) -> reqwest::Url {
//     let base_url = "https://api.coingecko.com/api/v3/coins/markets";
//     let params = build_params(asset); // Question: Why is this not borrowed?
//     Url::parse_with_params(base_url, params).unwrap() // Question: Why is this not borrowed?
// }

// pub fn build_params(asset: &str) -> Vec<(String, String)> {
//     let currency = "usd".to_string();
//     let percent_change_periods = "1h,24h,7d,14d,30d,200d,1y";

//     vec![
//         ("vs_currency".to_string(), currency),
//         ("ids".to_string(), asset.to_string().to_lowercase()),
//         ("price_change_percentage".to_string(), percent_change_periods.to_string()),
//     ]
// }
