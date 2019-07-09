// Rust JSON-RPC 1.0 Library
// Written in 2015 by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
//
// Modified in 2016 by
//     Jean Pierre De Jesus Dudey Diaz <jeandudey@hotmail.com>
//
// Modified in 2016 by
//     Aleksey Sidorov <aleksei.sidorov@xdev.re>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Error handling
//!
//! Some useful methods for creating Error objects
//!

use reqwest;
use serde_json;

/// A library error
#[derive(Debug, Fail)]
pub enum Error {
    /// Json decoding error.
    #[fail(display = "Json decoding error. {}", _0)]
    Json(serde_json::Error),
    /// Client error
    #[fail(display = "Client error. {}", _0)]
    Client(reqwest::Error),
    /// Rpc error,
    #[fail(display = "Rpc error. {}", _0)]
    Rpc(serde_json::Value),
    /// Response has neither error nor result.
    #[fail(display = "Response has neither error nor result")]
    NoErrorOrResult,
    /// Response to a request did not have the expected nonce
    #[fail(display = "Response to a request did not have the expected nonce")]
    NonceMismatch,
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Json(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Client(e)
    }
}