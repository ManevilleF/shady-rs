use crate::components::{BoxInteraction, InteractionBox, ShadyInputSlot, ShadyOutputSlot};
use crate::resources::{GlslTypeMaterials, ShadyAssets};
use bevy::ecs::component::Component;
use bevy::prelude::*;
use bevy::utils::HashMap;
use shady_generator::{Connection, ConnectionTo, InputField, NativeType, OutputFields};
use std::cmp::max;

const NODE_SIZE_X: f32 = 140.;
const NODE_HEADER_SIZE_Y: f32 = 40.;
const SLOT_SIZE: f32 = 15.;
pub const SLOT_STEP: f32 = 30.;

#[derive(Debug)]
pub struct SpawnResponse {
    pub entity: Entity,
    pub input_field_entities: HashMap<String, Entity>,
    pub output_field_entities: HashMap<String, Entity>,
}

#[derive(Debug, Clone)]
pub struct SlotSpawnInfo {
    field: String,
    native_type: NativeType,
    tolerant: bool,
}

impl From<(String, NativeType, bool)> for SlotSpawnInfo {
    fn from((field, native_type, tolerant): (String, NativeType, bool)) -> Self {
        Self {
            field,
            native_type,
            tolerant,
        }
    }
}

impl From<(String, NativeType)> for SlotSpawnInfo {
    fn from((field, native_type): (String, NativeType)) -> Self {
        Self {
            field,
            native_type,
            tolerant: false,
        }
    }
}

impl From<(String, InputField)> for SlotSpawnInfo {
    fn from((field, input): (String, InputField)) -> Self {
        Self {
            field,
            native_type: input.glsl_type,
            tolerant: input.tolerant,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SpawnType {
    Node {
        input_fields: Vec<SlotSpawnInfo>,
        output_fields: OutputFields,
    },
    InputProperty {
        output_fields: Vec<(String, NativeType)>,
    },
    Constant {
        output_fields: Vec<(String, NativeType)>,
    },
    OutputProperty {
        input_fields: Vec<SlotSpawnInfo>,
    },
}

impl SpawnType {
    pub fn max_field_len(&self) -> usize {
        match self {
            SpawnType::Node {
                input_fields,
                output_fields,
            } => max(input_fields.len(), output_fields.len()),
            SpawnType::InputProperty { output_fields } | SpawnType::Constant { output_fields } => {
                output_fields.len()
            }
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
                    color: assets.node_title_text_color,
                    font_size: 20.,
                },
            }],
            alignment: TextAlignment {
                vertical: VerticalAlign::Top,
                horizontal: HorizontalAlign::Center,
            },
        },
        transform: Transform::from_xyz(0., 0., 1.),
        ..Default::default()
    }
}

fn secondary_text_bundle(value: &str, assets: &ShadyAssets) -> Text2dBundle {
    Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value: value.to_string(),
                style: TextStyle {
                    font: assets.font.clone(),
                    color: assets.node_id_text_color,
                    font_size: 15.,
                },
            }],
            alignment: TextAlignment {
                vertical: VerticalAlign::Bottom,
                horizontal: HorizontalAlign::Center,
            },
        },
        transform: Transform::from_xyz(0., 0., 1.),
        ..Default::default()
    }
}

