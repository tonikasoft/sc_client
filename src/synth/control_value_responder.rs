use crate::{
    OscMessage, 
    OscResponder,
    AfterCallAction,
    OscType, 
    ScClientResult,
};

pub struct ControlValueResponder<F: Fn(OscType) + Send + Sync + 'static> {
    on_reply_callback: F,
    param: OscType,
    synth_id: i32,
}

impl<F: Fn(OscType) + Send + Sync + 'static> ControlValueResponder<F> {
    pub fn new(synth_id: i32, param: OscType, on_reply_callback: F) -> Self {
        ControlValueResponder { on_reply_callback, synth_id, param, }
    }
}

impl<F: Fn(OscType) + Send + Sync + 'static> OscResponder for ControlValueResponder<F> {
    fn callback(&self, message: &OscMessage) -> ScClientResult<()> {
        if let Some(ref args) = message.args {
            if args[0] == OscType::Int(self.synth_id) && args[1] == self.param {
                (self.on_reply_callback)(args[2].clone());
            }
        }
        Ok(())
    }       

    fn get_address(&self) -> String {
        String::from("/n_set")
    }

    fn get_after_call_action(&self, message: &OscMessage) -> AfterCallAction {
        if let Some(ref args) = message.args {
            if args[0] == OscType::Int(self.synth_id) && args[1] == self.param {
                return AfterCallAction::None;
            }
        }

        AfterCallAction::Reschedule
    }
}
