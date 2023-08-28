use chrono::{NaiveDate, NaiveDateTime};
// use std::error::Error;
use std::fmt::Debug;
use thiserror::Error;
use crate::funcs::asset;

#[derive(Error, Debug)]
pub enum IoError {
    #[error("invalid integer, could not match integer to asset")]
    InvalidInteger,
    #[error("asset input cannot be empty")]
    EmptyInput,
    #[error("invalid input")]
    InvalidInput,
}


#[derive(Error, Debug)]
pub enum DateTimeError {
    #[error("Unable to parse datetime")]
    ParseError
}

pub fn get_asset_string() -> Result<String, IoError> {
    println!("Please select asset or enter Coingecko API id. Type Q to Quit.\n");
    asset::Asset::display_enum_options();

    let input = match get_io_input() {
        None => return Err(IoError::EmptyInput),
        Some(input) => input,
    };

    // Check for valid intger or return input string
    match input.parse::<usize>() {
        Ok(input) => match asset::Asset::match_enum(input) {
            Some(response) => Ok(format!("{:?}", response)),
            None => Err(IoError::InvalidInteger),
        },
        Err(_) => Ok(input), // Return input if it is not an integer for use in API
    }
}


pub fn get_datetime() -> Option<NaiveDateTime> {
    loop {
        println!(
            "Press Return for current price. Or enter DateTime in a below format. 'Q' to Quit. \n
             YYYY-MM-DDTHH:MM:SS.###Z | YYYY-MM-DDTHH:MM:SSZ | YYYY-MM-DDTHH:MM:SS | YYYY-MM-DD HH:MM:SS | YYYY-MM-DD"
        );
        
        let input = match get_io_input() {
            None => return None,
            Some(input) => input,
        };

        match parse_datetime_string(input.trim()) {
            Ok(dt) => return Some(dt),
            Err(_) => {
                eprintln!(
                    "\nError found on input, {}. Please try again, or enter Q to quit\n",
                    &input
                );
            }
        };
    }
}

fn parse_datetime_string(datetime: &str) -> Result<NaiveDateTime, DateTimeError> {
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
        ).unwrap());
    }

    for fmt in common_formats.iter() {
        match NaiveDateTime::parse_from_str(datetime, fmt) {
            Ok(dt) => return Ok(dt),
            Err(_) => continue,
        }
    }

    Err(DateTimeError::ParseError)
}


pub fn check_for_repeat() -> Result<(), IoError> {
    
    println!("\nPress Enter (or Return) to get another price or type Q to exit\n");
    
    match get_io_input() {
        None => Ok(()),
        Some(_) => Err(IoError::InvalidInput)
    }
}


/// Retrieve user input. Checks for quit and returns Some(value) or None if empty 
fn get_io_input() -> Option<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_string();

    check_for_quit_and_quit(&input);

    match check_for_empty(&input) {
        true => None,
        false => Some(input)
    }

}

fn check_for_empty(input: &str) -> bool {
    input.is_empty()
}

fn check_for_quit_and_quit(input: &str) {
    if input == "Q" {
        std::process::exit(1);
    }
}