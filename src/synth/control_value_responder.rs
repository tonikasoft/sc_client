use crate::{types::{OscMessage, OscType, NodeValue}, AfterCallAction, OscResponder, ScClientResult};
use std::sync::Mutex;

pub struct ControlValueResponder<F: Fn(Vec<NodeValue>) + Send + Sync + 'static> {
    on_reply_callback: F,
    params: Vec<OscType>,
    synth_id: i32,
    after_call_action: Mutex<AfterCallAction>,
}

impl<F: Fn(Vec<NodeValue>) + Send + Sync + 'static> ControlValueResponder<F> {
    pub fn new(synth_id: i32, params: Vec<OscType>, on_reply_callback: F) -> Self {
        ControlValueResponder {
            on_reply_callback,
            synth_id,
            params,
            after_call_action: Mutex::new(AfterCallAction::Reschedule)
        }
    }

    fn args_into_node_values(&self, value: &Vec<OscType>) -> Vec<NodeValue> {
        value
            .chunks(2)
            .map(|chunk| NodeValue::from(chunk.to_vec()))
            .collect()
    }

    fn check_params(&self, params: &Vec<NodeValue>) -> bool {
        if self.params.len() != params.len() { return false; }

        params
            .iter()
            .all(|param| self.params.contains(&param.0))
    }
}

impl<F: Fn(Vec<NodeValue>) + Send + Sync + 'static> OscResponder for ControlValueResponder<F> {
    fn callback(&self, message: &OscMessage) -> ScClientResult<()> {
        if let Some(ref args) = message.args {
            let args_vec = args[1..].to_vec();
            let node_values = self.args_into_node_values(&args_vec);
            if args[0] == OscType::Int(self.synth_id) && self.check_params(&node_values) {
                (self.on_reply_callback)(node_values);
                *self.after_call_action
                    .lock()
                    .unwrap() = AfterCallAction::None;
            }
        }
        Ok(())
    }

    fn get_address(&self) -> String {
        String::from("/n_set")
    }

    fn get_after_call_action(&self, message: &OscMessage) -> AfterCallAction {
        (*self.after_call_action.lock().unwrap()).clone()
    }
}
