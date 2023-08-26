use chrono::{NaiveDate, NaiveDateTime};
use std::error::Error;

use crate::funcs::asset;

pub fn get_asset_string() -> String {
    println!("Please select asset or enter Coingecko API id. Type Q to Quit.\n");
    asset::Asset::display_enum_options();

    let input = get_io_input();

    // Quit program
    if input == "Q" {
        std::process::exit(1);
    }

    // Check for valid intger or return input string
    match input.parse::<usize>() {
        Ok(input) => match asset::Asset::match_enum(input) {
            Some(response) => format!("{:?}", response),
            None => panic!("Provided integer not found"),
        },
        Err(_) => input, // Return input if it is not an integer for use in API
    }
}

pub fn get_datetime() -> Option<NaiveDateTime> {
    loop {
        println!(
            "Press Return for current price. Or enter DateTime in a below format. 'Q' to Quit. \n
             YYYY-MM-DDTHH:MM:SS.###Z | YYYY-MM-DDTHH:MM:SSZ | YYYY-MM-DDTHH:MM:SS | YYYY-MM-DD HH:MM:SS | YYYY-MM-DD"
        );
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
                println!(
                    "\nError found on input, {}. Please try again, or enter Q to quit\n",
                    &input
                );
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
        return Ok(NaiveDateTime::parse_from_str(
            &format!("{} 00:00:00", datetime.trim()),
            "%Y-%m-%d %H:%M:%S",
        )?);
    }

    for fmt in common_formats.iter() {
        match NaiveDateTime::parse_from_str(datetime, fmt) {
            Ok(dt) => return Ok(dt),
            Err(_) => continue,
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
