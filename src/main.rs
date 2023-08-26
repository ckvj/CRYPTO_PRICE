use std::error::Error;

mod asset;
mod current_api;
mod current_api2;
mod hist_api;

mod io;
mod price_processor;

fn main() -> Result<(), Box<dyn Error>> {
    let asset: String = io::get_asset_string();
    let entry_dt = io::get_datetime();

    match entry_dt {
        // Get current price (user did not enter historical date)
        None => price_processor::process_current_price(&asset)?,
        // Fetch historical price
        Some(_) =>  price_processor::process_historical_price(&asset, &entry_dt)?
    }

    Ok(())
}
