pub mod options;
use self::options::Options;
use std::process::{Command, Stdio};
use std::process::Child;

pub struct Server {
    pub options: Options,
    pub process: Option<Child>,
}

impl Server {
    pub fn new(options: Options) -> Self {
        Server {
            options: options,
            process: None
        }
    }

    pub fn boot(&mut self) {
        if self.process.is_some() {
            return println!("SuperCollider server is already running.");
        }

        self.process = match Command::new(self.options.path.clone())
            .args(&self.options.to_args())
            // .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn() {
                Err(e) => panic!("couldn't spawn {}: {}", self.options.path, e),
                Ok(process) => Some(process),
            }
    }

    pub fn reboot(&mut self) {
        self.shutdown();
        self.boot();
    }

    pub fn shutdown(&mut self) {
        if self.process.is_some() {
            self.process.as_mut().unwrap().kill().unwrap();
            self.process = None;
        }
    }
}

