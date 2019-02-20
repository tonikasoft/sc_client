use crate::{
    OscType,
    ScClientResult,
    Server,
};

pub struct SynthDefinition;
impl SynthDefinition {
    pub fn send(server: &Server, buf: &Vec<u8>) -> ScClientResult<()> {
        server.osc_server.send_message("/d_recv", Some(vec!(OscType::Blob(buf.clone()))))?;
        Ok(())
    }

    /// file_path can be a pattern like "synthdefs/perc-*"
    pub fn load(server: &Server, file_path: &str) -> ScClientResult<()> {
        server.osc_server.send_message("/d_load", Some(vec!(OscType::String(file_path.to_string()))))?;
        Ok(())
    }

    pub fn load_directory(server: &Server, path: &str) -> ScClientResult<()> {
        server.osc_server.send_message("/d_loadDir", Some(vec!(OscType::String(path.to_string()))))?;
        Ok(())
    }

    pub fn free(server: &Server, name: &str) -> ScClientResult<()> {
        server.osc_server.send_message("/d_free", Some(vec!(OscType::String(name.to_string()))))?;
        Ok(())
    }
}
