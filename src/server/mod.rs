pub mod options;

use config::{Config, File};
use self::options::Options;

pub struct Server<'a> {
    // pub options: Options,
    config: &'a Config
}

impl<'a> Server<'a> {
    pub fn new(config: &'a Config) -> Self {
        Server {
            // options: Options { audio_busses: 64 },
            config: config
        }
    }

    pub fn say_hello(&self) {
        let server_path = self.config
            .get::<String>("server.path")
            .expect("server.path isn't specified in config");
        println!("Hello Server, {}", server_path);
    }

    pub fn boot(&self) {

    }

    pub fn reboot(&self) {

    }

    pub fn shutdown(&self) {

    }
}

