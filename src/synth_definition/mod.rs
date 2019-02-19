use crate::{
    OscType,
    ScClientResult,
    Server,
};

pub struct SynthDefinition<'a> {
    server: &'a Server,
}

impl<'a> SynthDefinition<'a> {
    pub fn new(server: &'a Server) -> Self {
        SynthDefinition { server }
    }

    pub fn send(&self, buf: &Vec<u8>) -> ScClientResult<&Self> {
        self.server.osc_server.send_message("/d_recv", Some(vec!(OscType::Blob(buf.clone()))))?;
        Ok(self)
    }

    pub fn load(&self, file_path: &str) -> ScClientResult<&Self> {
        self.server.osc_server.send_message("/d_load", Some(vec!(OscType::String(file_path.to_string()))))?;
        Ok(self)
    }

    pub fn load_directory(&self, path: &str) -> ScClientResult<&Self> {
        self.server.osc_server.send_message("/d_loadDir", Some(vec!(OscType::String(path.to_string()))))?;
        Ok(self)
    }

    pub fn free(&self, name: &str) -> ScClientResult<&Self> {
        self.server.osc_server.send_message("/d_free", Some(vec!(OscType::String(name.to_string()))))?;
        Ok(self)
    }
}
