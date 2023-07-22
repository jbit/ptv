#![doc = include_str!("../README.md")]

#[cfg(feature = "reqwest")]
mod reqwest;

#[cfg(test)]
mod test;

mod api {
    pub mod departures;
    pub mod search;
}
mod types;

pub use api::departures::*;
pub use api::search::*;

pub use types::*;

use hmac::{Hmac, Mac};
use http::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use time::OffsetDateTime;
use url::Url;

// Internal helper type aliases
type HmacSha1 = Hmac<Sha1>;
type Result<T> = std::result::Result<T, Error>;
type FutureResult<T> = Pin<Box<dyn Future<Output = Result<T>> + Send + 'static>>;

/// Base URL using HTTPS
const BASE_URL: &str = "https://timetableapi.ptv.vic.gov.au/";

/// Errors coming from this crate
#[derive(Debug, Clone)]
pub enum Error {
    /// Other error
    Other(String),
    /// Parse error
    JsonParseError(String),
    /// An error from the remote HTTP server
    HTTP(StatusCode, String),
    /// An error returned by the remote API endpoint
    API(StatusCode, ErrorResponse),
}
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::JsonParseError(error.to_string())
    }
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as std::fmt::Debug>::fmt(self, f)
    }
}

/// A trait for abstracting the HTTP client library easily.
///
/// See the `reqwest` module for the `reqwest` implementation of this trait
// TODO: It'd be nice to get rid of the boxed future, but `async` trait fns are nightly only right now
pub trait PTVHttpClient {
    fn api_get<T: DeserializeOwned + Send + 'static>(&self, url: Url) -> FutureResult<T>;
}

/// The main interface to the PTV API
pub struct PTV<HttpClient: PTVHttpClient> {
    http_client: HttpClient,
    devid: String,
    key: String,
    base: Url,
}

impl<Client: PTVHttpClient> PTV<Client> {
    /// Internal helper to build a complete endpoint URL given an API path and parameters
    fn build_url(&self, path: &str, params: impl Serialize) -> Url {
        let query = serde_html_form::to_string(&params).expect("Failed to serialize URL params");
        let mut url = self.base.join(path).expect("Failed to build URL");
        url.set_query(Some(&query));
        self.sign_url(url)
    }
    /// Internal helper to sign an endpoint URL using the supplied developer id and key
    fn sign_url(&self, mut url: Url) -> Url {
        // Add developer id to the request
        url.query_pairs_mut().append_pair("devid", &self.devid);

        // Sign the path and query part of the URL, with developer id
        let signature = HmacSha1::new_from_slice(self.key.as_bytes())
            .expect("Failed to initialize HMAC")
            .chain_update(url.path())
            .chain_update(b"?")
            .chain_update(url.query().unwrap_or_default())
            .finalize();

        // Convert it to a hex string
        let hex: String = signature
            .into_bytes()
            .iter()
            .map(|b| format!("{b:02X}"))
            .collect();

        // Append it to the URL
        url.query_pairs_mut().append_pair("signature", &hex);

        url
    }
}
