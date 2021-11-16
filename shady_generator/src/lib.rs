// TODO: enable
// #![deny(warnings)]
// #![forbid(missing_docs)]
// #![forbid(unsafe_code)]

pub use {error::*, glsl_type::GlslType, graphic_library::GraphicLibrary, node::*, shader::*};

mod error;
mod glsl_type;
mod graphic_library;
mod node;
mod shader;

use uuid::Uuid;

#[macro_use]
extern crate indoc;

pub(crate) fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub(crate) fn ordered_map<S, T>(
    value: &std::collections::HashMap<String, T>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: serde::Serialize,
{
    use serde::Serialize;

    let ordered: std::collections::BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

#[cfg(test)]
mod tests {
    #[ctor::ctor]
    fn init() {
        env_logger::init();
    }
}
