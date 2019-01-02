pub mod options;

use config::{Config, File};
use self::options::Options;

pub struct Server<'a> {
    pub options: Options,
    config: &'a Config
}

impl<'a> Server<'a> {
    pub fn new(config: &'a Config) -> Self {
        let options = Options::new(config);
        Server {
            options: options,
            config: config
        }
    }

    pub fn say_hello(&self) {
        println!("Hello Server, {}", self.options.path);
    }

    pub fn boot(&self) {

    }

    pub fn reboot(&self) {

    }

    pub fn shutdown(&self) {

    }
}

