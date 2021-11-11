mod error;
mod generator;
mod glsl;
mod graphic_library;
mod node;
mod property;
mod shader;
mod shader_operation;
mod shader_type;
mod value;

use uuid::Uuid;

#[macro_use]
extern crate indoc;

pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}
