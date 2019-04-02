pub use rosc::{OscBundle, OscColor, OscMessage, OscMidiMessage, OscPacket, OscType};

#[derive(Clone, Debug, PartialEq)]
pub struct NodeValue(pub OscType, pub OscType);

impl From<NodeValue> for Vec<OscType> {
    fn from(node_arg: NodeValue) -> Self {
        vec![node_arg.0, node_arg.1]
    }
}

impl From<&NodeValue> for Vec<OscType> {
    fn from(node_arg: &NodeValue) -> Self {
        vec![node_arg.0.clone(), node_arg.1.clone()]
    }
}

impl From<(OscType, OscType)> for NodeValue {
    fn from(osc_tuple: (OscType, OscType)) -> Self {
        NodeValue(osc_tuple.0, osc_tuple.1)
    }
}

impl From<Vec<OscType>> for NodeValue {
    fn from(value: Vec<OscType>) -> Self {
        if value.len() != 2 {
            panic!("vector length should be == 2 to convert it into NodeValue");
        }

        NodeValue(value[0].clone(), value[1].clone())
    }
}

impl From<&Vec<OscType>> for NodeValue {
    fn from(value: &Vec<OscType>) -> Self {
        NodeValue::from(value.clone())
    }
}
