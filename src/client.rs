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

//! # Client support
//!
//! Support for connecting to JSON-RPC 1.0 servers over HTTP, sending requests,
//! and parsing responses
//!

use std::sync::{Arc, Mutex};

use reqwest;
use serde_json::Value;

use super::{Request, Response};
use error::Error;

/// A handle to a remote JSONRPC server
pub struct Client {
    url: String,
    user: Option<String>,
    pass: Option<String>,
    client: reqwest::Client,
    nonce: Arc<Mutex<u64>>,
}

impl Client {
    /// Creates a new client
    ///
    /// # Examples
    /// ```
    /// use exonum_jsonrpc::client::Client;
    ///
    /// let client = Client::new(String::from("www.example.org"), None, None);
    /// ```
    ///
    /// # Panics
    /// This function panics if you provide a password without an username.
    pub fn new<S: Into<String>>(url: S, user: Option<S>, pass: Option<S>) -> Client {
        let url = url.into();
        let user = user.map(Into::into);
        let pass = pass.map(Into::into);
        // Check that if we have a password, we have a username; other way around is ok
        debug_assert!(pass.is_none() || user.is_some());

        Client {
            url,
            user,
            pass,
            client: reqwest::Client::new(),
            nonce: Arc::new(Mutex::new(0)),
        }
    }

    /// Sends a request to a client
    pub fn send_request(&self, request: &Request) -> Result<Response, Error> {
        // Send request
        let mut request_builder = self.client.post(&self.url);
        if let Some(ref user) = self.user {
            request_builder = request_builder.basic_auth(user.clone(), self.pass.clone());
        }
        let mut reqwest_response = request_builder.json(request).send()?;
        // Parse response
        let response: Response = reqwest_response.json()?;
        if response.id != request.id {
            return Err(Error::NonceMismatch);
        }
        Ok(response)
    }

    /// Builds a request
    pub fn build_request(&self, name: String, params: Vec<Value>) -> Request {
        let mut nonce = self.nonce.lock().unwrap();
        *nonce += 1;
        Request {
            method: name,
            params,
            id: json!(*nonce),
        }
    }

    /// Accessor for the last-used nonce
    pub fn last_nonce(&self) -> u64 {
        *self.nonce.lock().unwrap()
    }

    /// Returns rpc url
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    /// Returns rpc password
    pub fn password(&self) -> &Option<String> {
        &self.pass
    }

    /// Returns rpc username
    pub fn username(&self) -> &Option<String> {
        &self.user
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let client = Client::new("localhost".to_owned(), None, None);
        assert_eq!(client.last_nonce(), 0);
        let req1 = client.build_request("test".to_owned(), vec![]);
        assert_eq!(client.last_nonce(), 1);
        let req2 = client.build_request("test".to_owned(), vec![]);
        assert_eq!(client.last_nonce(), 2);
        assert!(req1 != req2);
    }
}
