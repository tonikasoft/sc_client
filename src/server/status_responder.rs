use super::super::{
    OscMessage, 
    OscResponder,
    OscResponderType,
    OscType, 
    ScClientResult,
    ServerStatus,
};

pub struct StatusResponder<F: Fn(ServerStatus) + Send + Sync + 'static> {
    on_reply_callback: F,
}

impl<F: Fn(ServerStatus) + Send + Sync + 'static> StatusResponder<F> {
    pub fn new(on_reply_callback: F) -> Self {
        StatusResponder {
            on_reply_callback: on_reply_callback,
        }
    }
}

impl<F: Fn(ServerStatus) + Send + Sync + 'static> OscResponder for StatusResponder<F> {
    fn callback(&self, message: &OscMessage) -> ScClientResult<()> {
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

            (self.on_reply_callback)(server_status);
        }
        Ok(())
    }       

    fn get_address(&self) -> String {
        String::from("/status.reply")
    }

    fn get_responder_type(&self) -> OscResponderType {
        OscResponderType::Always
    }
}
