use crate::components::{LogElement, LogLevel};
use crate::resources::shader_loader::ShaderLoader;
use crate::resources::ShadyAssets;
use bevy::log;
use bevy::prelude::{Commands, DespawnRecursiveExt, Entity};
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
                LogElement::new(LogLevel::Warn, format!("No entity for node {}", id))
                    .spawn(commands);
            }
            Some(e) => {
                commands.entity(e).despawn_recursive();
            }
        }
    }

    pub fn delete_input_property_entity(&mut self, id: &str, commands: &mut Commands) {
        match self.input_property_entities.remove(id) {
            None => {
                LogElement::new(LogLevel::Warn, format!("No entity for input {}", id))
                    .spawn(commands);
            }
            Some(e) => {
                commands.entity(e).despawn_recursive();
            }
        }
    }

    pub fn delete_output_property_entity(&mut self, id: &str, commands: &mut Commands) {
        match self.output_property_entities.remove(id) {
            None => {
                LogElement::new(LogLevel::Warn, format!("No entity for output {}", id))
                    .spawn(commands);
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

    fn clear(&mut self, commands: &mut Commands) {
        for (key, entity) in self.node_entities.drain() {
            log::info!("Removing node {} entity {:?}", key, entity);
            commands.entity(entity).despawn_recursive();
        }
        for (key, entity) in self.input_property_entities.drain() {
            log::info!("Removing input property {} entity {:?}", key, entity);
            commands.entity(entity).despawn_recursive();
        }
        for (key, entity) in self.output_property_entities.drain() {
            log::info!("Removing output property {} entity {:?}", key, entity);
            commands.entity(entity).despawn_recursive();
        }
        for (key, entity) in self.connection_entities.drain() {
            log::info!("Removing connection {} entity {:?}", key, entity);
            commands.entity(entity).despawn_recursive();
        }
    }

    pub fn load(&mut self, shader: Shader, commands: &mut Commands, assets: &ShadyAssets) {
        let mut loader = ShaderLoader::new(shader);
        loader.load(commands, assets);
        self.clear(commands);
        *self = loader.into();
        LogElement::new(
            LogLevel::Info,
            format!("Successfully loaded shader {}", self.name),
        )
        .spawn(commands);
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

impl From<ShaderLoader> for CurrentShader {
    fn from(l: ShaderLoader) -> Self {
        Self {
            shader: l.shader,
            node_entities: l.node_entities,
            input_property_entities: l.input_property_entities,
            output_property_entities: l.output_property_entities,
            connection_entities: l.connection_entities,
        }
    }
}
