use crate::{Connection, NativeType};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputField {
    pub glsl_type: NativeType,
    #[serde(default)]
    pub tolerant: bool,
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
    pub const fn glsl_type(&self) -> NativeType {
        self.glsl_type
    }

    pub const fn new(glsl_type: NativeType) -> Self {
        Self {
            glsl_type,
            tolerant: false,
            connection: None,
        }
    }

    pub const fn new_tolerant(glsl_type: NativeType) -> Self {
        Self {
            glsl_type,
            tolerant: true,
            connection: None,
        }
    }
}

impl Input {
    pub const fn none() -> Self {
        Self { fields: vec![] }
    }
}
