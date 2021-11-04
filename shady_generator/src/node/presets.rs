use crate::glsl::GlslType;
use crate::node::*;
use bevy::utils::Uuid;

#[derive(Debug, Copy, Clone)]
pub enum NodePresets {
    Vec2,
    Vec3,
    Vec4,
    FloatAdd,
    // FloatMultiply,
}

impl NodePresets {
    pub fn get_node(&self) -> Node {
        match self {
            NodePresets::Vec2 => Node {
                name: "Vec2".to_string(),
                uuid: Uuid::new_v4().to_string(),
                input_param: Input {
                    fields: vec![
                        ("x".to_string(), InputField::ExpectedValue(GlslType::Float)),
                        ("y".to_string(), InputField::ExpectedValue(GlslType::Float)),
                    ],
                },
                output_param: Output::GlslType {
                    glsl_type: GlslType::Vec2,
                    field_name: "vec2".to_string(),
                },
                glsl_function: "vec2".to_string(),
            },
            NodePresets::Vec3 => Node {
                name: "Vec3".to_string(),
                uuid: Uuid::new_v4().to_string(),
                input_param: Input {
                    fields: vec![
                        ("x".to_string(), InputField::ExpectedValue(GlslType::Float)),
                        ("y".to_string(), InputField::ExpectedValue(GlslType::Float)),
                        ("z".to_string(), InputField::ExpectedValue(GlslType::Float)),
                    ],
                },
                output_param: Output::GlslType {
                    glsl_type: GlslType::Vec3,
                    field_name: "vec3".to_string(),
                },
                glsl_function: "vec3".to_string(),
            },
            NodePresets::Vec4 => Node {
                name: "Vec4".to_string(),
                uuid: Uuid::new_v4().to_string(),
                input_param: Input {
                    fields: vec![
                        ("x".to_string(), InputField::ExpectedValue(GlslType::Float)),
                        ("y".to_string(), InputField::ExpectedValue(GlslType::Float)),
                        ("z".to_string(), InputField::ExpectedValue(GlslType::Float)),
                        ("w".to_string(), InputField::ExpectedValue(GlslType::Float)),
                    ],
                },
                output_param: Output::GlslType {
                    glsl_type: GlslType::Vec4,
                    field_name: "vec4".to_string(),
                },
                glsl_function: "vec4".to_string(),
            },
            NodePresets::FloatAdd => Node {
                name: "Add Float".to_string(),
                uuid: Uuid::new_v4().to_string(),
                input_param: Input {
                    fields: vec![
                        ("a".to_string(), InputField::ExpectedValue(GlslType::Float)),
                        ("b".to_string(), InputField::ExpectedValue(GlslType::Float)),
                    ],
                },
                output_param: Output::GlslType {
                    glsl_type: GlslType::Float,
                    field_name: "v".to_string(),
                },
                glsl_function: "float_add".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_vec2_node() {
        let node = NodePresets::Vec2.get_node();
        let res = node.to_glsl();
        assert_eq!(
            res,
            format!("vec2 {} = vec2(0.0, 0.0);", node.output_var_name())
        );
    }

    #[test]
    fn custom_vec2_node() {
        let mut node = NodePresets::Vec2.get_node();
        node.connect_input("x", ConnectionData::new("some_var", "a", GlslType::Float))
            .unwrap();
        node.connect_input("y", ConnectionData::new("other_var", "z", GlslType::Float))
            .unwrap();
        let res = node.to_glsl();
        assert_eq!(
            res,
            format!(
                "vec2 {} = vec2(some_var.a, other_var.z);",
                node.output_var_name()
            )
        );
    }

    #[test]
    fn default_vec3_node() {
        let node = NodePresets::Vec3.get_node();
        let res = node.to_glsl();
        assert_eq!(
            res,
            format!("vec3 {} = vec3(0.0, 0.0, 0.0);", node.output_var_name())
        );
    }

    #[test]
    fn default_vec4_node() {
        let node = NodePresets::Vec4.get_node();
        let res = node.to_glsl();
        assert_eq!(
            res,
            format!(
                "vec4 {} = vec4(0.0, 0.0, 0.0, 0.0);",
                node.output_var_name()
            )
        );
    }
}
