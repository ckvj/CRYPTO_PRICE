use std::error::Error;
use chrono::NaiveDateTime;

use crate::current_api;
use crate::hist_api;

pub fn process_current_price(asset: &str) -> Result<(), Box<dyn Error>> {
    println!("\nINPUT:\n Asset: {:?}\n Date: Now", &asset);

    let response = current_api::coingecko_get(current_api::build_url(asset))?;

    let (price, change_24hr) = current_api::parse_api_response(response)?;
        
    println!("\nOUTPUT:\n Price: ${} USD\n 24hr Change (%): {}", &price, &change_24hr);
    Ok(())

}

pub fn process_historical_price(asset: &str, entry_dt: &Option<NaiveDateTime>) -> Result<(), Box<dyn Error>> {
    println!("\nINPUT:\n Asset: {:?}\n DateTime: {:?}", &asset, entry_dt.unwrap());
        
    let response = hist_api::coingecko_get(hist_api::build_url(asset, &(entry_dt).unwrap()))?; // Why no need to include "&" before asset
    
    let (result_dt, price) = hist_api::parse_api_response(response)?;

    println!("\nOUTPUT:\n Date: {:?}\n Price: {:?}", &result_dt, &price);
    
    Ok(())

}
