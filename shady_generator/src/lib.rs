// TODO: enable
// #![deny(warnings)]
// #![forbid(missing_docs)]
// #![forbid(unsafe_code)]

pub use {glsl_type::GlslType, graphic_library::GraphicLibrary, node::*, shader::*};

mod error;
mod glsl_type;
mod graphic_library;
mod node;
mod shader;

use uuid::Uuid;

#[macro_use]
extern crate indoc;

pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    #[ctor::ctor]
    fn init() {
        env_logger::init();
    }
}
