use std::error::Error;
use chrono::NaiveDateTime;

use crate::funcs::current_api;
use crate::funcs::hist_api;

use crate::output_messages as msg;

pub fn process_and_display_current_price(asset: &str) -> Result<(), Box<dyn Error>> {
    
    display_inputs(asset, None);
    let response = current_api::coingecko_get(current_api::build_url(asset))?;
    let (price, change_24hr) = current_api::parse_api_response(response)?;
    display_outputs(price, None, Some(change_24hr));    
    Ok(())
}

pub fn process_historical_price(asset: &str, entry_dt: &NaiveDateTime) -> Result<(), Box<dyn Error>> {
    
    display_inputs(asset, Some(entry_dt));
    let response = hist_api::coingecko_get(hist_api::build_url(asset, entry_dt))?;
    let (result_dt, price) = hist_api::parse_api_response(response)?;
    display_outputs(price, Some(&result_dt), None);    
    Ok(())

}

pub fn display_inputs(asset: &str, dt: Option<&NaiveDateTime>) {
    
    println!("\n{}", msg::INPUT_STR);
    // Asset
    println!(" {} {:?}", msg::ASSET_STR, &asset);
    // DateTime
    match dt {
        None => println!(" {}", msg::DATEIME_NOW_STR),
        Some(dt) => println!(" {} {:?}", msg::DATEIME_STR, dt.format("%Y-%m-%d %H:%M:%S").to_string()),
    };   
}


pub fn display_outputs(price: f64, dt: Option<&NaiveDateTime>, change_24: Option<f64>) {
    
    println!("\n{}", msg::OUTPUT_STR);
    // DateTime
    if dt.is_some() {println!(" {} {:?}", msg::DATEIME_STR, dt.unwrap().format("%Y-%m-%d %H:%M:%S").to_string());}; 
    // Price
    println!(" {}{}",msg::PRICE_STR, price );
    // 24hr Change
    if change_24.is_some() {
        println!(" {} {}", msg::CHANGE_24_STR, change_24.unwrap());
    }; 
}