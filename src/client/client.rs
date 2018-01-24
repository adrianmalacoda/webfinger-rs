use std::borrow::Borrow;

use std::io;
use std::io::Read;

use reqwest;

use client::urlbuilder;

#[derive(Debug, Error)]
pub enum ClientError {
    /// HTTP request failed
    HTTP(reqwest::Error),
    /// IO error
    IO(io::Error)
}

pub fn get_by_http(hostname: &str, resource: &str) -> Result<String, ClientError> {
    let url = urlbuilder::get_http_url(hostname, resource);
    debug!("Performing HTTP query using URL {}", url);
    get(url.borrow())
}

pub fn get_by_https(hostname: &str, resource: &str) -> Result<String, ClientError> {
    let url = urlbuilder::get_https_url(hostname, resource);
    debug!("Performing HTTPS query using URL {}", url);
    get(url.borrow())
}


fn get(url: &str) -> Result<String, ClientError> {
    let mut res = reqwest::get(url)?;

    let mut body = String::new();
    res.read_to_string(&mut body)?;

    Ok(body)
}
