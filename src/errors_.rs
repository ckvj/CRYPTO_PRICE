use thiserror::Error;

#[derive(Error, Debug)]
pub enum IoError {
    #[error("ERROR: invalid integer, could not match integer to asset")]
    InvalidInteger,
    #[error("ERROR: Asset input cannot be empty")]
    EmptyInput,
    #[error("ERROR: invalid input")]
    InvalidInput,
}


#[derive(Error, Debug)]
pub enum DateTimeError {
    #[error("ERROR: Unable to parse datetime")]
    ParseError,
    #[error("Unable to conver unix datetime into NaiveDateTime")]
    ConvertError
}

#[derive(Error, Debug)]
pub enum ApiResponseParseError {
    #[error("ERROR: Received empty API response. Likely invalid coingecko ID")]
    Empty,

    #[error("ERROR: More than one price value found in API response")]
    MultiPrice,

    #[error("ERROR: No price data found, input datetime may be before price data available")]
    EmptyPrice,

    #[error("ERROR: Error processing price data found")]
    PriceParseError

}