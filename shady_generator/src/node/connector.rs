use crate::generate_uuid;
use crate::glsl::GlslType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionMessage {
    pub connector_id: String,
    pub glsl_type: GlslType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionResponse {
    pub connector_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Connection {
    PropertyConnection { property_id: String },
    NodeConnection { node_id: String, field_name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connector {
    pub id: String,
    pub from: Connection,
    pub to: Connection,
}

impl Connector {
    pub fn new(from: Connection, to: Connection) -> Self {
        Self {
            id: generate_uuid(),
            from,
            to,
        }
    }
}

impl ConnectionMessage {
    pub fn new(connector_id: &str, glsl_type: GlslType) -> Self {
        Self {
            connector_id: connector_id.to_string(),
            glsl_type,
        }
    }
}
