use crate::components::ShadyNode;
use crate::events::SpawnNode;
use crate::resources::ShadyAssets;
use crate::CurrentShader;
use bevy::log;
use bevy::prelude::*;
use shady_generator::{ShaderOperation, ShadyError};

pub fn handle_node_spawn(
    mut commands: Commands,
    mut spawn_evr: EventReader<SpawnNode>,
    mut current_shader: ResMut<CurrentShader>,
    assets: Res<ShadyAssets>,
) {
    for event in spawn_evr.iter() {
        let node = current_shader.create_node_from_preset(event.node_preset);
        ShadyNode::spawn(&mut commands, &assets, event.target_position, node);
    }
}
