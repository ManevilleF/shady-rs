//! # Shady Generator
//!
//! Shader generation lib for shady-rs
#![deny(warnings)]
// #![forbid(missing_docs)]
#![forbid(unsafe_code)]

pub use {error::*, graphic_library::*, native_type::*, node::*, shader::*};

mod error;
mod graphic_library;
mod native_type;
mod node;
mod shader;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

const UNIQUE_ID_LENGTH: usize = 10;

#[macro_use]
extern crate indoc;

// TODO: Check entropy of this and look for a better solution
pub(crate) fn generate_uuid() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(UNIQUE_ID_LENGTH)
        .map(char::from)
        .collect()
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
        std::env::set_var("CUSTOM_FUNCTIONS_PATH", "test");
        env_logger::init();
    }
}
