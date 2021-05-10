//! # Dogehouse-api
//! This is a work-in-progress crate to interact with the dogehouse API.
//! Note that the API's documentation isn't complete yet, and so is the API itself.
//! This uses reqwest and serde-json. I don't know if libraries should use something different from
//! reqwest. If so, please file an issue on GitHub.
//! ## Current features:
//! - Get a list of popular rooms
//! - Get a list of upcoming rooms
//! - Get a list of bots
//! - Get basic statistics
//! - Get individual users by username.
//! ## Caveats: 
//! - The API is changing by the minute at the current stage, and this can break this crate.
//! - JavaScript and by that JSON uses 64-bit numbers, so all numbers in this crate have to be
//! either u64's or i64's
//! ## Usage:
//! This crate consists of Endpoint's and Query's.
//! Endpoints are just the simple API endpoints, which does not take any arguments.
//! Queries are API endpoints, which take one or more arguments, such as "/search/some_username".

#![deny(missing_docs)]
#![feature(format_args_capture)]
/// Defines a bunch of Endpoint implementing structs, which do not take any arguments.
pub mod endpoints;
mod util;
/// Defines Query implementing structs, which do take one or more arguments.
pub mod queries;
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

// This is free and unencumbered software released into the public domain.
// 
// Anyone is free to copy, modify, publish, use, compile, sell, or
// distribute this software, either in source code form or as a compiled
// binary, for any purpose, commercial or non-commercial, and by any
// means.
// 
// In jurisdictions that recognize copyright laws, the author or authors
// of this software dedicate any and all copyright interest in the
// software to the public domain. We make this dedication for the benefit
// of the public at large and to the detriment of our heirs and
// successors. We intend this dedication to be an overt act of
// relinquishment in perpetuity of all present and future rights to this
// software under copyright law.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
// 
// For more information, please refer to <http://unlicense.org/>
