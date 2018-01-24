extern crate url;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate log;

#[macro_use]
extern crate derive_error;

pub mod client;
pub mod resource;

#[derive(Debug, Error)]
pub enum WebFingerError {
    /// Error from the HTTP client
    Client(client::client::ClientError),
    /// Parse error
    Parse(serde_json::Error)
}

pub fn get_from_host(hostname: &str, resource: &str) -> Result<resource::resource::Resource, WebFingerError> {
    Ok(serde_json::from_str(&client::client::get_by_https(hostname, resource)?)?)
}

pub fn get(resource: &str) -> Result<resource::resource::Resource, WebFingerError> {
    let hostname = client::urlbuilder::get_hostname(resource).unwrap().unwrap();
    get_from_host(&hostname, resource)
}
