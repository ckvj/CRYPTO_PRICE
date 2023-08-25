use std::error::Error;
use chrono::{NaiveDateTime, NaiveDate};

mod config;
mod hist_api;
mod current_api;
mod current_api2;

fn main() -> Result<(), Box<dyn Error>> {
    
    let asset: String = get_asset_string();
    let entry_dt =  get_datetime();

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



fn get_asset_string() -> String {

    println!("Please select asset or enter Coingecko API id. Type Q to Quit.\n");
    config::Asset::display_enum_options();

    let input = get_io_input();
    
    
    // Quit program
    if input == "Q" {
        std::process::exit(1);
    }

    match input.parse::<usize>() {
        Ok(input) => {
            match config::Asset::get_enum(input) {
                Some(response) => format!("{:?}",response),
                None => panic!("Provided integer not found")
            }
        }
        Err(_) => input // Return input if it is not an integer for use in API
    }
}

fn get_datetime() -> Option<NaiveDateTime> {

    loop {
        println!("Press Return for current price or enter DateTime in format YYYY-MM-DDTHH:MM:SS.###Z ");
        let input = get_io_input();

        if input.is_empty() {
            return None; // If no user input it means they want current time
        }

        if input == "Q" {
            std::process::exit(1);
        }

        match parse_datetime_string(input.trim()) {
            Ok(dt) => return Some(dt),
            Err(_) => {
                println!("\nError found on input, {}\nPlease try again, or enter Q to quit", &input);
            }
        };
    }
}


fn parse_datetime_string(datetime: &str) -> Result<NaiveDateTime, Box<dyn Error>> {
    // Common Date Formats
    let common_formats = [
        "%Y-%m-%dT%H:%M:%S%.3fZ",
        "%Y-%m-%dT%H:%M:%SZ",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M:%S",
    ];
    
    // Check first if Date vs DateTime and convert if so
    if NaiveDate::parse_from_str(datetime, "%Y-%m-%d").is_ok() {
        return Ok(NaiveDateTime::parse_from_str(&format!("{} 00:00:00",datetime.trim()), "%Y-%m-%d %H:%M:%S")?)
    }

    for fmt in common_formats.iter() {
        match NaiveDateTime::parse_from_str(datetime, fmt) {
            Ok(dt) => return Ok(dt),
            Err(_) => continue
            }
        }

    Err(format!("Unable to parse datetime: {}", datetime).into())
}


/// Retrieve user input and trim white space
fn get_io_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
