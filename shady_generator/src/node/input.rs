use crate::glsl::GlslType;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputField {
    pub glsl_type: GlslType,
    pub connector_id: Option<String>,
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
        self.glsl_type
    }

    pub fn new(glsl_type: GlslType) -> Self {
        Self {
            glsl_type,
            connector_id: None,
        }
    }
}

impl Input {
    pub fn none() -> Self {
        Self { fields: vec![] }
    }
}
