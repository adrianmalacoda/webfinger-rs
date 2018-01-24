use url::Url;
use url::ParseError;
use url::form_urlencoded;

use std::str::FromStr;

#[derive(Debug)]
pub struct ResourceUrl {
    pub scheme: Option<String>,
    pub host: Option<String>,
    pub user: Option<String>
}

impl ResourceUrl {
    pub fn new(url_str: &str) -> Result<ResourceUrl, ParseError> {
        let mut owned_url_str = url_str.to_owned();
        if !owned_url_str.starts_with("mailto://") && owned_url_str.starts_with("mailto:") {
            owned_url_str = format!("mailto://{}", owned_url_str.trim_left_matches("mailto:"));
        }
        if !owned_url_str.starts_with("acct://") && owned_url_str.starts_with("acct:") {
            owned_url_str = format!("acct://{}", owned_url_str.trim_left_matches("acct:"));
        }

        if !owned_url_str.contains("://") {
            Ok(ResourceUrl::from_username_host(&owned_url_str))
        } else {
            match Url::from_str(&owned_url_str) {
                Ok(url) => Ok(ResourceUrl::from_parsed_url(url)),
                Err(err) => Result::Err(err)
            }
        }
    }

    fn from_parsed_url(url: Url) -> ResourceUrl {
        ResourceUrl {
            scheme: Some(url.scheme().to_owned()),
            host: url.host_str().map(|host_str| {
                let mut host = host_str.to_owned();
                if let Some(port) = url.port() {
                    host = format!("{}:{}", host, port);
                }
                host
            }),
            user: if url.username().is_empty() { None } else { Some(url.username().to_owned()) }
        }
    }

    fn from_username_host(username_host: &str) -> ResourceUrl {
        username_host.find("@").map(|index| {
            let (username, hostname) = username_host.split_at(index);
            ResourceUrl {
                scheme: None,
                host: Some(hostname.trim_left_matches("@").to_owned()),
                user: Some(username.to_owned())
            }
        }).unwrap_or_else(|| {
            ResourceUrl {
                scheme: None,
                host: None,
                user: Some(username_host.to_owned())
            }
        })
    }
}

// Helper functions
pub fn get_https_url(hostname: &str, resource: &str) -> String {
    get_url(hostname, resource, "https")
}

pub fn get_http_url(hostname: &str, resource: &str) -> String {
    get_url(hostname, resource, "http")
}

pub fn get_hostname(url_str: &str) -> Result<Option<String>, ParseError> {
    ResourceUrl::new(url_str).map(|url| url.host)
}

fn get_url(hostname: &str, resource: &str, schema: &str) -> String {
    let query: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("resource", resource)
        .finish()
        .replace("+", "%20"); // TODO: Is this necessary?

    format!("{}://{}/.well-known/webfinger?{}", schema, hostname, query)
}


#[test]
fn test_resource_url() {
    let http_example_user = ResourceUrl::new("http://example.com/~user");
    let acct_bob_example = ResourceUrl::new("acct:bob@example.com");
    let fred_example = ResourceUrl::new("fred@example.com");
    let mailto_joe_example = ResourceUrl::new("mailto:joe@example.com");
    let https_steve_example = ResourceUrl::new("https://steve.example.com/");
    let https_localhost_dennis = ResourceUrl::new("https://localhost:8002/users/dennis");
    let acct_dennis_localhost = ResourceUrl::new("acct:dennis@localhost:8002");

    println!("{:?}", http_example_user);
    println!("{:?}", acct_bob_example);
    println!("{:?}", fred_example);
    println!("{:?}", mailto_joe_example);
    println!("{:?}", https_steve_example);
    println!("{:?}", https_localhost_dennis);
    println!("{:?}", acct_dennis_localhost);

    assert_eq!(Some("example.com".to_owned()), http_example_user.unwrap().host);
    assert_eq!(Some("example.com".to_owned()), acct_bob_example.unwrap().host);
    assert_eq!(Some("example.com".to_owned()), fred_example.unwrap().host);
    assert_eq!(Some("example.com".to_owned()), mailto_joe_example.unwrap().host);
    assert_eq!(Some("steve.example.com".to_owned()), https_steve_example.unwrap().host);
    assert_eq!(Some("localhost:8002".to_owned()), https_localhost_dennis.unwrap().host);
    assert_eq!(Some("localhost:8002".to_owned()), acct_dennis_localhost.unwrap().host);
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
