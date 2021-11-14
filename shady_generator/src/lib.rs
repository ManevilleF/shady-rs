// TODO: enable
// #![deny(warnings)]
// #![forbid(missing_docs)]
// #![forbid(unsafe_code)]

// TODO protection levels
pub mod error;
pub mod generator;
pub mod glsl;
pub mod graphic_library;
pub mod node;
pub mod property;
pub mod shader;
pub mod shader_operation;
pub mod shader_type;
pub mod value;

use uuid::Uuid;

#[macro_use]
extern crate indoc;

pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}
