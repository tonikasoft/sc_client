use crate::{
    OscMessage,
    OscResponder,
    AfterCallAction,
    ScClientResult,
};

pub struct QuitResponder;
impl OscResponder for QuitResponder {
    fn callback(&self, _message: &OscMessage) -> ScClientResult<()> {
        Ok(info!("Quit..."))
    }       
    
    fn get_address(&self) -> String {
        String::from("/quit")
    }

    fn get_after_call_action(&self, _message: &OscMessage) -> AfterCallAction {
        AfterCallAction::None
    }
}

