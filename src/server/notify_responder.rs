use super::super::{
    OscMessage,
    OscResponder,
    OscResponderType,
    ScClientResult,
};

pub struct NotifyResponder {
    is_receiving: bool,
}

impl NotifyResponder {
    pub fn new(is_receiving: bool) -> Self {
        NotifyResponder {
            is_receiving: is_receiving,
        }
    }
}

impl OscResponder for NotifyResponder {
    fn callback(&self, _message: &OscMessage) -> ScClientResult<()> {
        Ok(info!("Server notifications set to {}", self.is_receiving))
    }       

    fn get_address(&self) -> String {
        String::from("/notify")
    }

    fn get_responder_type(&self) -> OscResponderType {
        OscResponderType::Always
    }
}

