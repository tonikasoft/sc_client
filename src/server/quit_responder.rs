use super::super::{
    OscMessage,
    OscResponder,
    OscResponderType,
    ScClientResult,
};

pub struct QuitResponder;
impl OscResponder for QuitResponder {
    fn callback(&self, _message: &OscMessage) -> ScClientResult<()> {
        Ok(info!("Quiting..."))
    }       
    
    fn get_address(&self) -> String {
        String::from("/quit")
    }

    fn get_responder_type(&self) -> OscResponderType {
        OscResponderType::Always
    }
}

