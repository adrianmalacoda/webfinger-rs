use url::percent_encoding::{utf8_percent_encode, FORM_URLENCODED_ENCODE_SET};

// Helper functions
pub fn get_https_url(hostname: &str, resource: &str) -> String {
    get_url(hostname, resource, "https")
}

pub fn get_http_url(hostname: &str, resource: &str) -> String {
    get_url(hostname, resource, "http")
}

fn get_url(hostname: &str, resource: &str, schema: &str) -> String {
    format!("{}://{}/.well-known/webfinger?resource={}",
            schema,
            hostname,
            utf8_percent_encode(resource, FORM_URLENCODED_ENCODE_SET))
}


#[test]
fn test_simple_url_composition() {
    // HTTP
    assert_eq!("http://example/.well-known/webfinger?resource=sample",
               get_http_url("example", "sample"));

    assert_eq!("https://example/.well-known/webfinger?resource=sample",
               get_https_url("example", "sample"));
}

#[test]
fn test_encoded_url_composition() {
    // HTTP
    assert_eq!("http://example/.well-known/webfinger?resource=sample%20%26%20data",
               get_http_url("example", "sample & data"));

    assert_eq!("https://example/.well-known/webfinger?resource=sample%20%26%20data",
               get_https_url("example", "sample & data"));
}
