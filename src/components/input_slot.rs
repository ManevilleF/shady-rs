use bevy::prelude::*;

use crate::resources::ShadyAssets;
use crate::systems::spawner::{spawn_element, SpawnType};
#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;
use shady_generator::InputProperty;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug)]
pub struct ShadyInputSlot {
    pub connected_to: Option<Entity>,
}

impl Default for ShadyInputSlot {
    fn default() -> Self {
        Self { connected_to: None }
    }
}

impl ShadyInputSlot {
    pub fn spawn(
        commands: &mut Commands,
        assets: &ShadyAssets,
        pos: Vec2,
        property: &InputProperty,
    ) -> Entity {
        spawn_element(
            commands,
            assets,
            pos,
            (&property.reference, &property.name),
            SpawnType::InputProperty {
                output_fields: vec![(property.reference.clone(), property.glsl_type)],
            },
        )
    }
}
