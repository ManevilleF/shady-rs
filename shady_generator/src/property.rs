use crate::glsl::{AsGlslPrimitiveType, GlslType};
use bevy::utils::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Property {
    pub name: String,
    pub reference: String,
    pub glsl_type: GlslType,
    pub exposed: bool,
}

impl Property {
    pub fn new(glsl_type: GlslType, exposed: bool) -> Self {
        let name = glsl_type.get_glsl_type().to_string();
        Self {
            reference: format!("{}_{}", name.clone(), Uuid::new_v4()),
            name,
            glsl_type,
            exposed,
        }
    }
}
