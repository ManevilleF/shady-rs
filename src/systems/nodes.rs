use crate::components::ShadyNode;
use crate::events::SpawnNode;
use crate::resources::ShadyAssets;
use bevy::prelude::*;

pub fn handle_node_spawn(
    mut commands: Commands,
    mut spawn_evr: EventReader<SpawnNode>,
    assets: Res<ShadyAssets>,
) {
    for event in spawn_evr.iter() {
        ShadyNode::spawn(&mut commands, &assets, event.target_position, "test");
    }
}
