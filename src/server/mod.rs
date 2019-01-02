pub mod options;
use self::options::Options;

pub struct Server {
    pub options: Options
}

impl Server {
    pub fn new(options: Options) -> Self {
        Server {
            options: options,
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

