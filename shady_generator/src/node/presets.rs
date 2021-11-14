use crate::generate_uuid;
use crate::glsl::GlslType;
use crate::node::*;

#[derive(Debug, Copy, Clone)]
pub enum NodePreset {
    Vec2,
    Vec3,
    Vec4,
    FloatAdd,
    // FloatMultiply,
}

impl NodePreset {
    pub fn get_node(&self) -> Node {
        match self {
            NodePreset::Vec2 => Node {
                name: "Vec2".to_string(),
                uuid: generate_uuid(),
                input_param: Input {
                    fields: vec![
                        ("x".to_string(), InputField::new(GlslType::Float)),
                        ("y".to_string(), InputField::new(GlslType::Float)),
                    ],
                },
                output_param: Output::GlslType {
                    glsl_type: GlslType::Vec2,
                    field_name: "vec2".to_string(),
                },
                glsl_function: "vec2".to_string(),
            },
            NodePreset::Vec3 => Node {
                name: "Vec3".to_string(),
                uuid: generate_uuid(),
                input_param: Input {
                    fields: vec![
                        ("x".to_string(), InputField::new(GlslType::Float)),
                        ("y".to_string(), InputField::new(GlslType::Float)),
                        ("z".to_string(), InputField::new(GlslType::Float)),
                    ],
                },
                output_param: Output::GlslType {
                    glsl_type: GlslType::Vec3,
                    field_name: "vec3".to_string(),
                },
                glsl_function: "vec3".to_string(),
            },
            NodePreset::Vec4 => Node {
                name: "Vec4".to_string(),
                uuid: generate_uuid(),
                input_param: Input {
                    fields: vec![
                        ("x".to_string(), InputField::new(GlslType::Float)),
                        ("y".to_string(), InputField::new(GlslType::Float)),
                        ("z".to_string(), InputField::new(GlslType::Float)),
                        ("w".to_string(), InputField::new(GlslType::Float)),
                    ],
                },
                output_param: Output::GlslType {
                    glsl_type: GlslType::Vec4,
                    field_name: "vec4".to_string(),
                },
                glsl_function: "vec4".to_string(),
            },
            NodePreset::FloatAdd => Node {
                name: "Add Float".to_string(),
                uuid: generate_uuid(),
                input_param: Input {
                    fields: vec![
                        ("a".to_string(), InputField::new(GlslType::Float)),
                        ("b".to_string(), InputField::new(GlslType::Float)),
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
    use crate::property::InputProperty;
    use crate::shader::Shader;

    #[test]
    fn default_vec2_node() {
        let node = NodePreset::Vec2.get_node();
        let res = node.to_glsl();
        assert_eq!(
            res,
            format!("vec2 {} = vec2(0.0, 0.0);", node.output_var_name())
        );
    }

    #[test]
    fn default_vec3_node() {
        let node = NodePreset::Vec3.get_node();
        let res = node.to_glsl();
        assert_eq!(
            res,
            format!("vec3 {} = vec3(0.0, 0.0, 0.0);", node.output_var_name())
        );
    }

    #[test]
    fn default_vec4_node() {
        let node = NodePreset::Vec4.get_node();
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
