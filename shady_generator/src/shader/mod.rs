pub use {property::*, shader_operations::*, shader_type::*, to_glsl::*};

mod property;
mod shader_operations;
mod shader_type;
mod to_glsl;

use crate::error::ShadyError;
use crate::graphic_library::GraphicLibrary;
use crate::node::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// TODO: protection levels
#[derive(Debug, Serialize, Deserialize)]
pub struct Shader {
    pub name: String,
    pub library: GraphicLibrary,
    pub shader_type: ShaderType,
    pub input_properties: HashMap<String, InputProperty>,
    pub output_properties: HashMap<String, OutputProperty>,
    pub nodes: HashMap<String, Node>,
    pub max_processing_depth: usize,
}

impl Shader {
    pub fn create_node(&mut self, node: Node) {
        if let Some(n) = self.nodes.insert(node.uuid.clone(), node) {
            log::error!(
                "FATAL: Overwrote node {}_{} because of identical uuid",
                n.name,
                n.uuid
            );
        }
    }

    pub fn create_node_from_preset(&mut self, node: NodePreset) {
        let node = node.get_node();
        self.create_node(node)
    }

    pub fn remove_node(&mut self, node_uuid: &str) -> Option<Node> {
        match self.nodes.remove(node_uuid) {
            None => {
                log::error!("Could not find node with uuid {} to remove", node_uuid);
                None
            }
            Some(n) => Some(n),
        }
    }

    fn get_node(&self, id: &str) -> Result<&Node, ShadyError> {
        self.nodes
            .get(id)
            .ok_or_else(|| ShadyError::MissingNode(id.to_string()))
    }

    fn get_node_mut(&mut self, id: &str) -> Result<&mut Node, ShadyError> {
        self.nodes
            .get_mut(id)
            .ok_or_else(|| ShadyError::MissingNode(id.to_string()))
    }

    pub fn add_input_property(&mut self, property: InputProperty) -> Option<InputProperty> {
        self.input_properties
            .insert(property.reference.clone(), property)
    }

    pub fn add_output_property(&mut self, property: OutputProperty) -> Option<OutputProperty> {
        self.output_properties
            .insert(property.reference.clone(), property)
    }

    pub fn connect(
        &mut self,
        connection_attempt: ConnectionAttempt,
    ) -> Result<ConnectionResponse, ShadyError> {
        let glsl_type = match &connection_attempt.connection_from {
            Connection::PropertyConnection { property_id } => {
                self.input_properties
                    .get(property_id)
                    .ok_or_else(|| ShadyError::MissingInputProperty(property_id.clone()))?
                    .glsl_type
            }
            Connection::NodeConnection {
                node_id,
                field_name,
            } => {
                let from_node = self
                    .nodes
                    .get(node_id)
                    .ok_or_else(|| ShadyError::MissingNode(node_id.clone()))?;
                from_node
                    .get_output_field(field_name)
                    .ok_or_else(|| ShadyError::WrongFieldKey(field_name.clone()))?
            }
        };
        let connection_message = ConnectionMessage {
            connection: connection_attempt.connection_from.clone(),
            glsl_type,
        };
        match connection_attempt.connection_to {
            ConnectionTo::ToNode { id, field } => {
                let to_node = self.get_node_mut(&id)?;
                to_node.connect_input(&field, connection_message)
            }
            ConnectionTo::OutputProperty { id } => {
                let property = self
                    .output_properties
                    .get_mut(&id)
                    .ok_or(ShadyError::MissingInputProperty(id))?;
                property.connect_input(connection_message)
            }
        }
    }

    pub fn disconnect(
        &mut self,
        connection_to: ConnectionTo,
    ) -> Result<Option<Connection>, ShadyError> {
        match connection_to {
            ConnectionTo::ToNode { id, field } => {
                let to_node = self.get_node_mut(&id)?;
                to_node.disconnect_field(&field)
            }
            ConnectionTo::OutputProperty { id } => {
                let property = self
                    .output_properties
                    .get_mut(&id)
                    .ok_or(ShadyError::MissingInputProperty(id))?;
                Ok(property.disconnect())
            }
        }
    }

    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Self::default()
        }
    }
}

impl Default for Shader {
    fn default() -> Self {
        Self {
            name: "MyShader".to_string(),
            library: Default::default(),
            shader_type: Default::default(),
            input_properties: Default::default(),
            output_properties: Default::default(),
            nodes: Default::default(),
            max_processing_depth: 256,
        }
    }
}
