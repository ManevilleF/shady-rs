use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug)]
pub struct NodeConnector {
    pub output_from: Entity,
    pub input_to: Entity,
}
