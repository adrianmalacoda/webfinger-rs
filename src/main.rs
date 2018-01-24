extern crate webfinger;

#[macro_use]
extern crate log;
extern crate env_logger;

use webfinger::resource::resource;
use webfinger::client::client::get_by_https;
use webfinger::client::urlbuilder::get_hostname;
use std::env;

fn init_logger() {
    let mut builder = env_logger::Builder::new();
    builder.filter(None, log::LevelFilter::Info);

    if env::var("RUST_LOG").is_ok() {
       builder.parse(&env::var("RUST_LOG").unwrap());
    }

    builder.init();
}

fn main() {
    init_logger();

    let identifier = env::args().nth(1).expect("please supply an identifier");
    let hostname = env::args().nth(2).unwrap_or_else(|| {
        get_hostname(&identifier).expect("Failed to parse hostname from identifier")
                                 .expect("Identifier does not have hostname and no hostname was provided")
    });

    info!("Running query for identifier {} against hostname {}", identifier, hostname);
    let resource_json = get_by_https(&hostname, &identifier);
    debug!("{}", resource_json);

    let resource = resource::from_json(&resource_json);
    println!("{}", resource);
}
