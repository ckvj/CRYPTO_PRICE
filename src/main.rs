use std::error::Error;

mod funcs;
use funcs::io_;
use funcs::price_processor;

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        let asset: String = match io_::get_asset_string() {
            Err(err) => eprintln!("Error: {}", err),
            Ok(asset) => asset,
        };

        match io_::get_datetime() {
            None => price_processor::process_current_price(&asset)?,
            Some(entry_dt) => price_processor::process_historical_price(&asset, &entry_dt)?,
        }

        // Request if user wants to retrieve another price or exit.
        io_::check_for_repeat()?;
    }
}
