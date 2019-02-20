mod notify_responder;
mod options;
mod quit_responder;
mod status_responder;
mod version_responder;
mod sc_server_process;
pub use self::options::Options;
use self::notify_responder::NotifyResponder;
use self::quit_responder::QuitResponder;
use self::status_responder::StatusResponder;
use self::version_responder::VersionResponder;
use crate::{
    OscServer, 
    OscType, 
    ScClientError, 
    ScClientResult,
};
use self::sc_server_process::ScServerProcess;
use std::cell::RefCell;

pub struct Server {
    pub options: RefCell<Options>,
    pub osc_server: RefCell<OscServer>,
    sc_server_process: RefCell<Option<ScServerProcess>>,
}

impl Server {
    pub fn new(options: Options) -> Self {
        let server_address = format!("{}:{}", options.address, options.udp_port_number);
        let client_address = format!("{}:{}", options.client_address, options.client_port);
        let osc_server = OscServer::new(&client_address, &server_address);

        Server {
            options: RefCell::new(options),
            sc_server_process: RefCell::new(None),
            osc_server: RefCell::new(osc_server),
        }
    }

    pub fn boot(&self) -> ScClientResult<&Self> {
        let mut proc = self.sc_server_process.borrow_mut();

        if proc.is_some() {
            return Err(ScClientError::new("SuperCollider server is already running."));
        }

        *proc = Some(ScServerProcess::new(&self.options.borrow())?);

        Ok(self)
    }

    pub fn reboot(&self) -> ScClientResult<&Self> {
        self.shutdown()?;
        self.boot()?;
        Ok(self)
    }

    pub fn shutdown(&self) -> ScClientResult<&Self> {
        let mut proc = self.sc_server_process.borrow_mut();
        let osc_server = self.osc_server.borrow();
        if proc.is_some() {
            let quit_responder = QuitResponder{};
            osc_server.add_responder(quit_responder)?;

            osc_server.send_message("/quit", None)?;
            self.sync()?;

            proc.as_mut().unwrap().wait_for_finish()?;

            *proc = None;
        }

        Ok(self)
    }

    pub fn set_options_and_reboot(&self, opts: Options) -> ScClientResult<&Self> {
        self.options.replace(opts);
        self.reboot()
    }

    pub fn sync(&self) -> ScClientResult<&Self> {
        let mut osc_server = self.osc_server.borrow_mut();
        osc_server.sync()?;
        Ok(self)
    }
    
    pub fn set_receive_notifications(&self, is_receiving: bool) -> ScClientResult<&Self> {
        let notify_responder = NotifyResponder::new(is_receiving);
        let osc_server = self.osc_server.borrow();
        osc_server.add_responder(notify_responder)?;
        osc_server.send_message("/notify", Some(vec!(OscType::Int(is_receiving as i32))))?;
        Ok(self)
    }

    /// Get status and perform callback with [`ServerStatus`](server/struct.ServerStatus.html) as the parameter.
    /// > status won't return, if the server is in dump_osc mode
    pub fn get_status<F>(&self, on_reply: F) -> ScClientResult<&Self> 
        where F: Fn(ServerStatus) + Send + Sync + 'static {
            let status_responder = StatusResponder::new(on_reply);
            let osc_server = self.osc_server.borrow();
            osc_server.add_responder(status_responder)?;
            osc_server.send_message("/status", None)?;
            Ok(self)
        }

    pub fn set_dump_osc_mode(&self, mode: DumpOscMode) -> ScClientResult<&Self> {
        let osc_server = self.osc_server.borrow();
        osc_server.send_message("/dumpOSC", Some(vec!(OscType::Int(mode as i32))))?;
        Ok(self)
    }

    pub fn clear_message_queue(&self) -> ScClientResult<&Self> {
        let osc_server = self.osc_server.borrow();
        osc_server.send_message("/clearSched", None)?;
        Ok(self)
    }

    /// Get server version and perform callback with
    /// [`ServerVersion`](server/struct.ServerVersion.html) as the parameter.
    pub fn get_version<F>(&self, on_reply: F) -> ScClientResult<&Self>
        where F: Fn(ServerVersion) + Send + Sync + 'static {
            let version_responder = VersionResponder::new(on_reply);
            let osc_server = self.osc_server.borrow();
            osc_server.add_responder(version_responder)?;
            osc_server.send_message("/version", None)?;
            Ok(self)
        }

    pub fn call_plugin_command(&self, command_name: &str, arguments: Option<Vec<OscType>>) -> ScClientResult<&Self> {
        let mut send_args = vec!(OscType::String(command_name.to_string()));
        if let Some(mut command_args) = arguments {
            send_args.append(&mut command_args);
        };
        let osc_server = self.osc_server.borrow();
        osc_server.send_message("/cmd", Some(send_args))?;
        Ok(self)
    }

    pub fn set_error_mode(&self, error_mode: ScServerErrorMode) -> ScClientResult<&Self> {
        let osc_server = self.osc_server.borrow();
        osc_server.send_message("/error", Some(vec!(OscType::Int(error_mode as i32))))?;
        Ok(self)
    }
}

// Drop implemented for Server, because when we try to kill the child process in ScServerProcess
// in drop, we get an error that the process is already exited
impl Drop for Server {
    fn drop(&mut self) {
        if let Some(ref mut process) = *self.sc_server_process.borrow_mut() {
            process.kill_child()
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
