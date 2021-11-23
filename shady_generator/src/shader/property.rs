use crate::error::ShadyError;
use crate::generate_unique_id;
use crate::node::{Connection, ConnectionMessage, ConnectionResponse};
use crate::NativeType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InputProperty {
    pub name: String,
    pub reference: String,
    pub glsl_type: NativeType,
    // TODO: handle constants
    pub uniform: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutputProperty {
    pub name: String,
    pub reference: String,
    pub glsl_type: NativeType,
    pub(crate) connection: Option<Connection>,
}

impl InputProperty {
    pub fn new(glsl_type: NativeType, uniform: bool) -> Self {
        let name = glsl_type.get_glsl_type().to_string();
        Self {
            reference: format!("{}_{}", name, generate_unique_id()),
            name,
            glsl_type,
            uniform,
        }
    }

    pub fn glsl_declaration(&self) -> String {
        format!(
            "{} {} {}; // {}",
            if self.uniform { "uniform" } else { "in" },
            self.glsl_type.get_glsl_type(),
            self.reference,
            self.name
        )
    }

    // TODO Add default OpenGL/ES properties (must match version)
}

impl OutputProperty {
    pub fn new(glsl_type: NativeType) -> Self {
        let name = glsl_type.get_glsl_type().to_string();
        Self {
            reference: format!("{}_{}", name, generate_unique_id()),
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
            return Err(ShadyError::WrongNativeType {
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

    pub fn connection(&self) -> Option<&Connection> {
        self.connection.as_ref()
    }

    // TODO Add default OpenGL/ES properties (must match version)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod input {
        use super::*;

        #[test]
        fn prints_to_glsl() {
            let property = InputProperty {
                name: "Property".to_string(),
                reference: "ref".to_string(),
                glsl_type: NativeType::Bool,
                uniform: false,
            };
            let res = property.glsl_declaration();
            assert_eq!(&res, "in bool ref; // Property")
        }

        #[test]
        fn prints_uniform_to_glsl() {
            let property = InputProperty {
                name: "Property".to_string(),
                reference: "ref".to_string(),
                glsl_type: NativeType::Bool,
                uniform: true,
            };
            let res = property.glsl_declaration();
            assert_eq!(&res, "uniform bool ref; // Property")
        }
    }

    mod output {
        use super::*;

        #[test]
        fn prints_to_glsl() {
            let property = OutputProperty {
                name: "Property".to_string(),
                reference: "ref".to_string(),
                glsl_type: NativeType::Bool,
                connection: None,
            };
            let res = property.glsl_declaration();
            assert_eq!(&res, "out bool ref; // Property")
        }
    }
}
