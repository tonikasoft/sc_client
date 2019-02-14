mod notify_responder;
mod options;
mod quit_responder;
mod status_responder;
mod version_responder;
pub use self::options::Options;
use self::notify_responder::NotifyResponder;
use self::quit_responder::QuitResponder;
use self::status_responder::StatusResponder;
use self::version_responder::VersionResponder;
use std::process::{Command, Child, Stdio};
use std::sync::Arc;
use super::{
    OscResponder,
    OscServer, 
    OscType, 
    ScClientError, 
    ScClientResult,
};
use std::io::{BufRead, BufReader};

pub struct Server {
    pub options: Arc<Options>,
    pub osc_server: OscServer,
    sc_server: Option<Child>,
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

        self.sc_server = Some(self.init_new_sc_server());
        self.guess_server_ready()?;

        Ok(self)
    }

    fn init_new_sc_server(&self) -> Child {
        match Command::new(self.options.path.clone())
            .args(self.options.to_args())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn() {
                Err(e) => panic!("couldn't start {}: {}", self.options.path, e),
                Ok(process) => process,
            }
    }

    fn guess_server_ready(&mut self) -> ScClientResult<()> {
        let mut child_out = BufReader::new(self.sc_server.as_mut().expect("can't get SC server's process")
                                           .stdout.as_mut().expect("can't get SC server's stdout"));
        let mut line = String::new();
        loop {
            child_out.read_line(&mut line).unwrap();
            print!("{}", line);
            if line.contains("ready") { return Ok(()) }
            line.clear();
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }

    pub fn reboot(&mut self) -> ScClientResult<&Self> {
        self.shutdown()?;
        self.boot()?;
        Ok(self)
    }

    pub fn shutdown(&mut self) -> ScClientResult<&Self> {
        if self.sc_server.is_some() {
            let quit_responder = QuitResponder{};
            let address = quit_responder.get_address();
            self.osc_server.add_responder(quit_responder)?;

            self.osc_server.send_message("/quit", None)?;
            self.sync()?;

            if let Err(e) = self.sc_server.as_mut().unwrap().wait() {
                return Err(ScClientError::new(&format!("{}", e)));
            }

            self.osc_server.remove_responder_for_address(&address);
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
        let notify_responder = NotifyResponder::new(is_receiving);
        self.osc_server.add_responder(notify_responder)?;
        self.osc_server.send_message("/notify", Some(vec!(OscType::Int(is_receiving as i32))))?;
        Ok(self)
    }

    /// Get status and perform callback with [`ServerStatus`](server/struct.ServerStatus.html) as the parameter.
    /// > status won't return, if the server is in dump_osc mode
    pub fn get_status<F>(&mut self, on_reply: F) -> ScClientResult<&Self> 
        where F: Fn(ServerStatus) + Send + Sync + 'static {
            let status_responder = StatusResponder::new(on_reply);
            self.osc_server.add_responder(status_responder)?;
            self.osc_server.send_message("/status", None)?;
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
            let version_responder = VersionResponder::new(on_reply);
            self.osc_server.add_responder(version_responder)?;
            self.osc_server.send_message("/version", None)?;
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

    pub fn set_error_mode(&mut self, error_mode: &ScServerErrorMode) -> ScClientResult<&Self> {
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
pub enum ScServerErrorMode {
    OffUntilNext = 0,
    On = 1,
    OffForBundle = -1,
    OnForBundle = -2,
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
