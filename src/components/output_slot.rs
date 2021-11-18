use crate::resources::ShadyAssets;
use crate::systems::spawner::{spawn_element, SpawnType};
use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;
use shady_generator::OutputProperty;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug)]
pub struct ShadyOutputSlot;

impl ShadyOutputSlot {
    pub fn spawn(
        commands: &mut Commands,
        assets: &ShadyAssets,
        pos: Vec2,
        property: &OutputProperty,
    ) -> Entity {
        spawn_element(
            commands,
            assets,
            pos,
            (&property.reference, &property.name),
            SpawnType::OutputProperty {
                input_fields: vec![(property.reference.clone(), property.glsl_type)],
            },
        )
    }
}
