use crate::node::{Connector, Node};

pub enum ShaderOperation {
    CreateNode(Node),
    RemoveNode(String),
    Connect {},
    RemoveConnector(String),
}

#[derive(Debug, Clone)]
pub enum ShaderOperationResponse {
    AddedNode(String),
    AddedConnector(String),
    RemovedNode(Node),
    RemovedConnector(Connector),
}
