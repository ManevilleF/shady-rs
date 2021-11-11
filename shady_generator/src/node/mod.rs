pub use {connector::*, input::*, output::*, presets::*};

use crate::error::ShadyError;
use crate::glsl::GlslType;
use crate::shader::Shader;
use serde::{Deserialize, Serialize};

mod connector;
mod input;
mod output;
mod presets;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Node {
    pub name: String,
    pub uuid: String,
    pub input_param: Input,
    pub output_param: Output,
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
        self.output_param.fields().clone()
    }

    pub fn output_var_name(&self) -> String {
        format!("{}_{}", self.output_param.glsl_type(), self.uuid)
    }

    pub fn connect_input(
        &mut self,
        target_field: &str,
        connect_message: ConnectionMessage,
    ) -> Result<ConnectionResponse, ShadyError> {
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
        let response = ConnectionResponse {
            connector_id: field.connector_id.replace(connect_message.connector_id),
        };
        Ok(response)
    }

    pub fn to_glsl(&self, shader: &Shader) -> Result<String, ShadyError> {
        let mut buffer = format!(
            "{} {} = {}(",
            self.output_param.glsl_type(),
            self.output_var_name(),
            self.glsl_function
        );
        let len = self.input_param.len();
        for (i, (field, val)) in self.input_param.fields.iter().enumerate() {
            let val = match &val.connector_id {
                Some(connector_id) => match shader.connectors.get(connector_id) {
                    None => {
                        log::error!(
                            "Connector {} for Node {}::{} not found. Using default value.",
                            connector_id,
                            self.unique_name(),
                            field
                        );
                        val.glsl_type.default_glsl_value().to_string()
                    }
                    Some(c) => match &c.from {
                        Connection::PropertyConnection { property_id } => property_id.clone(),
                        Connection::NodeConnection {
                            node_id,
                            field_name,
                        } => {
                            let val = match shader.nodes.get(node_id) {
                                None => {
                                    log::error!("Node {} not found. Using default value.", node_id);
                                    val.glsl_type.default_glsl_value().to_string()
                                }
                                Some(node) => match node.get_output_field(&field_name) {
                                    None => {
                                        log::error!("Output field {} not found on Node {} Using default value.", field_name, node.unique_name());
                                        val.glsl_type.default_glsl_value().to_string()
                                    }
                                    Some(glsl_ype) => {
                                        if val.glsl_type != glsl_ype {
                                            log::error!("Node {} field {} type `{}` does not match Node {} field {}. Expected `{}`", node.unique_name(), field_name, glsl_ype, self.unique_name(), field, val.glsl_type);
                                            val.glsl_type.default_glsl_value().to_string()
                                        } else {
                                            format!("{}.{}", node_id, field_name)
                                        }
                                    }
                                },
                            };
                            val
                        }
                    },
                },
                None => {
                    log::warn!(
                        "No connector set for Node {}::{}. Using default value",
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
        Ok(format!("{});", buffer))
    }
}
