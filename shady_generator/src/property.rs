use bevy::utils::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Property<T> {
    pub name: String,
    pub reference: String,
    pub default_value: T,
    pub exposed: bool,
}

// impl<T: ShaderValue> Default for Property<T> {
//     fn default() -> Self {
//         Self {
//             name: T::name().to_string(),
//             reference: format!("{}_{}", T::name(), Uuid::new_v4()),
//             default_value: T::default(),
//             exposed: false,
//         }
//     }
// }
