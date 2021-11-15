// TODO: enable
// #![deny(warnings)]
// #![forbid(missing_docs)]
// #![forbid(unsafe_code)]

// TODO protection levels
pub mod error;
pub mod glsl;
pub mod graphic_library;
pub mod node;
pub mod shader;

use uuid::Uuid;

#[macro_use]
extern crate indoc;

pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}
