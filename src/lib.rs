//
// file: program.rs
// author: Michael Brockus
// gmail: <michaelbrockus@gmail.com>
//
mod uri;
pub mod data;
pub mod current;

#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate serde;
extern crate serde_json as json;
extern crate url;

use std::io::Read;
use data::*;
use current::*;

#[derive(Debug)]
pub enum Error {
    /// An error occurred while performing the HTTP request.
    HttpError(hyper::Error),

    /// The request was not correctly understood by the server. Details included.
    BadRequest(ErrorResponse),

    /// Invalid JSON received from the server, likely caused by an API change.
    JsonDecodeError(String, json::Error),

    /// Indicates an HTTP repsonse with a non-success status code.
    Failure(hyper::client::Response),
}

/// A universal result type used as return for all calls.
pub type Result<T> = std::result::Result<T, Error>;

/// Central hub to access all weather-related facilities.
pub struct WeatherHub {
    client: hyper::Client,
    key: String,
}

impl<'a> WeatherHub {
    /// Creates a new WeatherHub which will use the provided client to perform
    /// its requests. It also requires an OWM API key.
    pub fn new(client: hyper::Client, key: &str) -> WeatherHub {
        WeatherHub {
            client: client,
            key: key.to_string(),
        }
    }

    /// Provides access to the current-weather facilities.
    pub fn current(&'a self) -> CurrentWeatherQuery<'a> {
        CurrentWeatherQuery::new(&self, {
            let mut ub = uri::UriBuilder::new();
            ub.param("appid", self.key.clone());
            ub
        })
    }

    /// Does the actual API call, parses the response and handles any errors.
    fn run_query<D>(&'a self, query: String) -> Result<(hyper::client::Response, D)>
        where D: serde::Deserialize
    {
        let req_result = self.client.request(hyper::method::Method::Get, &query).send();

        match req_result {
            Err(err) => return Err(Error::HttpError(err)),
            Ok(mut res) => {
                if !res.status.is_success() {
                    let mut json_err = String::new();
                    res.read_to_string(&mut json_err).unwrap();
                    return match json::from_str::<ErrorResponse>(&json_err) {
                               Ok(serr) => Err(Error::BadRequest(serr)),
                               Err(_) => Err(Error::Failure(res)),
                           };
                }
                let mut json_resp = String::new();
                res.read_to_string(&mut json_resp).unwrap();
                return match json::from_str(&json_resp) {
                           Ok(decoded) => Ok((res, decoded)),
                           Err(err) => Err(Error::JsonDecodeError(json_resp, err)),
                       };
            }
        }
    }
}

/// Rectangle specified by geographic coordinates (latitude and longitude).
#[derive(Debug)]
pub struct BoundingBox {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

/// Units format for this query.
#[derive(Debug, Serialize, Deserialize)]
pub enum Units {
    Metric,
    Imperial,
}

impl ToString for Units {
    fn to_string(&self) -> String {
        match self {
            &Units::Metric => "metric".to_string(),
            &Units::Imperial => "imperial".to_string(),
        }
    }
}

pub trait FormatResponse<'a>
    where Self: std::marker::Sized + uri::HasBuilder<'a>
{
    /// Change units format for the query. Default is Standard.
    fn units(mut self, units: Units) -> Self {
        self.builder().param("units", units.to_string());
        self
    }

    /// Change language for the query. Note that only the `description` field
    /// of [Weather](struct.Weather.html) is translated.
    fn lang(mut self, lang: &str) -> Self {
        self.builder().param("lang", lang.to_string());
        self
    }
}
	
//
// Greet the user
//
pub fn greet() -> &'static str
{
    "Hello, Rust Developer."
}

//
// foundation of the program and related
// application logic must be implemented
// in the foundation.
//
pub fn foundation()
{
    println!("{}", greet());
} // end of function foundation
