use crate::error::ShadyError;
use crate::generate_uuid;
use crate::glsl::GlslType;
use crate::node::{Connection, ConnectionMessage, ConnectionResponse};
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
    pub connection: Option<Connection>,
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
            connection: None,
        }
    }

    pub fn connect_input(
        &mut self,
        connect_message: ConnectionMessage,
    ) -> Result<ConnectionResponse, ShadyError> {
        if connect_message.glsl_type != self.glsl_type {
            return Err(ShadyError::WrongGlslType {
                input_type: connect_message.glsl_type,
                expected_type: self.glsl_type,
            });
        }
        Ok(self.connection.replace(connect_message.connection))
    }

    // TODO Add default OpenGL/ES properties (must match version)
}