fn slot_text_bundle(value: String, assets: &ShadyAssets) -> Text2dBundle {
    Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value,
                style: TextStyle {
                    font: assets.font.clone(),
                    color: assets.slot_text_color,
                    font_size: 15.,
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

fn spawn_slots<F, CF, C>(
    cmd: &mut ChildBuilder,
    fields: Vec<SlotSpawnInfo>,
    (size, pos_x): (Vec2, f32),
    assets: &ShadyAssets,
    box_interaction: F,
    component: CF,
    use_field_name: bool,
) -> HashMap<String, Entity>
where
    F: Fn(&str) -> BoxInteraction,
    CF: Fn(Color) -> C,
    C: Component + Clone,
{
    let mut res = HashMap::default();
    for (i, info) in fields.into_iter().enumerate() {
        let entity = cmd
            .spawn_bundle(SpriteBundle {
                sprite: Sprite::new(size),
                material: if info.tolerant {
                    assets.tolerant_slot_material.clone()
                } else {
                    assets.glsl_type_material(info.native_type)
                },
                transform: Transform::from_xyz(
                    pos_x,
                    -NODE_HEADER_SIZE_Y - (SLOT_STEP * i as f32),
                    0.,
                ),
                ..Default::default()
            })
            .insert(Name::new(format!("{} input", info.field)))
            .insert(InteractionBox::new(size, box_interaction(&info.field)))
            .insert(component(GlslTypeMaterials::glsl_type_color(
                info.native_type,
            )))
            .with_children(|builder| {
                builder.spawn_bundle(Text2dBundle {
                    transform: Transform::from_xyz(-pos_x.signum() * SLOT_SIZE * 2., 0., 1.),
                    ..slot_text_bundle(
                        if use_field_name {
                            info.field.clone()
                        } else {
                            info.native_type.to_string()
                        },
                        assets,
                    )
                });
            })
            .id();
        res.insert(info.field, entity);
    }
    res
}

pub fn spawn_element(
    commands: &mut Commands,
    assets: &ShadyAssets,
    pos: Vec2,
    (id, name): (&str, &str),
    spawn_type: SpawnType,
) -> SpawnResponse {
    let header_size = Vec2::new(NODE_SIZE_X, NODE_HEADER_SIZE_Y);
    let close_button_size = Vec2::splat(NODE_HEADER_SIZE_Y / 2.);
    let slot_size = Vec2::splat(SLOT_SIZE);
    let slot_x_pos = NODE_SIZE_X / 2. - SLOT_SIZE;
    let field_len = spawn_type.max_field_len() as f32;
    let body_size = Vec2::new(NODE_SIZE_X, field_len * SLOT_STEP);
    let mut input_field_entities = HashMap::default();
    let mut output_field_entities = HashMap::default();
    let entity = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(header_size),
            material: match spawn_type {
                SpawnType::Node { .. } => assets.node_title_material.clone(),
                SpawnType::InputProperty { .. } => assets.input_property_title_material.clone(),
                SpawnType::OutputProperty { .. } => assets.output_property_title_material.clone(),
                SpawnType::Constant { .. } => assets.constant_title_material.clone(),
            },
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
                .spawn_bundle(secondary_text_bundle(id, assets))
                .insert(Name::new(format!("{} ref", id)));
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
                    material: assets.delete_icon_material.clone(),
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
                    input_field_entities = spawn_slots(
                        &mut builder,
                        input_fields,
                        (slot_size, -slot_x_pos),
                        assets,
                        |f| {
                            BoxInteraction::ConnectionEnd(ConnectionTo::Node {
                                id: id.to_string(),
                                field_name: f.to_string(),
                            })
                        },
                        ShadyInputSlot::new,
                        true,
                    );
                    output_field_entities = spawn_slots(
                        &mut builder,
                        output_fields
                            .field_names()
                            .into_iter()
                            .map(Into::into)
                            .collect(),
                        (slot_size, slot_x_pos),
                        assets,
                        |f| {
                            BoxInteraction::ConnectionStart(match output_fields {
                                OutputFields::SingleOutput(_) => {
                                    Connection::SingleOutputNode { id: id.to_string() }
                                }
                                OutputFields::Fields(_) => Connection::ComplexOutputNode {
                                    id: id.to_string(),
                                    field_name: f.to_string(),
                                },
                            })
                        },
                        ShadyOutputSlot::new,
                        true,
                    );
                }
                SpawnType::Constant { output_fields } => {
                    close_button.insert(InteractionBox::new(
                        close_button_size,
                        BoxInteraction::DeleteConstant(id.to_string()),
                    ));
                    output_field_entities = spawn_slots(
                        &mut builder,
                        output_fields.into_iter().map(Into::into).collect(),
                        (slot_size, slot_x_pos),
                        assets,
                        |_f| {
                            BoxInteraction::ConnectionStart(Connection::Constant {
                                id: id.to_string(),
                            })
                        },
                        ShadyOutputSlot::new,
                        false,
                    );
                }
                SpawnType::InputProperty { output_fields } => {
                    close_button.insert(InteractionBox::new(
                        close_button_size,
                        BoxInteraction::DeleteInput(id.to_string()),
                    ));
                    output_field_entities = spawn_slots(
                        &mut builder,
                        output_fields.into_iter().map(Into::into).collect(),
                        (slot_size, slot_x_pos),
                        assets,
                        |_f| {
                            BoxInteraction::ConnectionStart(Connection::InputProperty {
                                id: id.to_string(),
                            })
                        },
                        ShadyOutputSlot::new,
                        false,
                    );
                }
                SpawnType::OutputProperty { input_fields } => {
                    close_button.insert(InteractionBox::new(
                        close_button_size,
                        BoxInteraction::DeleteOutput(id.to_string()),
                    ));
                    input_field_entities = spawn_slots(
                        &mut builder,
                        input_fields,
                        (slot_size, -slot_x_pos),
                        assets,
                        |_f| {
                            BoxInteraction::ConnectionEnd(ConnectionTo::OutputProperty {
                                id: id.to_string(),
                            })
                        },
                        ShadyInputSlot::new,
                        false,
                    );
                }
            }
        })
        .id();
    SpawnResponse {
        entity,
        input_field_entities,
        output_field_entities,
    }
}
