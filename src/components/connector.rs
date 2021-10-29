use crate::common::Bounds;
use bevy::prelude::*;
use std::ops::Deref;

#[derive(Debug)]
pub struct NodeConnector {
    pub output_from: Entity,
    pub input_to: Entity,
}

#[derive(Debug)]
pub struct ConnectorBox(pub Bounds);

impl Deref for ConnectorBox {
    type Target = Bounds;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
