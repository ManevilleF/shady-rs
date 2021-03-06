//! # Shady Generator
//!
//! Shader generation lib for shady-rs
#![deny(warnings)]
// #![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::nursery,
    clippy::pedantic,
    nonstandard_style
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::default_trait_access,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

pub use {
    connection::*, error::*, graphic_library::*, input::*, native_type::*, node::*, output::*,
    shader::*,
};

mod connection;
mod error;
mod graphic_library;
mod input;
mod native_type;
mod node;
pub mod node_operation;
mod output;
mod shader;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const UNIQUE_ID_LENGTH: usize = 10;

#[macro_use]
extern crate indoc;

// TODO: Check entropy of this and look for a better solution
pub(crate) fn generate_unique_id() -> String {
    format!(
        "v{}",
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(UNIQUE_ID_LENGTH)
            .map(char::from)
            .collect::<String>()
    )
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
