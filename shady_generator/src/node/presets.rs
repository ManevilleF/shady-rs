use crate::node::*;

use crate::{
    GlslType::*, NativeOperation::*, NodeOperation::*, NonScalarNativeType::*, ScalarNativeType::*,
};

// TODO: add remaining presets
#[derive(Debug, Copy, Clone)]
pub enum NodePreset {
    Vec2,
    IVec2,
    Vec3,
    IVec3,
    Vec4,
    IVec4,
    FloatAdd,
    FloatMul,
    FloatDiv,
    IntAdd,
    IntMul,
    IntDiv,
    FloatSelection,
}

impl NodePreset {
    pub fn get_node(&self) -> Node {
        match self {
            NodePreset::Vec2 => Node::new("Vec2", TypeConstruction(Vec2)),
            NodePreset::IVec2 => Node::new("IVec2", TypeConstruction(IVec2)),
            NodePreset::Vec3 => Node::new("Vec3", TypeConstruction(Vec3)),
            NodePreset::IVec3 => Node::new("IVec3", TypeConstruction(IVec3)),
            NodePreset::Vec4 => Node::new("Vec4", TypeConstruction(Vec4)),
            NodePreset::IVec4 => Node::new("IVec4", TypeConstruction(IVec4)),
            NodePreset::FloatAdd => Node::new("Add floats", NativeOperation(Add(Scalar(Float)))),
            NodePreset::FloatMul => {
                Node::new("Multiply floats", NativeOperation(Mul(Scalar(Float))))
            }
            NodePreset::FloatDiv => Node::new("Divide floats", NativeOperation(Div(Scalar(Float)))),
            NodePreset::IntAdd => Node::new("Add integers", NativeOperation(Add(Scalar(Int)))),
            NodePreset::IntMul => Node::new("Multiply integers", NativeOperation(Mul(Scalar(Int)))),
            NodePreset::IntDiv => Node::new("Divide integers", NativeOperation(Div(Scalar(Int)))),
            NodePreset::FloatSelection => {
                Node::new("Select float", NativeOperation(Selection(Scalar(Float))))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_vec2_node() {
        let node = NodePreset::Vec2.get_node();
        let res = node.to_glsl();
        assert_eq!(
            res,
            format!("vec2 {} = vec2(0.0, 0.0); // Vec2 Node", node.uuid)
        );
    }

    #[test]
    fn default_vec3_node() {
        let node = NodePreset::Vec3.get_node();
        let res = node.to_glsl();
        assert_eq!(
            res,
            format!("vec3 {} = vec3(0.0, 0.0, 0.0); // Vec3 Node", node.uuid)
        );
    }

    #[test]
    fn default_vec4_node() {
        let node = NodePreset::Vec4.get_node();
        let res = node.to_glsl();
        assert_eq!(
            res,
            format!(
                "vec4 {} = vec4(0.0, 0.0, 0.0, 0.0); // Vec4 Node",
                node.uuid
            )
        );
    }

    #[test]
    fn default_float_selection_node() {
        let node = NodePreset::FloatSelection.get_node();
        let res = node.to_glsl();
        assert_eq!(
            res,
            format!(
                "float {} = false ? 0.0 : 0.0; // Select float Node",
                node.uuid
            )
        );
    }
}
