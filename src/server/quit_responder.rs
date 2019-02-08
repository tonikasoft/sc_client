use super::super::{
    OscMessage,
    OscResponder,
    ResponseType,
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

    fn get_response_type(&self) -> ResponseType {
        ResponseType::Always
    }
}
