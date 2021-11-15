pub use {connection::*, input::*, output::*, presets::*};

use crate::error::ShadyError;
use crate::glsl::GlslType;
use serde::{Deserialize, Serialize};

mod connection;
mod input;
mod output;
mod presets;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Node {
    pub name: String,
    pub uuid: String,
    pub input_param: Input,
    pub output_param: Output,
    // TODO: Add native operations possibilities (Add/Sub/Div/Etc)
    pub glsl_function: String,
}

impl Node {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn unique_name(&self) -> String {
        format!("{}_{}", self.name, self.uuid)
    }

    fn find_input_field_pos(&self, field: &str) -> Result<usize, ShadyError> {
        let field_pos = self
            .input_param
            .fields
            .iter()
            .position(|(key, _f)| key == field)
            .ok_or_else(|| ShadyError::WrongFieldKey(field.to_string()))?;
        Ok(field_pos)
    }

    fn find_output_field_pos(&self, field: &str) -> Result<usize, ShadyError> {
        let field_pos = self
            .output_param
            .fields()
            .iter()
            .position(|(key, _f)| key == field)
            .ok_or_else(|| ShadyError::WrongFieldKey(field.to_string()))?;
        Ok(field_pos)
    }

    pub fn get_input_field(&self, field: &str) -> Option<GlslType> {
        let pos = self.find_input_field_pos(field).ok()?;
        let (_k, f) = self.input_param.fields.get(pos)?;
        Some(f.glsl_type())
    }

    pub fn get_output_field(&self, field: &str) -> Option<GlslType> {
        let pos = self.find_output_field_pos(field).ok()?;
        let fields = self.output_param.fields();
        let (_k, f) = fields.get(pos)?;
        Some(*f)
    }

    pub fn input_fields(&self) -> Vec<(String, InputField)> {
        self.input_param.fields.clone()
    }

    pub fn output_fields(&self) -> Vec<(String, GlslType)> {
        self.output_param.fields()
    }

    pub fn node_connections(&self) -> Vec<String> {
        self.input_param
            .fields
            .iter()
            .filter_map(|(_, f)| match f.connection.as_ref()? {
                Connection::PropertyConnection { .. } => None,
                Connection::NodeConnection { node_id, .. } => Some(node_id.clone()),
            })
            .collect()
    }

    pub fn struct_declaration(&self) -> Option<String> {
        self.output_param.custom_declaration()
    }

    pub fn connect_input(
        &mut self,
        target_field: &str,
        connect_message: ConnectionMessage,
    ) -> Result<ConnectionResponse, ShadyError> {
        // Same connection check
        if let Connection::NodeConnection { node_id, .. } = &connect_message.connection {
            if node_id == &self.uuid {
                return Err(ShadyError::SameNodeConnection(node_id.clone()));
            }
        }
        let field_pos = self.find_input_field_pos(target_field)?;
        let (_key, field) = self
            .input_param
            .fields
            .get_mut(field_pos)
            .ok_or_else(|| ShadyError::WrongFieldKey(target_field.to_string()))?;
        let expected_type = field.glsl_type();
        if connect_message.glsl_type != expected_type {
            return Err(ShadyError::WrongGlslType {
                input_type: connect_message.glsl_type,
                expected_type,
            });
        }
        Ok(field.connection.replace(connect_message.connection))
    }

    pub fn disconnect_field(&mut self, field_name: &str) -> Result<Option<Connection>, ShadyError> {
        let field_pos = self.find_input_field_pos(field_name)?;
        let (_key, field) = self
            .input_param
            .fields
            .get_mut(field_pos)
            .ok_or_else(|| ShadyError::WrongFieldKey(field_name.to_string()))?;
        Ok(field.connection.take())
    }

    pub fn to_glsl(&self) -> String {
        let mut buffer = format!(
            "{} {} = {}(",
            self.output_param.glsl_type(),
            self.uuid,
            self.glsl_function
        );
        let len = self.input_param.len();
        for (i, (field, val)) in self.input_param.fields.iter().enumerate() {
            let val = match &val.connection {
                Some(connection) => connection.glsl_call(),
                None => {
                    log::warn!(
                        "No connection set for Node {}::{}. Using default value",
                        self.unique_name(),
                        field
                    );
                    val.glsl_type.default_glsl_value().to_string()
                }
            };
            buffer = format!("{}{}", buffer, val);
            if i < len - 1 {
                buffer = format!("{}, ", buffer)
            }
        }
        format!("{}); // {} Node", buffer, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_vec2_node() {
        let mut node = NodePreset::Vec2.get_node();
        node.connect_input(
            "x",
            ConnectionMessage {
                connection: Connection::NodeConnection {
                    node_id: "some_var".to_string(),
                    field_name: "a".to_string(),
                },
                glsl_type: GlslType::Float,
            },
        )
        .unwrap();
        node.connect_input(
            "y",
            ConnectionMessage {
                connection: Connection::NodeConnection {
                    node_id: "other_var".to_string(),
                    field_name: "z".to_string(),
                },
                glsl_type: GlslType::Float,
            },
        )
        .unwrap();
        let res = node.to_glsl();
        assert_eq!(
            res,
            format!(
                "vec2 {} = vec2(some_var.a, other_var.z); // Vec2 Node",
                node.uuid
            )
        );
    }
}
