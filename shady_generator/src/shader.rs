use crate::error::ShadyError;
use crate::glsl::AsGlslPrimitiveType;
use crate::node::{ConnectResponse, ConnectionData, InputField, Node};
use crate::property::Property;
use bevy::log;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Shader {
    pub name: String,
    pub properties: HashMap<String, Property>,
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

    pub fn remove_node(&mut self, node_uuid: &str) -> Option<Node> {
        match self.nodes.remove(node_uuid) {
            None => {
                log::error!("Could not find node with uuid {} to remove", node_uuid);
                None
            }
            Some(n) => Some(n),
        }
    }

    pub fn connect(
        &mut self,
        (from_node_uuid, output_field): (&str, &str),
        (to_node_uuid, input_field): (&str, &str),
    ) -> Result<(), ShadyError> {
        let (output_var_name, from_field) = {
            let from_node = self
                .nodes
                .get(from_node_uuid)
                .ok_or_else(|| ShadyError::MissingNode(from_node_uuid.to_string()))?;
            let from_field = from_node
                .get_output_field(output_field)
                .ok_or_else(|| ShadyError::WrongFieldKey(output_field.to_string()))?;
            (from_node.output_var_name(), from_field)
        };
        let mut to_node = self
            .nodes
            .get_mut(to_node_uuid)
            .ok_or_else(|| ShadyError::MissingNode(to_node_uuid.to_string()))?;
        let connection_data = ConnectionData::new(&output_var_name, output_field, from_field);
        let res = to_node.connect_input(input_field, connection_data)?;
        if let Some(data) = res {}
        Ok(())
    }

    pub fn refresh_connections(&mut self) {
        for (uuid, mut node) in self.nodes.iter_mut() {
            for (key, input_field) in node.input_fields().iter() {
                if let InputField::NodeConnected(data) = input_field {}
            }
        }
    }
}
