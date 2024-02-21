use std::error::Error;

mod funcs;
use funcs::helpers::strings_;
use funcs::io_;
use funcs::price_processor;

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        println!("{}", strings_::TOP_OF_PROGRAM);

        let asset: String = match io_::get_asset_string() {
            Err(err) => {
                println!("{}", err);
                println!("{}", strings_::TRY_AGAIN);
                continue; // Loop to top of program
            }
            Ok(asset) => asset,
        };

        match io_::get_datetime() {
            None => match price_processor::process_and_display_current_price(&asset) {
                Ok(()) => (),
                Err(_) => continue,
            },
            Some(entry_dt) => match price_processor::process_historical_price(&asset, &entry_dt) {
                Ok(()) => (),
                Err(_) => continue,
            },
        }

        // Request if user wants to retrieve another price or exit.
        match io_::check_for_repeat() {
            Ok(()) => (),
            Err(_) => continue,
        };
    }
}
