use std::error::Error;

mod funcs;
use funcs::io_;
mod output_messages;
mod errors_;
use funcs::price_processor;
use output_messages as msg;


fn main() -> Result<(), Box<dyn Error>> {
    loop {
        println!("{}", msg::TOP_OF_PROGRAM);

        let asset: String = match io_::get_asset_string() {
            Err(err) => {
                println!("{}", err);
                println!("{}", msg::TRY_AGAIN);
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
        match io_::check_for_repeat(){
            Ok(()) => (),
            Err(_) => continue,
        };
    }
}
