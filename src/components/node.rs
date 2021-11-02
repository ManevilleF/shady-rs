use crate::common::Bounds;
use crate::components::{BoxInteraction, InteractionBox, NodeInput, NodeOutput};
use crate::resources::ShadyAssets;
use bevy::prelude::*;

pub struct ShadyNode {}

impl ShadyNode {
    pub fn spawn(commands: &mut Commands, assets: &ShadyAssets, pos: Vec2, name: &str) -> Entity {
        let x_size = 75.;
        let title_size = Vec2::new(x_size, 20.);
        let box_size = Vec2::splat(5.);
        let body_size = Vec2::new(x_size, 100.);
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite::new(title_size),
                material: assets.node_title_material.clone(),
                transform: Transform::from_xyz(pos.x, pos.y, 0.),
                ..Default::default()
            })
            .insert(Name::new(format!("{}_node_title", name)))
            .insert(InteractionBox::new(title_size, BoxInteraction::Drag))
            .with_children(|b| {
                b.spawn_bundle(SpriteBundle {
                    sprite: Sprite::new(body_size),
                    material: assets.node_body_material.clone(),
                    transform: Transform::from_xyz(0., -title_size.y / 2. - body_size.y / 2., 0.),
                    ..Default::default()
                })
                .insert(Name::new(format!("{}_node_body", name)))
                .insert(InteractionBox::new(body_size, BoxInteraction::Ignore))
                .with_children(|bb| {
                    for i in 0..2 {
                        bb.spawn_bundle(SpriteBundle {
                            sprite: Sprite::new(box_size),
                            material: assets.input_slot_material.clone(),
                            transform: Transform::from_xyz(-40., -20. * i as f32, 0.),
                            ..Default::default()
                        })
                        .insert(InteractionBox::new(box_size, BoxInteraction::ConnectionEnd))
                        .insert(NodeInput {});
                        bb.spawn_bundle(SpriteBundle {
                            sprite: Sprite::new(box_size),
                            material: assets.output_slot_material.clone(),
                            transform: Transform::from_xyz(40., -20. * i as f32, 0.),
                            ..Default::default()
                        })
                        .insert(InteractionBox::new(
                            box_size,
                            BoxInteraction::ConnectionStart,
                        ))
                        .insert(NodeOutput {});
                    }
                });
            })
            .id()
    }
}
