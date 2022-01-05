use crate::NativeType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Ord)]
pub enum ConnectionTo {
    Node { id: String, field_name: String },
    OutputProperty { id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionAttempt {
    pub connection_from: Connection,
    pub connection_to: ConnectionTo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionMessage {
    pub connection: Connection,
    pub native_type: NativeType,
}

pub type ConnectionResponse = Option<Connection>;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Ord)]
pub enum Connection {
    Constant { id: String },
    InputProperty { id: String },
    ComplexOutputNode { id: String, field_name: String },
    SingleOutputNode { id: String },
}

impl ConnectionMessage {
    pub const fn new(connection: Connection, native_type: NativeType) -> Self {
        Self {
            connection,
            native_type,
        }
    }
}

impl Connection {
    pub fn glsl_call(&self) -> String {
        match self {
            Connection::InputProperty { id }
            | Connection::SingleOutputNode { id }
            | Connection::Constant { id } => id.clone(),
            Connection::ComplexOutputNode {
                id: node_id,
                field_name,
            } => format!("{}.{}", node_id, field_name),
        }
    }
}
