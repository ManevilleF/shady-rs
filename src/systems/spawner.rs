use crate::components::{BoxInteraction, InteractionBox, ShadyInputSlot, ShadyOutputSlot};
use crate::resources::ShadyAssets;
use bevy::prelude::*;
use shady_generator::{Connection, ConnectionTo, GlslType};
use std::cmp::max;

const NODE_SIZE_X: f32 = 120.;
const NODE_HEADER_SIZE_Y: f32 = 30.;
const SLOT_SIZE: f32 = 15.;
const SLOT_STEP: f32 = 40.;

#[derive(Debug, Clone)]
pub enum SpawnType {
    Node {
        input_fields: Vec<(String, GlslType)>,
        output_fields: Vec<(String, GlslType)>,
    },
    InputProperty {
        output_fields: Vec<(String, GlslType)>,
    },
    OutputProperty {
        input_fields: Vec<(String, GlslType)>,
    },
}

impl SpawnType {
    pub fn max_field_len(&self) -> usize {
        match self {
            SpawnType::Node {
                input_fields,
                output_fields,
            } => max(input_fields.len(), output_fields.len()),
            SpawnType::InputProperty { output_fields } => output_fields.len(),
            SpawnType::OutputProperty { input_fields } => input_fields.len(),
        }
    }
}

fn title_text_bundle(value: &str, assets: &ShadyAssets) -> Text2dBundle {
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

fn spawn_input_slot(
    cmd: &mut ChildBuilder,
    fields: Vec<(String, GlslType)>,
    (size, pos_x): (Vec2, f32),
    id: &str,
    assets: &ShadyAssets,
    property: bool,
) {
    for (i, (field_name, field)) in fields.into_iter().enumerate() {
        cmd.spawn_bundle(SpriteBundle {
            sprite: Sprite::new(size),
            material: assets.glsl_type_material(field),
            transform: Transform::from_xyz(
                -pos_x,
                -NODE_HEADER_SIZE_Y - (SLOT_STEP * i as f32),
                0.,
            ),
            ..Default::default()
        })
        .insert(Name::new(format!("{} input", field_name)))
        .insert(InteractionBox::new(
            size,
            BoxInteraction::ConnectionEnd(if property {
                ConnectionTo::OutputProperty { id: id.to_string() }
            } else {
                ConnectionTo::Node {
                    node_id: id.to_string(),
                    field_name,
                }
            }),
        ))
        .insert(ShadyInputSlot { connected_to: None });
    }
}

fn spawn_output_slot(
    cmd: &mut ChildBuilder,
    fields: Vec<(String, GlslType)>,
    (size, pos_x): (Vec2, f32),
    id: &str,
    assets: &ShadyAssets,
    property: bool,
) {
    for (i, (field_name, field)) in fields.into_iter().enumerate() {
        cmd.spawn_bundle(SpriteBundle {
            sprite: Sprite::new(size),
            material: assets.glsl_type_material(field),
            transform: Transform::from_xyz(pos_x, -NODE_HEADER_SIZE_Y - (SLOT_STEP * i as f32), 0.),
            ..Default::default()
        })
        .insert(Name::new(format!("{} output", field_name)))
        .insert(InteractionBox::new(
            size,
            BoxInteraction::ConnectionStart(if property {
                Connection::InputProperty {
                    property_id: id.to_string(),
                }
            } else {
                Connection::Node {
                    node_id: id.to_string(),
                    field_name,
                }
            }),
        ))
        .insert(ShadyOutputSlot);
    }
}

pub fn spawn_element(
    commands: &mut Commands,
    assets: &ShadyAssets,
    pos: Vec2,
    (id, name): (&str, &str),
    spawn_type: SpawnType,
) -> Entity {
    let header_size = Vec2::new(NODE_SIZE_X, NODE_HEADER_SIZE_Y);
    let close_button_size = Vec2::splat(NODE_HEADER_SIZE_Y / 2.);
    let slot_size = Vec2::splat(SLOT_SIZE);
    let slot_x_pos = NODE_SIZE_X / 2. - SLOT_SIZE;
    let field_len = spawn_type.max_field_len() as f32;
    let body_size = Vec2::new(NODE_SIZE_X, field_len * SLOT_STEP);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(header_size),
            material: assets.node_title_material.clone(),
            transform: Transform::from_xyz(pos.x, pos.y, 0.),
            ..Default::default()
        })
        .insert(Name::new(format!("{} node: {}", name, id)))
        .insert(InteractionBox::new(header_size, BoxInteraction::Drag))
        .with_children(|mut builder| {
            builder
                .spawn_bundle(title_text_bundle(name, assets))
                .insert(Name::new(format!("{} title", name)));
            builder
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite::new(body_size),
                    material: assets.node_body_material.clone(),
                    transform: Transform::from_xyz(0., -header_size.y / 2. - body_size.y / 2., 0.),
                    ..Default::default()
                })
                .insert(Name::new(format!("{} body", name)))
                .insert(InteractionBox::new(body_size, BoxInteraction::Ignore));
            let mut close_button = builder.spawn();
            close_button
                .insert_bundle(SpriteBundle {
                    sprite: Sprite::new(close_button_size),
                    material: assets.close_node_material.clone(),
                    transform: Transform::from_xyz(
                        NODE_SIZE_X / 2. + close_button_size.x / 2.,
                        close_button_size.y / 2.,
                        0.,
                    ),
                    ..Default::default()
                })
                .insert(Name::new(format!("{} close button", name)));
            match spawn_type {
                SpawnType::Node {
                    input_fields,
                    output_fields,
                } => {
                    close_button.insert(InteractionBox::new(
                        close_button_size,
                        BoxInteraction::DeleteNode(id.to_string()),
                    ));
                    spawn_input_slot(
                        &mut builder,
                        input_fields,
                        (slot_size, slot_x_pos),
                        id,
                        assets,
                        false,
                    );
                    spawn_output_slot(
                        &mut builder,
                        output_fields,
                        (slot_size, slot_x_pos),
                        id,
                        assets,
                        false,
                    );
                }
                SpawnType::InputProperty { output_fields } => {
                    close_button.insert(InteractionBox::new(
                        close_button_size,
                        BoxInteraction::DeleteInput(id.to_string()),
                    ));
                    spawn_output_slot(
                        &mut builder,
                        output_fields,
                        (slot_size, slot_x_pos),
                        id,
                        assets,
                        true,
                    );
                }
                SpawnType::OutputProperty { input_fields } => {
                    close_button.insert(InteractionBox::new(
                        close_button_size,
                        BoxInteraction::DeleteOutput(id.to_string()),
                    ));
                    spawn_input_slot(
                        &mut builder,
                        input_fields,
                        (slot_size, slot_x_pos),
                        id,
                        assets,
                        true,
                    );
                }
            }
        })
        .id()
}
