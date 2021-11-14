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
    // TODO: handle constants
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

    pub fn glsl_declaration(&self) -> String {
        format!(
            "{} {} {}; // {}",
            if self.uniform { "uniform " } else { "in " },
            self.glsl_type.get_glsl_type(),
            self.reference,
            self.name
        )
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

    pub fn disconnect(&mut self) -> Option<Connection> {
        self.connection.take()
    }

    pub fn glsl_declaration(&self) -> String {
        format!(
            "out {} {}; // {}",
            self.glsl_type.get_glsl_type(),
            self.reference,
            self.name
        )
    }

    pub fn to_glsl(&self) -> String {
        format!(
            "{} = {}; // {}",
            self.reference,
            match &self.connection {
                None => {
                    log::warn!(
                        "No connection set for output property {} ({}). Using default value",
                        self.name,
                        self.reference
                    );
                    self.glsl_type.default_glsl_value().to_string()
                }
                Some(connection) => connection.glsl_call(),
            },
            self.name
        )
    }

    // TODO Add default OpenGL/ES properties (must match version)
}

#[cfg(tests)]
mod tests {}
