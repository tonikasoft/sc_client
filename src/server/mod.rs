mod options;
pub use self::options::Options;
use std::process::{Command, Output, Stdio};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::thread;
use super::{ScClientResult, ScClientError, OscType, OscServer};

pub struct Server {
    pub options: Arc<Options>,
    process_join_handle: Option<JoinHandle<Output>>,
    pub osc_server: OscServer,
}

impl Server {
    pub fn new(options: Options) -> Self {
        let server_address = format!("{}:{}", options.address, options.udp_port_number);
        let client_address = format!("{}:{}", options.client_address, options.client_port);
        let osc_server = OscServer::new(&client_address, &server_address);

        Server {
            options: Arc::new(options),
            process_join_handle: None,
            osc_server: osc_server,
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
        self.osc_server.send_message("/quit", None)?;

        self.osc_server.add_responder_for_address("/quit", |_| info!("Quiting"));

        if let Some(handle) = self.process_join_handle.take() {
            handle.join()
                .map_err(|e| ScClientError::new(&format!("Failed join SC process thread: {:?}", e)))?;
            self.process_join_handle = None;
            self.osc_server.remove_responder_for_address("/quit");
        }

        Ok(())
    }

    pub fn set_options_and_reboot(&mut self, opts: Options) -> Result<(), ScClientError> {
        self.options = Arc::new(opts);
        Ok(self.reboot()?)
    }

    pub fn set_receive_notifications(&mut self, is_receiving: bool) -> Result<(), ScClientError> {
        self.osc_server.send_message("/notify", Some(vec!(OscType::Int(is_receiving as i32))))?;
        self.osc_server.add_responder_for_address("/notify", move |_| info!("Server notifications set to {}", is_receiving));
        Ok(())
    }

    /// Get status and performs callback with [`ServerStatus`](server/struct.ServerStatus.html) as the parameter.
    /// > status won't return, if the server is in dump_osc mode
    pub fn get_status<F>(&mut self, on_reply: F) -> ScClientResult<&Self> 
        where F: Fn(ServerStatus) + Send + Sync + 'static {
            self.osc_server.send_message("/status", None)?;
            self.osc_server.add_responder_for_address("/status.reply", move |message| {
                if let Some(ref args) = message.args {
                    let mut server_status = ServerStatus {
                        num_of_ugens: 0,
                        num_of_synths: 0,
                        num_of_groups: 0,
                        num_of_synthdefs: 0,
                        avg_cpu: 0.0,
                        peak_cpu: 0.0,
                        nom_sample_rate: 0.0,
                        actual_sample_rate: 0.0,
                    };
                    if let OscType::Int(n) = args[0] { server_status.num_of_ugens = n; }
                    if let OscType::Int(n) = args[1] { server_status.num_of_synths = n; }
                    if let OscType::Int(n) = args[2] { server_status.num_of_groups = n; }
                    if let OscType::Int(n) = args[3] { server_status.num_of_synthdefs = n; }
                    if let OscType::Float(a) = args[4] { server_status.avg_cpu = a; }
                    if let OscType::Float(p) = args[5] { server_status.peak_cpu = p; }
                    if let OscType::Float(n) = args[6] { server_status.nom_sample_rate = n; }
                    if let OscType::Float(a) = args[7] { server_status.actual_sample_rate = a; }
                    on_reply(server_status);
                }
            });
            Ok(self)
        }

    pub fn set_dump_osc_mode(&mut self, mode: DumpOscMode) -> Result<(), ScClientError> {
        self.osc_server.send_message("/dumpOSC", Some(vec!(OscType::Int(mode as i32))))?;
        Ok(())
    }

    pub fn clear_message_queue(&mut self) -> Result<(), ScClientError> {
        self.osc_server.send_message("/clearSched", None)?;
        Ok(())
    }

    /// Get server version and performs callback with
    /// [`ServerVersion`](server/struct.ServerVersion.html) as the parameter.
    pub fn get_version<F>(&mut self, on_reply: F) -> Result<(), ScClientError> 
        where F: Fn(ServerVersion) + Send + Sync + 'static {
            self.osc_server.send_message("/version", None)?;
            self.osc_server.add_responder_for_address("/version.reply", move |message| {
                if let Some(ref args) = message.args {
                    let mut server_version = ServerVersion {
                        program_name: String::new(),
                        major_version: 0,
                        minor_version: 0,
                        patch_name: String::new(),
                        git_branch: String::new(),
                        commit_hash: String::new(),
                    };
                    if let OscType::String(ref v) = args[0] { server_version.program_name = v.to_string(); }
                    if let OscType::Int(n) = args[1] { server_version.major_version = n; }
                    if let OscType::Int(n) = args[2] { server_version.minor_version = n; }
                    if let OscType::String(ref v) = args[3] { server_version.patch_name = v.to_string(); }
                    if let OscType::String(ref v) = args[4] { server_version.git_branch = v.to_string(); }
                    if let OscType::String(ref v) = args[5] { server_version.commit_hash = v.to_string(); }
                    on_reply(server_version);
                }
            });
            Ok(())
        }

    pub fn sync(&mut self) -> ScClientResult<&Self> {
        self.osc_server.sync()?;
        Ok(self)
    }
}

#[derive(Clone, Debug)]
pub enum DumpOscMode {
    Off,
    PrintParsed,
    PrintHex,
    PrintParsedAndHex,
}

#[derive(Clone, Debug)]
pub struct ServerStatus {
    /// - number of unit generators.
    pub num_of_ugens: i32,
    /// - number of synths.
    pub num_of_synths: i32,
    /// - number of groups.
    pub num_of_groups: i32,
    /// - number of loaded synth definitions.
    pub num_of_synthdefs: i32,
    /// - average percent CPU usage for signal processing
    pub avg_cpu: f32,
    /// - peak percent CPU usage for signal processing
    pub peak_cpu: f32,
    /// - nominal sample rate
    pub nom_sample_rate: f32,
    /// - actual sample rate
    pub actual_sample_rate: f32,
}

#[derive(Clone, Debug)]
pub struct ServerVersion {
    /// - Program name. May be "scsynth" or "supernova".
    pub program_name: String,
    /// - Major version number.
    pub major_version: i32,
    /// - Minor version number.
    pub minor_version: i32,
    /// - Patch version name.
    pub patch_name: String,
    /// - Git branch name.
    pub git_branch: String,
    /// - First seven hex digits of the commit hash.
    pub commit_hash: String,
}
