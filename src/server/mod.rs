pub mod options;
use self::options::Options;
use std::process::{Command, Output, Stdio};
use std::thread;
use std::thread::JoinHandle;
use rosc::OscMessage;
use super::osc_handler::OscHandler;

pub struct Server {
    pub options: Options,
    process_join_handle: Option<JoinHandle<Output>>,
    pub osc_handler: OscHandler,
}

impl Server {
    pub fn new(options: Options) -> Self {
        let osc_handler = OscHandler::new(&options);

        Server {
            options: options,
            process_join_handle: None,
            osc_handler: osc_handler,
        }
    }

    pub fn boot(&mut self) {
        if self.process_join_handle.is_some() {
            return println!("SuperCollider server is already running.");
        }

        // get "Incorrect checksum for freed object" error when use Arc here,
        // but simple clone performs fine
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
        self.osc_handler.send_message(OscMessage {
            addr: "/quit".to_string(),
            args: None,
        });

        self.osc_handler.add_responder_for_address("/quit", |packet| {
            println!("quiting")
        });

        if let Some(handle) = self.process_join_handle.take() {
            handle.join().expect("Failed join SC process thread");
            self.process_join_handle = None;
        }
    }

    pub fn set_options_and_reboot(&mut self, opts: Options) {
        self.options = opts;
        self.reboot();
    }
}

