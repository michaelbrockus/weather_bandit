use ::*;
use std::collections::HashMap;

/// Generic URI builder that handles all URI-related stuff.
pub struct UriBuilder<'a> {
    api_ver: &'a str,
    method: &'a str,
    params: HashMap<&'a str, String>,
}

/// Implemented by basically every query builder in the crate.
pub trait HasBuilder<'a> {
    fn builder(&mut self) -> &mut UriBuilder<'a>;
}

impl<'a> UriBuilder<'a> {
    pub fn new() -> Self {
        UriBuilder {
            api_ver: "2.5",
            method: "",
            params: HashMap::with_capacity(10),
        }
    }

    /// Set the endpoint method.
    pub fn method(&mut self, method: &'a str) -> &mut Self {
        self.method = method;
        self
    }

    /// Add param to the URI.
    pub fn param(&mut self, key: &'a str, val: String) -> &mut Self {
        self.params.insert(key, val);
        self
    }

    /// Consumes the builder and returns the corresponding URI.
    pub fn build(&mut self) -> String {
        let base = format!("http://api.openweathermap.org/data/{api}/{method}",
                           api = self.api_ver,
                           method = self.method);
        let mut ser = url::form_urlencoded::Serializer::new(String::new());

        match self.params.len() {
            0 => base,
            _ => {
                for (k, v) in self.params.iter() {
                    ser.append_pair(k, v.as_str());
                }
                base + "?" + ser.finish().as_str()
            }            
        }
    }
}
