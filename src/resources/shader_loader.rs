use crate::components::{LogElement, LogLevel, NodeConnector};
use crate::resources::ShadyAssets;
use crate::systems::spawner::{spawn_element, SpawnResponse, SpawnType};
use crate::CurrentShader;
use bevy::prelude::*;
use bevy::utils::HashMap;
use shady_generator::{Connection, ConnectionTo, OutputFields, Shader};

macro_rules! get_entity_or_continue {
    ($res:expr, $cmd:expr) => {
        match $res {
            Ok(e) => e,
            Err(e) => {
                LogElement::new(LogLevel::Error, e).spawn($cmd);
                continue;
            }
        }
    };
}

#[derive(Debug, Copy, Clone)]
enum SlotType {
    Property,
    Node,
    Constant,
}

#[derive(Debug)]
pub struct ShaderLoader {
    pub shader: Shader,
    pub node_entities: HashMap<String, Entity>,
    pub constants_entities: HashMap<String, Entity>,
    pub input_property_entities: HashMap<String, Entity>,
    pub output_property_entities: HashMap<String, Entity>,
    pub connection_entities: HashMap<String, Entity>,
    pub input_field_entities: HashMap<String, Entity>,
    pub output_field_entities: HashMap<String, Entity>,
}

impl ShaderLoader {
    pub fn new(shader: Shader) -> Self {
        Self {
            shader,
            node_entities: Default::default(),
            constants_entities: Default::default(),
            input_property_entities: Default::default(),
            output_property_entities: Default::default(),
            connection_entities: Default::default(),
            input_field_entities: Default::default(),
            output_field_entities: Default::default(),
        }
    }

    fn unique_slot_id(element_id: &str, slot_id: &str, slot_type: SlotType) -> String {
        format!(
            "{}::{}::{}",
            match slot_type {
                SlotType::Property => "Property",
                SlotType::Node => "Node",
                SlotType::Constant => "Constant",
            },
            element_id,
            slot_id
        )
    }

    fn handle_spawn_response_fields(
        &mut self,
        response: SpawnResponse,
        element_id: &str,
        slot_type: SlotType,
    ) {
        for (key, entity) in response.output_field_entities {
            let id = Self::unique_slot_id(element_id, &key, slot_type);
            self.output_field_entities.insert(id, entity);
        }
        for (key, entity) in response.input_field_entities {
            let id = Self::unique_slot_id(element_id, &key, slot_type);
            self.input_field_entities.insert(id, entity);
        }
    }

    fn get_field_id(&self, id: &str, input: bool) -> Result<Entity, String> {
        if input {
            match self.input_field_entities.get(id) {
                Some(e) => Ok(*e),
                None => Err(format!("Failed to find {} input property. Skipping.", id)),
            }
        } else {
            match self.output_field_entities.get(id) {
                Some(e) => Ok(*e),
                None => Err(format!("Failed to find {} input property. Skipping.", id)),
            }
        }
    }

