extern crate uid;
use crate::{
    OscType,
    ScClientResult,
    Server,
};
use self::uid::Id;

pub struct Synth {
    name: String,
    id: i32,
    target_id: i32,
}

impl Synth {
    pub fn new(server: &Server, name: &str, add_action: &AddAction, target_id: i32) -> ScClientResult<Self> {
        let id = Synth::init_id();
        Synth::init_on_server(server, name, id, add_action, target_id)?;
        Ok(Synth {
            name: name.to_string(),
            id,
            target_id,
        })
    }

    fn init_id() -> i32 {
        let id = Id::<i32>::new();
        id.get() as i32
    }

    fn init_on_server(server: &Server, name: &str, id: i32, add_action: &AddAction, target_id: i32) -> ScClientResult<()> {
        server.osc_server.send_message(
            "/s_new", 
            Some(vec!(
                    OscType::String(name.to_string()),
                    OscType::Int(id),
                    OscType::Int(add_action.clone() as i32),
                    OscType::Int(target_id))
            )
        )?;

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
