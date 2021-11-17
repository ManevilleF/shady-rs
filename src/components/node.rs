use crate::common::Bounds;
use crate::components::{BoxInteraction, InteractionBox, NodeInput, NodeOutput};
use crate::resources::ShadyAssets;
use bevy::prelude::*;
use shady_generator::Node;
use std::cmp::max;

const NODE_SIZE_X: f32 = 75.;
const NODE_HEADER_SIZE_Y: f32 = 20.;
const SLOT_SIZE: f32 = 10.;
const SLOT_STEP: f32 = 30.;

pub struct ShadyNode {
    node_id: String,
}

impl ShadyNode {
    fn title_text(value: &str, assets: &ShadyAssets) -> Text {
        Text {
            sections: vec![TextSection {
                value: value.to_string(),
                style: TextStyle {
                    font: assets.font.clone(),
                    color: Color::BLACK,
                    ..Default::default()
                },
            }],
            alignment: TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        }
    }

    pub fn spawn(commands: &mut Commands, assets: &ShadyAssets, pos: Vec2, node: &Node) -> Entity {
        let node_name = node.name();
        let header_size = Vec2::new(NODE_SIZE_X, NODE_HEADER_SIZE_Y);
        let slot_size = Vec2::splat(SLOT_SIZE);
        let slot_x_pos = NODE_SIZE_X / 2. - slot_size.x / 2. - 5.;
        let input_fields = node.input_fields();
        let output_fields = node.output_fields();
        let field_len = max(input_fields.len(), output_fields.len()) as f32;
        let body_size = Vec2::new(NODE_SIZE_X, field_len * SLOT_STEP);
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite::new(header_size),
                material: assets.node_title_material.clone(),
                transform: Transform::from_xyz(pos.x, pos.y, 0.),
                ..Default::default()
            })
            .insert(Name::new(format!("{}_node_title", node_name)))
            .insert(InteractionBox::new(header_size, BoxInteraction::Drag))
            .with_children(|b| {
                b.spawn_bundle(Text2dBundle {
                    text: Self::title_text(node_name, assets),
                    transform: Transform::from_xyz(0., 0., 1.),
                    ..Default::default()
                });
                b.spawn_bundle(SpriteBundle {
                    sprite: Sprite::new(body_size),
                    material: assets.node_body_material.clone(),
                    transform: Transform::from_xyz(0., -header_size.y / 2. - body_size.y / 2., 0.),
                    ..Default::default()
                })
                .insert(Name::new(format!("{}_node_body", node_name)))
                .insert(InteractionBox::new(body_size, BoxInteraction::Ignore));
                for (i, (field_name, field)) in input_fields.into_iter().enumerate() {
                    b.spawn_bundle(SpriteBundle {
                        sprite: Sprite::new(slot_size),
                        material: assets.glsl_type_material(field.glsl_type),
                        transform: Transform::from_xyz(
                            -slot_x_pos,
                            -NODE_HEADER_SIZE_Y - (SLOT_STEP * i as f32),
                            0.,
                        ),
                        ..Default::default()
                    })
                    .insert(InteractionBox::new(
                        slot_size,
                        BoxInteraction::ConnectionEnd,
                    ))
                    .insert(NodeInput {
                        connected_to: None,
                        field_name,
                    });
                }
                for (i, (field_name, field)) in output_fields.into_iter().enumerate() {
                    b.spawn_bundle(SpriteBundle {
                        sprite: Sprite::new(slot_size),
                        material: assets.glsl_type_material(field),
                        transform: Transform::from_xyz(
                            slot_x_pos,
                            -NODE_HEADER_SIZE_Y - (SLOT_STEP * i as f32),
                            0.,
                        ),
                        ..Default::default()
                    })
                    .insert(InteractionBox::new(
                        slot_size,
                        BoxInteraction::ConnectionStart,
                    ))
                    .insert(NodeOutput { field_name });
                }
            })
            .id()
    }
}
