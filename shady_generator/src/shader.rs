use crate::error::ShadyError;
use crate::graphic_library::GraphicLibrary;
use crate::node::*;
use crate::property::*;
use crate::shader_operation::{ShaderOperation, ShaderOperationResponse};
use crate::shader_type::ShaderType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const MAX_DEPTH: u8 = u8::MAX;

// TODO: protection levels
#[derive(Debug, Serialize, Deserialize)]
pub struct Shader {
    pub name: String,
    pub library: GraphicLibrary,
    pub shader_type: ShaderType,
    pub input_properties: HashMap<String, InputProperty>,
    pub output_properties: HashMap<String, OutputProperty>,
    pub nodes: HashMap<String, Node>,
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
                let to_node = self.nodes.get_mut(&id).ok_or(ShadyError::MissingNode(id))?;
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
                let to_node = self.nodes.get_mut(&id).ok_or(ShadyError::MissingNode(id))?;
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

    pub fn to_glsl(&self) -> Result<String, ShadyError> {
        let mut property_declarations = String::new();
        for property in self.input_properties.values() {
            property_declarations =
                format!("{}\n{}", property_declarations, property.glsl_declaration());
        }
        for property in self.output_properties.values() {
            property_declarations =
                format!("{}\n{}", property_declarations, property.glsl_declaration());
        }
        let mut struct_declarations = Vec::new();
        // TODO: implement the function loading
        let mut function_declarations = vec![""];
        let mut main_content = String::new();
        let mut nodes_to_handle = Vec::new();
        let mut handled_nodes = Vec::new();
        for property in self.output_properties.values() {
            main_content = format!("{}\n{}", main_content, property.to_glsl());
            if let Some(Connection::NodeConnection { node_id, .. }) = &property.connection {
                nodes_to_handle.push(node_id.clone());
            }
        }
        for depth in 0..=MAX_DEPTH {
            log::debug!(
                "Depth: {}, {} nodes to declare",
                depth,
                nodes_to_handle.len()
            );
            nodes_to_handle.dedup();
            let mut tmp_nodes = Vec::new();
            for node_id in nodes_to_handle.drain(..) {
                log::trace!("Processing node {}", node_id);
                if handled_nodes.contains(&node_id) {
                    return Err(ShadyError::NodeLoopDetected(node_id));
                }
                let node = self
                    .nodes
                    .get(&node_id)
                    .ok_or_else(|| ShadyError::MissingNode(node_id.clone()))?;
                main_content = format!("{}\n{}", main_content, node.to_glsl());
                let mut connections = node.node_connections();
                tmp_nodes.append(&mut connections);
                if let Some(declaration) = node.struct_declaration() {
                    struct_declarations.push(declaration);
                }
                handled_nodes.push(node_id);
            }
            nodes_to_handle = tmp_nodes;
            if nodes_to_handle.is_empty() {
                log::info!("Finished processing nodes at depth {}", depth);
                break;
            } else if depth == MAX_DEPTH {
                return Err(ShadyError::MaxDepthReached(MAX_DEPTH));
            }
        }

        struct_declarations.dedup();
        function_declarations.dedup();
        let struct_declarations = struct_declarations.join("\n\n");
        let function_declarations = function_declarations.join("\n\n");

        Ok(formatdoc! {"
            {properties}
            
            {structs}

            {functions}

            void main() {{
                {main}
            }}
        ", 
            properties = property_declarations,
            structs = struct_declarations,
            functions = function_declarations,
            main = main_content
        })
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
        }
    }
}
