pub use {input::*, output::*, presets::*};

use crate::error::ShadyError;
use crate::glsl::GlslType;
use serde::{Deserialize, Serialize};

mod input;
mod output;
mod presets;

pub type ConnectResponse = Option<ConnectionData>;

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
        connect_message: ConnectionData,
    ) -> Result<ConnectResponse, ShadyError> {
        let field_pos = self
            .input_param
            .fields
            .iter()
            .position(|(key, _f)| key == target_field)
            .ok_or_else(|| ShadyError::WrongFieldKey(target_field.to_string()))?;
        let (_key, field) = self
            .input_param
            .fields
            .get_mut(field_pos)
            .ok_or_else(|| ShadyError::WrongFieldKey(target_field.to_string()))?;
        let expected_type = field.glsl_type();
        if !matches!(connect_message.glsl_type, expected_type) {
            return Err(ShadyError::WrongGlslType {
                input_type: connect_message.glsl_type,
                expected_type,
            });
        }
        let res = if let InputField::Connected(c) = &field {
            Some(c.clone())
        } else {
            None
        };
        *field = InputField::Connected(connect_message);
        Ok(res)
    }

    pub fn to_glsl(&self) -> String {
        let mut buffer = format!(
            "{} {} = {}(",
            self.output_param.glsl_type(),
            self.output_var_name(),
            self.glsl_function
        );
        let len = self.input_param.len();
        for (i, (_field, val)) in self.input_param.fields.iter().enumerate() {
            let val = match val {
                InputField::ExpectedValue(v) => v.default_glsl_value().to_string(),
                InputField::Connected(c) => c.linked_var_name(),
            };
            buffer = format!("{}{}", buffer, val);
            if i < len - 1 {
                buffer = format!("{}, ", buffer)
            }
        }
        format!("{});", buffer)
    }
}
