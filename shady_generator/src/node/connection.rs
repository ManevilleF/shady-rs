use crate::glsl::GlslType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionTo {
    ToNode { id: String, field: String },
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
    pub glsl_type: GlslType,
}

pub type ConnectionResponse = Option<Connection>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Connection {
    PropertyConnection { property_id: String },
    NodeConnection { node_id: String, field_name: String },
}

impl ConnectionMessage {
    pub fn new(connection: Connection, glsl_type: GlslType) -> Self {
        Self {
            connection,
            glsl_type,
        }
    }
}

impl Connection {
    pub fn glsl_call(&self) -> String {
        match self {
            Connection::PropertyConnection { property_id } => property_id.clone(),
            Connection::NodeConnection {
                node_id,
                field_name,
            } => format!("{}.{}", node_id, field_name),
        }
    }
}
