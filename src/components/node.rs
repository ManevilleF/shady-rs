use crate::common::Bounds;
use crate::components::{BoxInteraction, InteractionBox};
use crate::resources::ShadyAssets;
use bevy::prelude::*;

pub struct ShadyNode {}

impl ShadyNode {
    pub fn spawn(commands: &mut Commands, assets: &ShadyAssets, pos: Vec2, name: &str) -> Entity {
        let x_size = 75.;
        let title_size = Vec2::new(x_size, 20.);
        let box_size = Vec2::splat(5.);
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
                    sprite: Sprite::new(Vec2::new(x_size, 100.)),
                    material: assets.node_body_material.clone(),
                    transform: Transform::from_xyz(0., -50., 0.),
                    ..Default::default()
                })
                .insert(Name::new(format!("{}_node_body", name)))
                .with_children(|bb| {
                    for i in 0..2 {
                        bb.spawn_bundle(SpriteBundle {
                            sprite: Sprite::new(box_size),
                            material: assets.input_slot_material.clone(),
                            transform: Transform::from_xyz(-40., -20. * i as f32, 0.),
                            ..Default::default()
                        })
                        .insert(InteractionBox::new(box_size, BoxInteraction::Connect));
                        bb.spawn_bundle(SpriteBundle {
                            sprite: Sprite::new(box_size),
                            material: assets.output_slot_material.clone(),
                            transform: Transform::from_xyz(40., -20. * i as f32, 0.),
                            ..Default::default()
                        })
                        .insert(InteractionBox::new(box_size, BoxInteraction::Connect));
                    }
                });
            })
            .id()
    }
}
