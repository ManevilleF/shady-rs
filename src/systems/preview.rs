use crate::resources::CreationCandidate;
use crate::{PreviewMaterial, ShaderEvent};
use bevy::prelude::*;

pub fn handle_shader_event(
    mut spawn_evr: EventReader<ShaderEvent>,
    mut preview_material: ResMut<PreviewMaterial>,
) {
    for event in spawn_evr.iter() {
        match event {
            ShaderEvent::CreateElement {
                candidate: CreationCandidate::InputProperty(property),
                ..
            } => {
                preview_material.insert_input(property);
            }
            ShaderEvent::DeleteInputProperty { id } => {
                preview_material.input_values.remove(id);
            }
            _ => (),
        }
    }
}
