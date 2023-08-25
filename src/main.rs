use std::error::Error;

mod config;
mod hist_api;
mod current_api;
mod current_api2;
mod io;

fn main() -> Result<(), Box<dyn Error>> {
    
    let asset: String = io::get_asset_string();
    let entry_dt =  io::get_datetime();

    if entry_dt.is_none() {
        println!("\nINPUT:\n Asset: {:?}\n Date: Now", &asset);
        
        let response = current_api::coingecko_get(current_api::build_url(&asset))?;
        let (price, change_24hr) = current_api::parse_api_response(response)?;
        
        println!("\nOUTPUT:\n Price: {}\n 24hr Change (%): {}", &price, &change_24hr);

    } else {
        println!("\nINPUT:\n Asset: {:?}\n DateTime: {:?}", &asset, entry_dt.unwrap());
        
        let response = hist_api::coingecko_get(hist_api::build_url(&asset, &(entry_dt).unwrap()))?;
        let (result_dt, price) = hist_api::parse_api_response(response)?;
        
        println!("\nOUTPUT:\n Date: {:?}\n Price: {:?}", &result_dt, &price);
    } 
    
    Ok(())
}
