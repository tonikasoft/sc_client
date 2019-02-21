use crate::{
    OscType,
    ScClientResult,
    Server,
};

pub struct SynthDefinition;

impl SynthDefinition {
    pub fn send(server: &Server, buf: &Vec<u8>) -> ScClientResult<()> {
        server.osc_server.borrow_mut().send_message("/d_recv", vec![buf.clone().into()].into())?;
        Ok(())
    }

    /// file_path can be a pattern like "synthdefs/perc-*"
    pub fn load(server: &Server, file_path: &str) -> ScClientResult<()> {
        server.osc_server.borrow_mut().send_message("/d_load", vec![file_path.into()].into())?;
        Ok(())
    }

    pub fn load_directory(server: &Server, path: &str) -> ScClientResult<()> {
        server.osc_server.borrow_mut().send_message("/d_loadDir", vec![path.into()].into())?;
        Ok(())
    }

    pub fn free(server: &Server, name: &str) -> ScClientResult<()> {
        server.osc_server.borrow_mut().send_message("/d_free", vec![name.into()].into())?;
        Ok(())
    }
}
