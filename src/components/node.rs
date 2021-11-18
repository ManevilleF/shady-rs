use crate::components::{BoxInteraction, InteractionBox, NodeInput, NodeOutput};
use crate::resources::ShadyAssets;
use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;
use shady_generator::{Connection, ConnectionTo, Node};
use std::cmp::max;

const NODE_SIZE_X: f32 = 120.;
const NODE_HEADER_SIZE_Y: f32 = 30.;
const SLOT_SIZE: f32 = 15.;
const SLOT_STEP: f32 = 40.;

#[cfg_attr(feature = "debug", derive(Inspectable))]
#[derive(Debug)]
pub struct ShadyNode;

impl ShadyNode {
    fn title_text(value: &str, assets: &ShadyAssets) -> Text2dBundle {
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: value.to_string(),
                    style: TextStyle {
                        font: assets.font.clone(),
                        color: Color::BLACK,
                        font_size: 20.,
                    },
                }],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            },
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        }
    }

    pub fn spawn(commands: &mut Commands, assets: &ShadyAssets, pos: Vec2, node: &Node) -> Entity {
        let node_name = node.name().to_ascii_lowercase().replace(" ", "_");
        let node_id = node.unique_id();
        let header_size = Vec2::new(NODE_SIZE_X, NODE_HEADER_SIZE_Y);
        let close_node_size = Vec2::splat(NODE_HEADER_SIZE_Y / 2.);
        let slot_size = Vec2::splat(SLOT_SIZE);
        let slot_x_pos = NODE_SIZE_X / 2. - SLOT_SIZE;
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
            .insert(Name::new(format!("{}_node", node_name)))
            .insert(ShadyNode)
            .insert(InteractionBox::new(header_size, BoxInteraction::Drag))
            .with_children(|b| {
                b.spawn_bundle(Self::title_text(&node_name, assets));
                b.spawn_bundle(SpriteBundle {
                    sprite: Sprite::new(body_size),
                    material: assets.node_body_material.clone(),
                    transform: Transform::from_xyz(0., -header_size.y / 2. - body_size.y / 2., 0.),
                    ..Default::default()
                })
                .insert(Name::new(format!("{}_node_body", node_name)))
                .insert(InteractionBox::new(body_size, BoxInteraction::Ignore));
                b.spawn_bundle(SpriteBundle {
                    sprite: Sprite::new(close_node_size),
                    material: assets.close_node_material.clone(),
                    transform: Transform::from_xyz(
                        NODE_SIZE_X / 2. + close_node_size.x / 2.,
                        close_node_size.y / 2.,
                        0.,
                    ),
                    ..Default::default()
                })
                .insert(Name::new(format!("{}_node_close_button", node_name)))
                .insert(InteractionBox::new(
                    close_node_size,
                    BoxInteraction::DeleteNode(node_id.clone()),
                ));
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
                        BoxInteraction::ConnectionEnd(ConnectionTo::ToNode {
                            id: node_id.clone(),
                            field: field_name.clone(),
                        }),
                    ))
                    .insert(NodeInput { connected_to: None });
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
                        BoxInteraction::ConnectionStart(Connection::NodeConnection {
                            node_id: node_id.clone(),
                            field_name,
                        }),
                    ))
                    .insert(NodeOutput);
                }
            })
            .id()
    }
}
