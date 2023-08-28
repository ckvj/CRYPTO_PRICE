
use chrono::{NaiveDate, NaiveDateTime};
use crate::funcs::asset;
use crate::errors_::{DateTimeError, IoError};
use crate::output_messages as msg;

pub fn get_asset_string() -> Result<String, IoError> {
    println!("{}", msg::ASSET_INPUT_PROMPT);
    asset::Asset::display_enum_options();

    let input = match get_io_input() {
        None => return Err(IoError::EmptyInput),
        Some(input) => input,
    };

    // Check for valid intger or return input string
    match input.parse::<usize>() {
        Ok(input) => match asset::Asset::match_enum(input) {
            Some(response) => Ok(format!("{:?}", response)), // Return asset as string
            None => Err(IoError::InvalidInteger),
        },
        Err(_) => Ok(input), // Return string input if it is not an integer for use in API
    }
}


pub fn get_datetime() -> Option<NaiveDateTime> {
    loop {
        println!("{}", msg::DATETIME_INPUT_PROMPT);

        let input = match get_io_input() {
            None => return None,
            Some(input) => input,
        };

        match parse_datetime_string(input.trim()) {
            Ok(dt) => return Some(dt),
            Err(_) => {
                println!("{}. Input was: {}.", DateTimeError::ParseError, &input);
                println!("{}", msg::TRY_AGAIN_OR_QUIT);
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

    // Check first if Date vs DateTime and pad zeros if so
    if NaiveDate::parse_from_str(datetime, "%Y-%m-%d").is_ok() {
        return Ok(NaiveDateTime::parse_from_str(
            &format!("{} 00:00:00", datetime.trim()),
            "%Y-%m-%d %H:%M:%S",
        )
        .unwrap());
    }

    for fmt in common_formats.iter() {
        if let Ok(dt) = NaiveDateTime::parse_from_str(datetime, fmt) { 
            return Ok(dt)
        } 
    }
    
    Err(DateTimeError::ParseError)
}


pub fn check_for_repeat() -> Result<(), IoError> {
    println!("{}", msg::REPEAT_PROMPT);

    match get_io_input() {
        None => Ok(()),
        Some(_) => {
            println!("{}", IoError::InvalidInput);
            println!("Restarting program");
            Err(IoError::InvalidInput)
        },
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
        false => Some(input),
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
