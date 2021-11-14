use crate::node::{Connection, ConnectionAttempt, ConnectionTo, Node, NodePreset};

#[derive(Debug, Clone)]
pub enum ShaderOperation {
    CreateNodeFromPreset(NodePreset),
    CreateNode(Node),
    RemoveNode(String),
    Connect(ConnectionAttempt),
    RemoveConnection(ConnectionTo),
}

#[derive(Debug, Clone)]
pub enum ShaderOperationResponse {
    AddedNode(String),
    AddedConnection(ConnectionAttempt),
    RemovedNode(Node),
    RemovedConnection(Connection),
}
