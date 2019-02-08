mod options;
pub use self::options::Options;
use std::process::{Command, Child, Stdio};
use std::sync::Arc;
use super::{ScClientResult, ScClientError, OscType, OscServer};
use std::io::{BufRead, BufReader};

pub struct Server {
    pub options: Arc<Options>,
    sc_server: Option<Child>,
    pub osc_server: OscServer,
}

impl Server {
    pub fn new(options: Options) -> Self {
        let server_address = format!("{}:{}", options.address, options.udp_port_number);
        let client_address = format!("{}:{}", options.client_address, options.client_port);
        let osc_server = OscServer::new(&client_address, &server_address);

        Server {
            options: Arc::new(options),
            sc_server: None,
            osc_server: osc_server,
        }
    }

    pub fn boot(&mut self) -> ScClientResult<&Self> {
        if self.sc_server.is_some() {
            return Err(ScClientError::new("SuperCollider server is already running."));
        }

        let options = self.options.clone();
        self.sc_server = Some(self.init_new_sc_server(options));
        self.guess_server_ready()?;

        Ok(self)
    }

    fn guess_server_ready(&mut self) -> ScClientResult<()> {
        let mut child_out = BufReader::new(self.sc_server.as_mut().expect("can't get SC server's process")
                                           .stdout.as_mut().expect("can't get SC server's stdout"));
        let mut line = String::new();
        loop {
            child_out.read_line(&mut line).unwrap();
            print!("{}", line);
            if line.contains("ready") {
                return Ok(())
            }
            line = String::new();
        }
    }

    fn init_new_sc_server(&self, options: Arc<Options>) -> Child {
        match Command::new(options.path.clone())
            .args(&options.to_args())
            // .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn() {
                Err(e) => panic!("couldn't start {}: {}", options.path, e),
                Ok(process) => process,
            }
    }

    pub fn reboot(&mut self) -> ScClientResult<&Self> {
        self.shutdown()?;
        self.boot()?;
        Ok(self)
    }

    pub fn shutdown(&mut self) -> ScClientResult<&Self> {
        self.osc_server.send_message("/quit", None)?;

        self.osc_server.add_responder_for_address("/quit", |_| info!("Quiting"));

        if self.sc_server.is_some() {
            self.osc_server.remove_responder_for_address("/quit");
            self.sc_server = None;
        }

        Ok(self)
    }

    pub fn set_options_and_reboot(&mut self, opts: Options) -> ScClientResult<&Self> {
        self.options = Arc::new(opts);
        self.reboot()
    }

    pub fn sync(&mut self) -> ScClientResult<&Self> {
        self.osc_server.sync()?;
        Ok(self)
    }
    
    pub fn set_receive_notifications(&mut self, is_receiving: bool) -> ScClientResult<&Self> {
        self.osc_server.send_message("/notify", Some(vec!(OscType::Int(is_receiving as i32))))?;
        self.osc_server.add_responder_for_address("/notify", move |_| info!("Server notifications set to {}", is_receiving));
        Ok(self)
    }

    /// Get status and perform callback with [`ServerStatus`](server/struct.ServerStatus.html) as the parameter.
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

    pub fn set_dump_osc_mode(&mut self, mode: DumpOscMode) -> ScClientResult<&Self> {
        self.osc_server.send_message("/dumpOSC", Some(vec!(OscType::Int(mode as i32))))?;
        Ok(self)
    }

    pub fn clear_message_queue(&mut self) -> ScClientResult<&Self> {
        self.osc_server.send_message("/clearSched", None)?;
        Ok(self)
    }

    /// Get server version and perform callback with
    /// [`ServerVersion`](server/struct.ServerVersion.html) as the parameter.
    pub fn get_version<F>(&mut self, on_reply: F) -> ScClientResult<&Self>
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
            Ok(self)
        }

    pub fn call_plugin_command(&mut self, command_name: &str, arguments: Option<Vec<OscType>>) -> ScClientResult<&Self> {
        let mut send_args = vec!(OscType::String(command_name.to_string()));
        if let Some(mut command_args) = arguments {
            send_args.append(&mut command_args);
        };
        self.osc_server.send_message("/cmd", Some(send_args))?;
        Ok(self)
    }

}

impl Drop for Server {
    fn drop(&mut self) {
        if let Some(ref mut process) = self.sc_server {
            process.kill()
                .expect("can't kill SC server");
        };
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
    /// number of unit generators.
    pub num_of_ugens: i32,
    /// number of synths.
    pub num_of_synths: i32,
    /// number of groups.
    pub num_of_groups: i32,
    /// number of loaded synth definitions.
    pub num_of_synthdefs: i32,
    /// average percent CPU usage for signal processing
    pub avg_cpu: f32,
    /// peak percent CPU usage for signal processing
    pub peak_cpu: f32,
    /// nominal sample rate
    pub nom_sample_rate: f32,
    /// actual sample rate
    pub actual_sample_rate: f32,
}

#[derive(Clone, Debug)]
pub struct ServerVersion {
    /// Program name. May be "scsynth" or "supernova".
    pub program_name: String,
    /// Major version number.
    pub major_version: i32,
    /// Minor version number.
    pub minor_version: i32,
    /// Patch version name.
    pub patch_name: String,
    /// Git branch name.
    pub git_branch: String,
    /// First seven hex digits of the commit hash.
    pub commit_hash: String,
}
