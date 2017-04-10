use url::Url;
use url::ParseError;
use url::form_urlencoded;

use std::str::FromStr;

// Helper functions
pub fn get_https_url(hostname: &str, resource: &str) -> String {
    get_url(hostname, resource, "https")
}

pub fn get_http_url(hostname: &str, resource: &str) -> String {
    get_url(hostname, resource, "http")
}

pub fn get_hostname(url_str: &str) -> Result<String, ParseError> {
    match Url::from_str(url_str) {
        Ok(url) => {
            url.host().map(|host| format!("{}", host)).ok_or_else(|| ParseError::EmptyHost)
        },
        Err(err) => {
            if !url_str.starts_with("acct:") {
                return get_hostname(&format!("acct:{}", url_str));
            }

            Result::Err(err)
        }
    }
}

fn get_url(hostname: &str, resource: &str, schema: &str) -> String {
    let query: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("resource", resource)
        .finish()
        .replace("+", "%20"); // TODO: Is this necessary?

    format!("{}://{}/.well-known/webfinger?{}", schema, hostname, query)
}

#[test]
fn test_simple_url_composition() {
    assert_eq!("http://example/.well-known/webfinger?resource=sample",
               get_http_url("example", "sample"));

    assert_eq!("https://example/.well-known/webfinger?resource=sample",
               get_https_url("example", "sample"));
}

#[test]
fn test_encoded_url_composition() {
    assert_eq!("http://example/.well-known/webfinger?resource=sample%20%26%20data",
               get_http_url("example", "sample & data"));

    assert_eq!("https://example/.well-known/webfinger?resource=sample%20%26%20data",
               get_https_url("example", "sample & data"));
}
