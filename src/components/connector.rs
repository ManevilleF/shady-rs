use crate::common::Bounds;
use bevy::prelude::*;

#[derive(Debug)]
pub struct NodeConnector {
    pub output_from: Entity,
    pub input_to: Entity,
}
