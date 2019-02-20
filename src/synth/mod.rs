extern crate uid;
mod control_value_responder;
use crate::{
    OscType,
    ScClientError,
    ScClientResult,
    Server,
};
use self::uid::Id;
use self::control_value_responder::ControlValueResponder;

pub struct Synth<'a> {
    name: String,
    id: i32,
    target_id: i32,
    server: &'a Server,
}

impl<'a> Synth<'a> {
    pub fn new(server: &'a Server, name: &str, add_action: &AddAction, target_id: i32, args: Vec<OscType>) -> ScClientResult<Self> {
        let id = Synth::init_id();
        let synth = Synth {
            name: name.to_string(),
            id,
            target_id,
            server,
        };
        synth.init_on_server(add_action, args)?;
        Ok(synth)
    }

    fn init_id() -> i32 {
        let id = Id::<i32>::new();
        id.get() as i32
    }

    fn init_on_server(&self, add_action: &AddAction, mut args: Vec<OscType>) -> ScClientResult<()> {
        Synth::check_args(&args)?;
        let mut send_args = vec!(
            OscType::String(self.name.clone()),
            OscType::Int(self.id),
            OscType::Int(add_action.clone() as i32),
            OscType::Int(self.target_id)
        );
        send_args.append(&mut args);
        self.server.osc_server.borrow_mut().send_message("/s_new", Some(send_args))?;

        Ok(())
    }

    fn check_args(args: &Vec<OscType>) -> ScClientResult<()> {
        if args.len()%2 != 0 {
            return Err(ScClientError::new("wrong number of arguments for Synth"));
        }

        Ok(())
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_target_id(&self) -> i32 {
        self.target_id
    }

    pub fn get_control_value<F>(&self, param: OscType, on_reply: F) -> ScClientResult<&Self> 
        where F: Fn(OscType) + Send + Sync + 'static {
            let responder = ControlValueResponder::new(self.id, param.clone(), on_reply);
            self.server.osc_server.borrow_mut().add_responder(responder)?;
            self.server.osc_server.borrow_mut().send_message("/s_get", Some(vec!(OscType::Int(self.id), param)))?;
            Ok(self)
        }
}

#[derive(Debug, Clone)]
pub enum AddAction {
    /// add the new node to the the head of the group specified by the add target ID.
    Head = 0,
    /// add the new node to the the tail of the group specified by the add target ID.
    Tail = 1,
    /// add the new node just before the node specified by the add target ID.
    Before = 2,
    /// add the new node just after the node specified by the add target ID.
    After = 3,
    /// the new node replaces the node specified by the add target ID. The target node is freed.
    Replace = 4,
}
