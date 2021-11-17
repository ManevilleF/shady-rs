use crate::{Commands, DespawnRecursiveExt, Entity};
use bevy::log;
use bevy::utils::HashMap;
use shady_generator::{NodePreset, Shader};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct CurrentShader {
    pub shader: Shader,
    pub node_entities: HashMap<String, Entity>,
    pub connection_entities: HashMap<String, Entity>,
}

#[derive(Debug, Clone)]
pub struct SelectedNodePreset(pub Option<NodePreset>);

impl CurrentShader {
    pub fn delete_node_entity(&mut self, node_id: &str, commands: &mut Commands) {
        match self.node_entities.remove(node_id) {
            None => {
                log::warn!("No entity for node {}", node_id);
            }
            Some(e) => {
                commands.entity(e).despawn_recursive();
            }
        }
    }
}

impl Default for CurrentShader {
    fn default() -> Self {
        Self {
            shader: Default::default(),
            node_entities: Default::default(),
            connection_entities: Default::default(),
        }
    }
}

impl Default for SelectedNodePreset {
    fn default() -> Self {
        Self(None)
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
