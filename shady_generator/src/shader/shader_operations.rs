use crate::error::ShadyError;
use crate::node::{Connection, ConnectionAttempt, ConnectionTo, Node, NodePreset};
use crate::shader::Shader;

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

impl Shader {
    pub fn apply_operation(
        &mut self,
        operation: ShaderOperation,
    ) -> Result<Vec<ShaderOperationResponse>, ShadyError> {
        let mut vec = Vec::new();
        match operation {
            ShaderOperation::CreateNodeFromPreset(preset) => {
                let node = preset.get_node();
                let id = node.uuid.clone();
                self.create_node(node);
                vec.push(ShaderOperationResponse::AddedNode(id));
            }
            ShaderOperation::CreateNode(node) => {
                let id = node.uuid.clone();
                self.create_node(node);
                vec.push(ShaderOperationResponse::AddedNode(id));
            }
            ShaderOperation::RemoveNode(id) => match self.remove_node(&id) {
                None => {
                    log::warn!("Tried to removed inexistant node {}", id);
                }
                Some(n) => {
                    vec.push(ShaderOperationResponse::RemovedNode(n));
                }
            },
            ShaderOperation::Connect(attempt) => {
                let res = self.connect(attempt.clone())?;
                vec.push(ShaderOperationResponse::AddedConnection(attempt));
                if let Some(connection) = res {
                    vec.push(ShaderOperationResponse::RemovedConnection(connection));
                }
            }
            ShaderOperation::RemoveConnection(connection_to) => {
                if let Some(connection) = self.disconnect(connection_to)? {
                    vec.push(ShaderOperationResponse::RemovedConnection(connection))
                }
            }
        }
        Ok(vec)
    }
}
