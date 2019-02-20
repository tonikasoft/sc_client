use crate::{
    OscMessage, 
    OscResponder,
    ResponseType,
    OscType, 
    ScClientResult,
    ServerVersion,
};

pub struct VersionResponder<F: Fn(ServerVersion) + Send + Sync + 'static> {
    on_reply_callback: F,
}

impl<F: Fn(ServerVersion) + Send + Sync + 'static> VersionResponder<F> {
    pub fn new(on_reply_callback: F) -> Self {
        VersionResponder { on_reply_callback }
    }
}

impl<F: Fn(ServerVersion) + Send + Sync + 'static> OscResponder for VersionResponder<F> {
    fn callback(&self, message: &OscMessage) -> ScClientResult<()> {
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

            (self.on_reply_callback)(server_version);
        }
        Ok(())
    }       

    fn get_address(&self) -> String {
        String::from("/version.reply")
    }

    fn get_response_type(&self) -> ResponseType {
        ResponseType::Once
    }
}
