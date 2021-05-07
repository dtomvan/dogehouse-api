#![feature(format_args_capture)]
pub mod endpoints;
mod util;
#[cfg(test)]
mod test;

#[derive(Debug)]
/// Error type for any error that could occur after sending a request to the Dogehouse API.
pub enum ErrorType {
    /// Constructed when the API Endpoint didn't return the `200` status code. Contains the
    /// returned status code by the API.
    NotFound(u16),
    /// Constructed when the API didn't return valid JSON, or without valid fields.
    /// Contains serde_json's error.
    ParseError(serde_json::Error),
    /// Constructed when for some reason reqwest's get() doesn't return an Ok() value.
    NoHTTP,
}
