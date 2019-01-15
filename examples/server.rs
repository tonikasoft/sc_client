extern crate sc_client;
extern crate env_logger;

use sc_client::server::{Server, Options};
use std::thread;
use std::time::Duration;
use std::env;

fn main() {
    env::set_var("RUST_LOG", "sc_client=debug");
    env_logger::init();

    let options = Options::new("examples/settings.toml");
    let mut server = Server::new(options);
    server.boot();

    thread::sleep(Duration::from_secs(5));

    server.reboot();

    loop{std::thread::sleep(Duration::from_millis(1))}
}
