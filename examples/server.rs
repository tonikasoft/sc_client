extern crate sc_client;

use sc_client::server::{Server, options::Options};
use std::thread;
use std::time::Duration;

fn main() {
    let options = Options::new("examples/settings.toml");
    let mut server = Server::new(options);
    server.boot();

    thread::sleep(Duration::from_secs(5));

    server.reboot();

    loop{}
}
