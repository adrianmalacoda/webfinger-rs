extern crate webfinger;

use webfinger::client::client::get_by_https;
use webfinger::client::urlbuilder::get_hostname;
use std::env;

fn main() {
    let identifier = env::args().nth(1).expect("please supply an identifier");
    let hostname = env::args().nth(2).unwrap_or_else(|| get_hostname(&identifier).expect("Failed to parse hostname from identifier"));

    println!("Running query for identifier {} against hostname {}", identifier, hostname);
    println!("{}", get_by_https(&hostname, &identifier));
}
