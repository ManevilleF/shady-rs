use crate::node::{Connection, ConnectionAttempt, Node};

#[derive(Debug, Clone)]
pub enum ShaderOperation {
    CreateNode(Node),
    RemoveNode(String),
    Connect(ConnectionAttempt),
    RemoveConnector(String),
}

#[derive(Debug, Clone)]
pub enum ShaderOperationResponse {
    AddedNode(String),
    AddedConnection(Connection),
    RemovedNode(Node),
    RemovedConnection(Connection),
}
