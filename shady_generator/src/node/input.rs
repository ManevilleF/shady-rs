use crate::node::Connection;
use crate::NativeType;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputField {
    // TODO: Add enum for specific Glsltype selection or a more global type allowing conversion
    // For example, if it accepts all scalar types you may convert it like `float(bool)`
    pub glsl_type: NativeType,
    pub connection: Option<Connection>,
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
    pub fn glsl_type(&self) -> NativeType {
        self.glsl_type
    }

    pub fn new(glsl_type: NativeType) -> Self {
        Self {
            glsl_type,
            connection: None,
        }
    }
}

impl Input {
    pub fn none() -> Self {
        Self { fields: vec![] }
    }
}
