use reqwest::{blocking, Error as ApiError, Url};
use serde_json::{Map, Value};

use super::helpers::errors_::ApiResponseParseError;

/// Call coingecko API
pub fn coingecko_get(url: Url) -> Result<blocking::Response, ApiError> {
    reqwest::blocking::get(url)
}

/// Parse coingecko API response for the 'simple/price' endpoint.
pub fn parse_api_response(
    response: blocking::Response,
) -> Result<(f64, f64), ApiResponseParseError> {
    //  Since Coingecko returns the asset name as a field (eg solana), we cannot deserialize into a struct
    // directly and instead need unpack values.

    // Parse API response into a map
    let response_text = response
        .text()
        .map_err(|_e| ApiResponseParseError::DeserializationError)?;
    let data: Map<String, Value> = serde_json::from_str(&response_text)
        .map_err(|_e| ApiResponseParseError::DeserializationError)?;

    // Return error for empty API response
    if data.is_empty() {
        return Err(ApiResponseParseError::Empty);
    }

    // Unpack response
    let (price, change_24hr) = data
        .values()
        .flat_map(|value| {
            let price = value["usd"].as_f64();
            let change_24hr = value["usd_24h_change"].as_f64();
            price.zip(change_24hr)
        })
        .next()
        .ok_or(ApiResponseParseError::PriceParseError)?;

    Ok((price, change_24hr))
}

// -------------
// Build URL
// -------------

pub fn build_url(asset: &str) -> reqwest::Url {
    let base_url = "https://api.coingecko.com/api/v3/simple/price";
    let params = build_params(asset);
    Url::parse_with_params(base_url, params).unwrap()
}

pub fn build_params(asset: &str) -> Vec<(String, String)> {
    let currency = "USD".to_string();
    let bool_24hr = "true".to_string();

    vec![
        ("ids".to_string(), asset.to_string().to_lowercase()),
        ("vs_currencies".to_string(), currency),
        ("include_24hr_change".to_string(), bool_24hr),
    ]
}
