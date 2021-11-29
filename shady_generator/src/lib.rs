//! # Shady Generator
//!
//! Shader generation lib for shady-rs
#![deny(warnings)]
// #![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::fallible_impl_from,
    clippy::filter_map_next,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::if_let_mutex,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::map_flatten,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_pass_by_value,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::todo,
    clippy::trait_duplication_in_bounds,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unused_self,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
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