    pub fn load(&mut self, commands: &mut Commands, assets: &ShadyAssets, pos: Vec2) {
        let mut pos = pos;
        let delta = 200.;
        for (key, property) in self.shader.input_properties().clone() {
            let response = spawn_element(
                commands,
                assets,
                pos,
                (&key, &property.name),
                SpawnType::InputProperty {
                    output_fields: vec![(property.reference.clone(), property.native_type)],
                },
            );
            pos.y -= delta;
            self.input_property_entities
                .insert(key.clone(), response.entity);
            self.handle_spawn_response_fields(response, &key, SlotType::Property);
        }
        pos.x += delta;
        pos.y = 0.;
        for (key, constant) in self.shader.constants().clone() {
            let response = spawn_element(
                commands,
                assets,
                pos,
                (&key, &constant.name),
                SpawnType::Constant {
                    output_fields: vec![(constant.key(), constant.native_type())],
                },
            );
            pos.y -= delta;
            self.constants_entities.insert(key.clone(), response.entity);
            self.handle_spawn_response_fields(response, &key, SlotType::Constant);
        }
        pos.x += delta;
        pos.y = 0.;
        for (key, node) in self.shader.nodes().clone() {
            let response = spawn_element(
                commands,
                assets,
                pos,
                (&key, node.name()),
                SpawnType::Node {
                    input_fields: node.input_fields().into_iter().map(Into::into).collect(),
                    output_fields: node.output_fields(),
                },
            );
            self.node_entities.insert(key.clone(), response.entity);
            self.handle_spawn_response_fields(response, &key, SlotType::Node);
            pos.y -= delta;
        }
        for (key, node) in self.shader.nodes() {
            for (field, connection) in node.connections() {
                let connection_to = ConnectionTo::Node {
                    id: key.to_string(),
                    field_name: field.to_string(),
                };
                let connector_id = CurrentShader::unique_connector_id(&connection_to, connection);
                let from_id = match connection {
                    Connection::InputProperty { id } => {
                        Self::unique_slot_id(id, id, SlotType::Property)
                    }
                    Connection::Constant { id } => Self::unique_slot_id(id, id, SlotType::Constant),
                    Connection::SingleOutputNode { id } => {
                        Self::unique_slot_id(id, OutputFields::SINGLE_FIELD_NAME, SlotType::Node)
                    }
                    Connection::ComplexOutputNode {
                        id: node_id,
                        field_name,
                    } => Self::unique_slot_id(node_id, field_name, SlotType::Node),
                };
                let to_id = Self::unique_slot_id(key, field, SlotType::Node);
                let from = get_entity_or_continue!(self.get_field_id(&from_id, false), commands);
                let to = get_entity_or_continue!(self.get_field_id(&to_id, true), commands);
                let entity = commands
                    .spawn()
                    .insert(NodeConnector {
                        output_from: from,
                        input_to: to,
                    })
                    .insert(Name::new(format!("{} connector", connector_id)))
                    .id();
                self.connection_entities.insert(connector_id, entity);
            }
        }
        pos.x += delta;
        pos.y = 0.;
        for (key, property) in self.shader.output_properties().clone() {
            let response = spawn_element(
                commands,
                assets,
                pos,
                (&key, &property.name),
                SpawnType::OutputProperty {
                    input_fields: vec![
                        (property.reference.clone(), property.native_type, false).into()
                    ],
                },
            );
            self.input_property_entities
                .insert(key.clone(), response.entity);
            self.handle_spawn_response_fields(response, &key, SlotType::Property);
            if let Some(connection) = property.connection() {
                let connection_to = ConnectionTo::OutputProperty { id: key.clone() };
                let connector_id = CurrentShader::unique_connector_id(&connection_to, connection);
                let from_id = match connection {
                    Connection::InputProperty { id } => {
                        Self::unique_slot_id(id, id, SlotType::Property)
                    }
                    Connection::Constant { id } => Self::unique_slot_id(id, id, SlotType::Constant),
                    Connection::SingleOutputNode { id } => {
                        Self::unique_slot_id(id, OutputFields::SINGLE_FIELD_NAME, SlotType::Node)
                    }
                    Connection::ComplexOutputNode {
                        id: node_id,
                        field_name,
                    } => Self::unique_slot_id(node_id, field_name, SlotType::Node),
                };
                let to_id = Self::unique_slot_id(&key, &key, SlotType::Property);
                let from = get_entity_or_continue!(self.get_field_id(&from_id, false), commands);
                let to = get_entity_or_continue!(self.get_field_id(&to_id, true), commands);
                let entity = commands
                    .spawn()
                    .insert(NodeConnector {
                        output_from: from,
                        input_to: to,
                    })
                    .insert(Name::new(format!("{} connector", connector_id)))
                    .id();
                self.connection_entities.insert(connector_id, entity);
            }
            pos.y -= delta;
        }
    }
}
