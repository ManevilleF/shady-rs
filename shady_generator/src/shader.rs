use crate::error::ShadyError;
use crate::graphic_library::GraphicLibrary;
use crate::node::*;
use crate::property::*;
use crate::shader_type::ShaderType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// TODO: prottection levels
#[derive(Debug, Serialize, Deserialize)]
pub struct Shader {
    pub name: String,
    pub library: GraphicLibrary,
    pub shader_type: ShaderType,
    pub input_properties: HashMap<String, InputProperty>,
    pub output_properties: HashMap<String, OutputProperty>,
    pub nodes: HashMap<String, Node>,
    pub connectors: HashMap<String, Connector>,
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

    pub fn create_node_from_preset(&mut self, node: NodePresets) {
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
        (from_node_uuid, output_field): (&str, &str),
        (to_node_uuid, input_field): (&str, &str),
    ) -> Result<(), ShadyError> {
        let from_field = {
            let from_node = self
                .nodes
                .get(from_node_uuid)
                .ok_or_else(|| ShadyError::MissingNode(from_node_uuid.to_string()))?;
            let from_field = from_node
                .get_output_field(output_field)
                .ok_or_else(|| ShadyError::WrongFieldKey(output_field.to_string()))?;
            from_field
        };
        let mut to_node = self
            .nodes
            .get_mut(to_node_uuid)
            .ok_or_else(|| ShadyError::MissingNode(to_node_uuid.to_string()))?;
        let connector = Connector::new(
            Connection::NodeConnection {
                node_id: from_node_uuid.to_string(),
                field_name: from_field.to_string(),
            },
            Connection::NodeConnection {
                node_id: to_node_uuid.to_string(),
                field_name: input_field.to_string(),
            },
        );
        let connection_message = ConnectionMessage::new(&connector.id, from_field);
        let res = to_node.connect_input(input_field, connection_message)?;
        self.connectors.insert(connector.id.clone(), connector);
        if let Some(connector_id) = res.connector_id {
            self.connectors.remove(&connector_id);
        }
        Ok(())
    }

    pub fn refresh_connections(&mut self) {
        for (uuid, mut node) in self.nodes.iter_mut() {
            for (key, input_field) in node.input_fields().iter() {}
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
            connectors: Default::default(),
        }
    }
}
