pub use {constant::*, property::*, shader_type::*, to_glsl::*};

mod constant;
mod precision;
mod property;
mod shader_type;
mod to_glsl;

use crate::shader::precision::ShaderPrecision;
use crate::ShadyError::{DuplicateConstant, DuplicateInputProperty, DuplicateOutputProperty};
use crate::{
    ordered_map, Connection, ConnectionAttempt, ConnectionMessage, ConnectionResponse,
    ConnectionTo, GraphicLibrary, NativeType, Node, OutputFields, ShadyError,
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
    pub default_precisions: HashMap<NativeType, ShaderPrecision>,
    #[serde(serialize_with = "ordered_map")]
    constants: HashMap<String, Constant>,
    #[serde(serialize_with = "ordered_map")]
    input_properties: HashMap<String, InputProperty>,
    #[serde(serialize_with = "ordered_map")]
    output_properties: HashMap<String, OutputProperty>,
    #[serde(serialize_with = "ordered_map")]
    nodes: HashMap<String, Node>,
    pub max_processing_depth: usize,
}

impl Shader {
    pub fn create_node(&mut self, node: Node) -> Result<&Node, ShadyError> {
        let id = node.unique_id().clone();
        if self.nodes.contains_key(&id) {
            return Err(ShadyError::DuplicateNode(id));
        }
        self.nodes.insert(id.clone(), node);
        Ok(self.get_node(&id).unwrap())
    }

    pub fn remove_node(&mut self, id: &str) -> Option<Node> {
        match self.nodes.remove(id) {
            None => {
                log::error!("Could not find node with id {} to remove", id);
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

    pub fn nodes(&self) -> &HashMap<String, Node> {
        &self.nodes
    }

    fn get_constant(&self, id: &str) -> Result<&Constant, ShadyError> {
        self.constants
            .get(id)
            .ok_or_else(|| ShadyError::MissingConstant(id.to_string()))
    }

    pub fn constants(&self) -> &HashMap<String, Constant> {
        &self.constants
    }

    fn get_input_property(&self, id: &str) -> Result<&InputProperty, ShadyError> {
        self.input_properties
            .get(id)
            .ok_or_else(|| ShadyError::MissingInputProperty(id.to_string()))
    }

    pub fn input_properties(&self) -> &HashMap<String, InputProperty> {
        &self.input_properties
    }

    fn get_output_property(&self, id: &str) -> Result<&OutputProperty, ShadyError> {
        self.output_properties
            .get(id)
            .ok_or_else(|| ShadyError::MissingOutputProperty(id.to_string()))
    }

    pub fn output_properties(&self) -> &HashMap<String, OutputProperty> {
        &self.output_properties
    }

    fn get_node_mut(&mut self, id: &str) -> Result<&mut Node, ShadyError> {
        self.nodes
            .get_mut(id)
            .ok_or_else(|| ShadyError::MissingNode(id.to_string()))
    }

    pub fn add_constant(&mut self, constant: Constant) -> Result<&Constant, ShadyError> {
        let id = constant.reference.clone();
        if self.constants.contains_key(&id) {
            return Err(DuplicateConstant(id));
        }
        self.constants.insert(id.clone(), constant);
        Ok(self.get_constant(&id).unwrap())
    }

    pub fn add_input_property(
        &mut self,
        property: InputProperty,
    ) -> Result<&InputProperty, ShadyError> {
        let id = property.reference.clone();
        if self.input_properties.contains_key(&id) {
            return Err(DuplicateInputProperty(id));
        }
        self.input_properties.insert(id.clone(), property);
        Ok(self.get_input_property(&id).unwrap())
    }

    pub fn add_output_property(
        &mut self,
        property: OutputProperty,
    ) -> Result<&OutputProperty, ShadyError> {
        let id = property.reference.clone();
        if self.output_properties.contains_key(&id) {
            return Err(DuplicateOutputProperty(id));
        }
        self.output_properties.insert(id.clone(), property);
        Ok(self.get_output_property(&id).unwrap())
    }

    pub fn remove_constant(&mut self, id: &str) -> Option<Constant> {
        match self.constants.remove(id) {
            None => {
                log::error!("Could not find constant with id {} to remove", id);
                None
            }
            Some(n) => Some(n),
        }
    }
    pub fn remove_input_property(&mut self, id: &str) -> Option<InputProperty> {
        match self.input_properties.remove(id) {
            None => {
                log::error!("Could not find input property with id {} to remove", id);
                None
            }
            Some(n) => Some(n),
        }
    }

    pub fn remove_output_property(&mut self, id: &str) -> Option<OutputProperty> {
        match self.output_properties.remove(id) {
            None => {
                log::error!("Could not find output property with id {} to remove", id);
                None
            }
            Some(n) => Some(n),
        }
    }

    pub fn connect(
        &mut self,
        connection_attempt: ConnectionAttempt,
    ) -> Result<ConnectionResponse, ShadyError> {
        let glsl_type = match &connection_attempt.connection_from {
            Connection::InputProperty { id } => {
                self.input_properties
                    .get(id)
                    .ok_or_else(|| ShadyError::MissingInputProperty(id.clone()))?
                    .native_type
            }
            Connection::Constant { id } => self
                .constants
                .get(id)
                .ok_or_else(|| ShadyError::MissingConstant(id.clone()))?
                .native_type(),
            Connection::ComplexOutputNode { id, field_name } => {
                let from_node = self
                    .nodes
                    .get(id)
                    .ok_or_else(|| ShadyError::MissingNode(id.clone()))?;
                from_node.get_output_field(field_name)?
            }
            Connection::SingleOutputNode { id } => {
                let from_node = self
                    .nodes
                    .get(id)
                    .ok_or_else(|| ShadyError::MissingNode(id.clone()))?;
                match from_node.output_fields() {
                    OutputFields::SingleOutput(t) => t,
                    OutputFields::Fields(_) => return Err(ShadyError::ComplexOutput(id.clone())),
                }
            }
        };
        let connection_message = ConnectionMessage {
            connection: connection_attempt.connection_from.clone(),
            native_type: glsl_type,
        };
        match connection_attempt.connection_to {
            ConnectionTo::Node {
                id,
                field_name: field,
            } => {
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
            ConnectionTo::Node {
                id,
                field_name: field,
            } => {
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

    pub fn safe_name(&self) -> String {
        self.name.to_ascii_lowercase().trim().replace(" ", "_")
    }

    pub fn shader_file_name(&self) -> String {
        format!(
            "{}.{}",
            self.safe_name(),
            match self.shader_type {
                ShaderType::Vertex => "vert",
                ShaderType::Fragment => "frag",
            }
        )
    }

    pub fn save_file_name(&self) -> String {
        format!("{}.yaml", self.safe_name(),)
    }

    pub fn save(&self) -> Result<(), ShadyError> {
        self.save_to(self.save_file_name().as_str())
    }

    pub fn export_glsl_to(&self, file_path: &str) -> Result<(), ShadyError> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
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
            constants: Default::default(),
            input_properties: Default::default(),
            output_properties: Default::default(),
            nodes: Default::default(),
            max_processing_depth: DEFAULT_MAX_DEPTH,
        }
    }
}
