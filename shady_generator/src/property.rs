use crate::generate_uuid;
use crate::glsl::GlslType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InputProperty {
    pub name: String,
    pub reference: String,
    pub glsl_type: GlslType,
    pub uniform: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutputProperty {
    pub name: String,
    pub reference: String,
    pub glsl_type: GlslType,
}

impl InputProperty {
    pub fn new(glsl_type: GlslType, uniform: bool) -> Self {
        let name = glsl_type.get_glsl_type().to_string();
        Self {
            reference: format!("{}_{}", name.clone(), generate_uuid()),
            name,
            glsl_type,
            uniform,
        }
    }

    // TODO Add default OpenGL/ES properties (must match version)
}

impl OutputProperty {
    pub fn new(glsl_type: GlslType) -> Self {
        let name = glsl_type.get_glsl_type().to_string();
        Self {
            reference: format!("{}_{}", name.clone(), generate_uuid()),
            name,
            glsl_type,
        }
    }

    // TODO Add default OpenGL/ES properties (must match version)
}
