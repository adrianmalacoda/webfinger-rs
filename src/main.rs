extern crate webfinger;

#[macro_use]
extern crate log;
extern crate env_logger;

use webfinger::client::client::get_by_https;
use webfinger::client::urlbuilder::get_hostname;
use std::env;

fn init_logger() {
    let mut builder = env_logger::LogBuilder::new();
    builder.filter(None, log::LogLevelFilter::Info);

    if env::var("RUST_LOG").is_ok() {
       builder.parse(&env::var("RUST_LOG").unwrap());
    }

    builder.init().unwrap();
}

fn main() {
    init_logger();

    let identifier = env::args().nth(1).expect("please supply an identifier");
    let hostname = env::args().nth(2).unwrap_or_else(|| get_hostname(&identifier).expect("Failed to parse hostname from identifier"));

    info!("Running query for identifier {} against hostname {}", identifier, hostname);
    println!("{}", get_by_https(&hostname, &identifier));
}
