mod options;
pub use self::options::Options;
use std::process::{Command, Output, Stdio};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::thread;
use super::ScClientError;
use super::OscType;
use super::OscHandler;

pub struct Server {
    pub options: Arc<Options>,
    process_join_handle: Option<JoinHandle<Output>>,
    pub osc_handler: OscHandler,
    sync_uid: u64,
}

impl Server {
    pub fn new(options: Options) -> Self {
        let server_address = format!("{}:{}", options.address, options.udp_port_number);
        let client_address = format!("{}:{}", options.client_address, options.client_port);
        let osc_handler = OscHandler::new(&client_address, &server_address);

        Server {
            options: Arc::new(options),
            process_join_handle: None,
            osc_handler: osc_handler,
            sync_uid: 0,
        }
    }

    pub fn boot(&mut self) {
        if self.process_join_handle.is_some() {
            return warn!("SuperCollider server is already running.");
        }

        let options = self.options.clone();
        self.process_join_handle = Some(self.init_new_sc_process_thread(options));
    }
    
    fn init_new_sc_process_thread(&self, options: Arc<Options>) -> JoinHandle<Output> {
        thread::spawn(move || {
            match Command::new(options.path.clone())
                .args(&options.to_args())
                // .stdin(Stdio::piped())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output() {
                    Err(e) => panic!("couldn't start {}: {}", options.path, e),
                    Ok(process) => process,
                }
        })
    }

    pub fn reboot(&mut self) -> Result<(), ScClientError> {
        self.shutdown()?;
        Ok(self.boot())
    }

    pub fn shutdown(&mut self) -> Result<(), ScClientError> {
        self.osc_handler.send_message("/quit", None)?;

        self.osc_handler.add_responder_for_address("/quit", |_| info!("Quiting"));

        if let Some(handle) = self.process_join_handle.take() {
            handle.join()
                .map_err(|e| ScClientError::new(&format!("Failed join SC process thread: {:?}", e)))?;
            self.process_join_handle = None;
            self.osc_handler.remove_responder_for_address("/quit");
        }

        Ok(())
    }

    pub fn set_options_and_reboot(&mut self, opts: Options) -> Result<(), ScClientError> {
        self.options = Arc::new(opts);
        Ok(self.reboot()?)
    }

    pub fn set_receive_notifications(&mut self, is_receiving: bool) -> Result<(), ScClientError> {
        self.osc_handler.send_message("/notify", Some(vec!(OscType::Int(is_receiving as i32))))?;
        self.osc_handler.add_responder_for_address("/notify", move |_| info!("Server notifications set to {}", is_receiving));
        Ok(())
    }

    /// Get status and performs callback with parameters:
    ///
    /// - number of unit generators.
    /// - number of synths.
    /// - number of groups.
    /// - number of loaded synth definitions.
    /// - average percent CPU usage for signal processing
    /// - peak percent CPU usage for signal processing
    /// - nominal sample rate
    /// - actual sample rate
    ///
    /// > status won't return, if the server is in dump_osc mode
    pub fn get_status<F>(&mut self, on_reply: F) -> Result<(), ScClientError> 
        where F: Fn(i32, i32, i32, i32, f32, f32, f32, f32) + Send + Sync + 'static {
            self.osc_handler.send_message("/status", None)?;
            self.osc_handler.add_responder_for_address("/status.reply", move |message| {
                if let Some(ref args) = message.args {
                    let mut num_of_ugens: i32 = 0;
                    if let OscType::Int(n) = args[0] { num_of_ugens = n; }
                    let mut num_of_synths: i32 = 0;
                    if let OscType::Int(n) = args[1] { num_of_synths = n; }
                    let mut num_of_groups: i32 = 0;
                    if let OscType::Int(n) = args[2] { num_of_groups = n; }
                    let mut num_of_synthdefs: i32 = 0;
                    if let OscType::Int(n) = args[3] { num_of_synthdefs = n; }
                    let mut avg_cpu: f32 = 0.0;
                    if let OscType::Float(a) = args[4] { avg_cpu = a; }
                    let mut peak_cpu: f32 = 0.0;
                    if let OscType::Float(p) = args[5] { peak_cpu = p; }
                    let mut nom_sample_rate: f32 = 0.0;
                    if let OscType::Float(n) = args[6] { nom_sample_rate = n; }
                    let mut actual_sample_rate: f32 = 0.0;
                    if let OscType::Float(a) = args[7] { actual_sample_rate = a; }
                    on_reply(num_of_ugens, 
                             num_of_synths,
                             num_of_groups, 
                             num_of_synthdefs, 
                             avg_cpu, 
                             peak_cpu, 
                             nom_sample_rate, 
                             actual_sample_rate);
                }
            });
            Ok(())
        }

    pub fn set_dump_osc_mode(&mut self, mode: DumpOscMode) -> Result<(), ScClientError> {
        self.osc_handler.send_message("/dumpOSC", Some(vec!(OscType::Int(mode as i32))))?;
        Ok(())
    }

    pub fn clear_message_queue(&mut self) -> Result<(), ScClientError> {
        self.osc_handler.send_message("/clearSched", None)?;
        Ok(())
    }

    /// Get server version and performs callback with parameters:
    ///
    /// - Program name. May be "scsynth" or "supernova".
    /// - Major version number.
    /// - Minor version number.
    /// - Patch version name.
    /// - Git branch name.
    /// - First seven hex digits of the commit hash.
    pub fn get_version<F>(&mut self, on_reply: F) -> Result<(), ScClientError> 
        where F: Fn(String, i32, i32, String, String, String) + Send + Sync + 'static {
            self.osc_handler.send_message("/version", None)?;
            self.osc_handler.add_responder_for_address("/version.reply", move |message| {
                if let Some(ref args) = message.args {
                    let mut program_name = String::new();
                    if let OscType::String(ref v) = args[0] { program_name = v.to_string(); }
                    let mut major_version: i32 = 0;
                    if let OscType::Int(n) = args[1] { major_version = n; }
                    let mut minor_version: i32 = 0;
                    if let OscType::Int(n) = args[2] { minor_version = n; }
                    let mut patch_name = String::new();
                    if let OscType::String(ref v) = args[3] { patch_name = v.to_string(); }
                    let mut git_branch = String::new();
                    if let OscType::String(ref v) = args[4] { git_branch = v.to_string(); }
                    let mut commit_hash = String::new();
                    if let OscType::String(ref v) = args[5] { commit_hash = v.to_string(); }
                    on_reply(program_name, 
                             major_version,
                             minor_version,
                             patch_name,
                             git_branch,
                             commit_hash);
                }
            });
            Ok(())
        }

    pub fn sync(&mut self) {
        self.sync_uid += 1;
        self.osc_handler.sync(self.sync_uid);
    }
}

#[derive(Debug)]
pub enum DumpOscMode {
    Off,
    PrintParsed,
    PrintHex,
    PrintParsedAndHex,
}
