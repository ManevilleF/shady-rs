use crate::glsl::AsGlslPrimitiveType;
use crate::node::Node;
use crate::property::Property;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Shader {
    pub name: String,
    pub properties: HashMap<String, Property>,
    pub nodes: HashMap<String, Node>,
}
