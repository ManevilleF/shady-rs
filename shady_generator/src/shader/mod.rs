pub use {property::*, shader_operations::*, shader_type::*, to_glsl::*};

mod precision;
mod property;
mod shader_operations;
mod shader_type;
mod to_glsl;

use crate::shader::precision::ShaderPrecision;
use crate::{
    ordered_map, Connection, ConnectionAttempt, ConnectionMessage, ConnectionResponse,
    ConnectionTo, GlslType, GraphicLibrary, Node, NodePreset, ShadyError,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;

const DEFAULT_MAX_DEPTH: usize = 256;
const EXPORT_HEADER: &str =
    "Generated by shady-rs -- https://github.com/ManevilleF/shady-rs made by @ManevilleF";

const SAVE_HEADER: &str =
    "Save file for shady-rs -- https://github.com/ManevilleF/shady-rs made by @ManevilleF";

#[derive(Debug, Serialize, Deserialize)]
pub struct Shader {
    pub name: String,
    #[serde(default)]
    pub library: GraphicLibrary,
    #[serde(default)]
    pub shader_type: ShaderType,
    // TODO: enable
    // #[serde(
    //     skip_serializing_if = "HashMap::is_empty",
    //     serialize_with = "ordered_map"
    // )]
    pub default_precisions: HashMap<GlslType, ShaderPrecision>,
    #[serde(
        skip_serializing_if = "HashMap::is_empty",
        serialize_with = "ordered_map"
    )]
    input_properties: HashMap<String, InputProperty>,
    #[serde(
        skip_serializing_if = "HashMap::is_empty",
        serialize_with = "ordered_map"
    )]
    output_properties: HashMap<String, OutputProperty>,
    #[serde(
        skip_serializing_if = "HashMap::is_empty",
        serialize_with = "ordered_map"
    )]
    nodes: HashMap<String, Node>,
    pub max_processing_depth: usize,
}

impl Shader {
    pub fn create_node(&mut self, node: Node) -> &Node {
        let id = node.unique_id().clone();
        if let Some(n) = self.nodes.insert(id.clone(), node) {
            log::error!(
                "FATAL: Overwrote node {}_{} because of identical ids",
                n.name(),
                n.unique_id()
            );
        }
        self.get_node(&id).unwrap()
    }

    pub fn create_node_from_preset(&mut self, node: NodePreset) -> &Node {
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

    pub fn node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
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

    pub fn save_to(&self, file_path: &str) -> Result<(), ShadyError> {
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(file_path)?;
        let data = serde_yaml::to_string(&self)?;
        let data = format!("# {}\n{}", SAVE_HEADER, data);
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn save(&self) -> Result<(), ShadyError> {
        let path = self.name.to_ascii_lowercase();
        self.save_to(path.replace(" ", "_").as_str())
    }

    pub fn export_glsl_to(&self, file_path: &str) -> Result<(), ShadyError> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)?;
        let data = self.to_glsl()?;
        let data = format!("// {}\n{}", EXPORT_HEADER, data);
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn load(file_path: &str) -> Result<Self, ShadyError> {
        let val = read_to_string(file_path)?;
        let res = serde_yaml::from_str(&val)?;
        Ok(res)
    }
}

impl Default for Shader {
    fn default() -> Self {
        Self {
            name: "MyShader".to_string(),
            library: Default::default(),
            shader_type: Default::default(),
            default_precisions: Default::default(),
            input_properties: Default::default(),
            output_properties: Default::default(),
            nodes: Default::default(),
            max_processing_depth: DEFAULT_MAX_DEPTH,
        }
    }
}
