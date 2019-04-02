mod control_value_responder;
use self::control_value_responder::ControlValueResponder;
use crate::{types::NodeValue, types::OscType, ScClientResult, Server};
use failure::Fail;
use uid::Id;

pub struct Synth<'a> {
    name: String,
    id: i32,
    target_id: i32,
    server: &'a Server,
}

impl<'a> Synth<'a> {
    pub fn new(
        server: &'a Server,
        name: &str,
        add_action: &AddAction,
        target_id: i32,
        args: &Vec<NodeValue>,
    ) -> ScClientResult<Self> {
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

    fn init_on_server(&self, add_action: &AddAction, args: &Vec<NodeValue>) -> ScClientResult<()> {
        let mut send_args: Vec<OscType> = vec![
            self.name.clone().into(),
            self.id.into(),
            (add_action.clone() as i32).into(),
            self.target_id.into(),
        ];
        let mut flattened_args = Synth::flatten_args(&args);
        send_args.append(&mut flattened_args);
        self.server
            .osc_server
            .borrow()
            .send_message("/s_new", Some(send_args))?;

        Ok(())
    }

    fn flatten_args(args: &Vec<NodeValue>) -> Vec<OscType> {
        args.iter().map(|arg| Vec::from(arg)).flatten().collect()
    }

    pub fn get_control_value<F>(
        &self,
        params: &mut Vec<OscType>,
        on_reply: F,
    ) -> ScClientResult<&Self>
    where
        F: Fn(Vec<NodeValue>) + Send + Sync + 'static,
    {
        let responder = ControlValueResponder::new(self.id, params.clone(), on_reply);
        self.server
            .osc_server
            .borrow_mut()
            .add_responder(responder)?;

        let mut send_args = vec![OscType::Int(self.id)];
        send_args.append(params);
        self.server
            .osc_server
            .borrow()
            .send_message("/s_get", Some(send_args))?;

        Ok(self)
    }

    // pub fn get_control_values_range<F>(&self, from: OscType, number_of_params: u32, on_reply: F) -> ScClientResult<&Self>
    // where F: Fn(OscType) + Send + Sync + 'static {
    // let responder = ControlValuesRangeResponder::new(self.id, param.clone(), on_reply);
    // self.server.osc_server.borrow_mut().add_responder(responder)?;
    // self.server.osc_server.borrow().send_message("/s_getn",
    // Some(vec!(
    // OscType::Int(self.id),
    // param,
    // )))?;
    // Ok(self)
    // }
    //

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_target_id(&self) -> i32 {
        self.target_id
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
