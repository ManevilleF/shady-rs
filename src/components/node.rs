use crate::common::Bounds;
use crate::components::{BoxInteraction, InteractionBox, NodeInput, NodeOutput};
use crate::resources::ShadyAssets;
use bevy::prelude::*;
use shady_generator::Node;
use std::cmp::max;

pub struct ShadyNode {
    node_id: String,
}

impl ShadyNode {
    pub fn spawn(commands: &mut Commands, assets: &ShadyAssets, pos: Vec2, node: &Node) -> Entity {
        let name = node.name();
        let x_size = 75.;
        let title_size = Vec2::new(x_size, 20.);
        let box_size = Vec2::splat(10.);
        let box_x_pos = x_size / 2. + box_size.x / 2.;
        let input_fields = node.input_fields();
        let output_fields = node.output_fields();
        let field_len = max(input_fields.len(), output_fields.len()) as f32;
        let body_size = Vec2::new(x_size, field_len * 30.);
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
                    for (i, (field_name, _)) in input_fields.into_iter().enumerate() {
                        bb.spawn_bundle(SpriteBundle {
                            sprite: Sprite::new(box_size),
                            material: assets.input_slot_material.clone(),
                            transform: Transform::from_xyz(-box_x_pos, -20. * i as f32, 0.),
                            ..Default::default()
                        })
                        .insert(InteractionBox::new(box_size, BoxInteraction::ConnectionEnd))
                        .insert(NodeInput {
                            connected_to: None,
                            field_name,
                        });
                    }
                    for (i, (field_name, _)) in output_fields.into_iter().enumerate() {
                        bb.spawn_bundle(SpriteBundle {
                            sprite: Sprite::new(box_size),
                            material: assets.output_slot_material.clone(),
                            transform: Transform::from_xyz(box_x_pos, -20. * i as f32, 0.),
                            ..Default::default()
                        })
                        .insert(InteractionBox::new(
                            box_size,
                            BoxInteraction::ConnectionStart,
                        ))
                        .insert(NodeOutput { field_name });
                    }
                });
            })
            .id()
    }
}
