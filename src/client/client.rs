use std::borrow::Borrow;
use std::io::Read;

use reqwest;

use client::urlbuilder;

pub fn get_by_http(hostname: &str, resource: &str) -> String {
    let url = urlbuilder::get_http_url(hostname, resource);
    debug!("Performing HTTP query using URL {}", url);
    get(url.borrow())
}

pub fn get_by_https(hostname: &str, resource: &str) -> String {
    let url = urlbuilder::get_https_url(hostname, resource);
    debug!("Performing HTTPS query using URL {}", url);
    get(url.borrow())
}


fn get(url: &str) -> String {
    let mut res = reqwest::get(url).unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    body
}
