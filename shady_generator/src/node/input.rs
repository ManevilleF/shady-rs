use crate::glsl::GlslType;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionData {
    pub var_name: String,
    pub field_name: String,
    pub glsl_type: GlslType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputField {
    ExpectedValue(GlslType),
    Connected(ConnectionData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    pub fields: Vec<(String, InputField)>,
}

impl Deref for Input {
    type Target = Vec<(String, InputField)>;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

impl InputField {
    pub fn glsl_type(&self) -> GlslType {
        match self {
            InputField::ExpectedValue(t) => *t,
            InputField::Connected(c) => c.glsl_type,
        }
    }
}

impl ConnectionData {
    pub fn new(var_name: &str, field_name: &str, glsl_type: GlslType) -> Self {
        Self {
            var_name: var_name.to_string(),
            field_name: field_name.to_string(),
            glsl_type,
        }
    }

    pub fn linked_var_name(&self) -> String {
        format!("{}.{}", self.var_name, self.field_name)
    }
}

impl Input {
    pub fn none() -> Self {
        Self { fields: vec![] }
    }
}
