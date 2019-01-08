pub mod options;
use self::options::Options;
use std::process::{Command, Output, Stdio};
use std::thread;
use std::thread::JoinHandle;

pub struct Server {
    pub options: Options,
    process_join_handle: Option<JoinHandle<Output>>,
}

impl Server {
    pub fn new(options: Options) -> Self {
        Server {
            options: options,
            process_join_handle: None
        }
    }

    pub fn boot(&mut self) {
        if self.process_join_handle.is_some() {
            return println!("SuperCollider server is already running.");
        }

        // getting "Incorrect checksum for freed object" error with Arc here,
        // but simple clone fixes the issue
        let options = self.options.clone();

        self.process_join_handle = Some(thread::spawn(move || {
            match Command::new(options.path.clone())
                .args(&options.to_args())
                // .stdin(Stdio::piped())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output() {
                    Err(e) => panic!("couldn't start {}: {}", options.path, e),
                    Ok(process) => process,
                }
        }));
    }

    pub fn reboot(&mut self) {
        self.shutdown();
        self.boot();
    }

    pub fn shutdown(&mut self) {
        // TODO use server's `/quit` command
    }

    pub fn set_options_and_reboot(&mut self, opts: Options) {
        self.options = opts;
        self.reboot();
    }
}

