use chrono::{Duration, NaiveDateTime};
use reqwest::{Error as ApiError, Url};
use serde::Deserialize;
use std::error::Error;

use crate::errors_::{ApiResponseParseError, DateTimeError};
use crate::output_messages as msg;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub prices: Vec<(i64, f64)>,
    pub market_caps: Vec<(i64, f64)>,
    pub total_volumes: Vec<(i64, f64)>,
}

pub fn coingecko_get(url: Url) -> Result<ApiResponse, ApiError> {
    let api_response = reqwest::blocking::get(url)?;
    api_response.json()
}

pub fn parse_api_response(response: ApiResponse) -> Result<(NaiveDateTime, f64), Box<dyn Error>> {
    if response.prices.len() > 1 {
        // Log a warning as more than one price may come up but is rare
        println!("{}", ApiResponseParseError::MultiPrice);
        return Err(ApiResponseParseError::MultiPrice.into());
    }

    let (unix_time, price) = match response
        .prices
        .first()
        .ok_or(ApiResponseParseError::EmptyPrice) {
            Ok((a,b)) => (a,b),
            Err(_) => {
                println!("{}\n", ApiResponseParseError::EmptyPrice);
                println!("{}\n", msg::TRY_AGAIN);
                return Err(ApiResponseParseError::EmptyPrice.into());
            }
        };

    let result_dt = NaiveDateTime::from_timestamp_millis(*unix_time)
        .ok_or(DateTimeError::ConvertError)?;
    Ok((result_dt, *price))
}

// -------------
// Build URL
// -------------

pub fn build_url(asset: &str, target_date: &NaiveDateTime) -> reqwest::Url {
    let base_url = build_base_url(asset); // Question: Why is this not borrowed?
    let params = build_params(target_date); // Question: Why is this not borrowed?
    Url::parse_with_params(&base_url, params).unwrap() // Question: Why is this not borrowed?
}

pub fn build_base_url(asset: &str) -> String {
    format!(
        "https://api.coingecko.com/api/v3/coins/{}/market_chart/range",
        asset.trim().to_lowercase()
    )
}

pub fn build_params(target_date: &NaiveDateTime) -> Vec<(String, String)> {
    let currency = "USD".to_string();
    let early_dt = (*target_date - Duration::minutes(30)).timestamp();
    let later_dt = (*target_date + Duration::minutes(30)).timestamp();

    vec![
        ("vs_currency".to_string(), currency),
        ("from".to_string(), early_dt.to_string()),
        ("to".to_string(), later_dt.to_string()),
    ]
}
