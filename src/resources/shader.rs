use crate::{Commands, DespawnRecursiveExt, Entity};
use bevy::log;
use bevy::utils::HashMap;
use shady_generator::{Connection, ConnectionTo, Shader};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct CurrentShader {
    pub shader: Shader,
    pub node_entities: HashMap<String, Entity>,
    pub input_property_entities: HashMap<String, Entity>,
    pub output_property_entities: HashMap<String, Entity>,
    pub connection_entities: HashMap<String, Entity>,
}
impl CurrentShader {
    pub fn delete_node_entity(&mut self, id: &str, commands: &mut Commands) {
        match self.node_entities.remove(id) {
            None => {
                log::error!("No entity for node {}", id);
            }
            Some(e) => {
                commands.entity(e).despawn_recursive();
            }
        }
    }

    pub fn delete_input_property_entity(&mut self, id: &str, commands: &mut Commands) {
        match self.input_property_entities.remove(id) {
            None => {
                log::error!("No entity for input {}", id);
            }
            Some(e) => {
                commands.entity(e).despawn_recursive();
            }
        }
    }

    pub fn delete_output_property_entity(&mut self, id: &str, commands: &mut Commands) {
        match self.output_property_entities.remove(id) {
            None => {
                log::error!("No entity for output {}", id);
            }
            Some(e) => {
                commands.entity(e).despawn_recursive();
            }
        }
    }

    pub fn unique_connector_id(to: &ConnectionTo, from: &Connection) -> String {
        format!(
            "{}_{}",
            match from {
                Connection::InputProperty { property_id } => property_id.clone(),
                Connection::Node {
                    node_id,
                    field_name,
                } => format!("{}::{}", node_id, field_name),
            },
            match to {
                ConnectionTo::Node {
                    node_id: id,
                    field_name: field,
                } => format!("{}::{}", id, field),
                ConnectionTo::OutputProperty { id } => id.clone(),
            }
        )
    }
}

impl Default for CurrentShader {
    fn default() -> Self {
        Self {
            shader: Default::default(),
            node_entities: Default::default(),
            input_property_entities: Default::default(),
            output_property_entities: Default::default(),
            connection_entities: Default::default(),
        }
    }
}

impl Deref for CurrentShader {
    type Target = Shader;

    fn deref(&self) -> &Self::Target {
        &self.shader
    }
}

impl DerefMut for CurrentShader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.shader
    }
}
