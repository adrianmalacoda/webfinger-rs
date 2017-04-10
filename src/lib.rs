extern crate url;
extern crate reqwest;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate log;

pub mod client;
pub mod resource;

pub fn get_from_host(hostname: &str, resource: &str) -> resource::resource::Resource {
    resource::resource::from_json(&client::client::get_by_https(hostname, resource))
}

pub fn get(resource: &str) -> resource::resource::Resource {
    let hostname = client::urlbuilder::get_hostname(resource).unwrap();
    get_from_host(&hostname, resource)
}
