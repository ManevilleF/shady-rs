use crate::node::*;

use crate::{NativeOperation::*, NodeOperation::*, NonScalarNativeType};

macro_rules! make_enum {
    (
        $name:ident $array:ident {
            $( $variant:ident, )*
        }
    ) => {
        #[derive(Debug, Copy, Clone)]
        pub enum $name {
            $( $variant, )*
        }

        impl $name {
            pub const  $array: &'static [$name] = &[
                $( $name::$variant, )*
            ];
        }
    }
}

make_enum!(NodePreset VARIANTS {
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
});

impl NodePreset {
    pub fn get_node(&self) -> Node {
        let name = self.name();
        match self {
            Self::Vec2 => Node::new(name, TypeConstruction(NonScalarNativeType::Vec2)),
            Self::IVec2 => Node::new(name, TypeConstruction(NonScalarNativeType::IVec2)),
            Self::Vec3 => Node::new(name, TypeConstruction(NonScalarNativeType::Vec3)),
            Self::IVec3 => Node::new(name, TypeConstruction(NonScalarNativeType::IVec3)),
            Self::Vec4 => Node::new(name, TypeConstruction(NonScalarNativeType::Vec4)),
            Self::IVec4 => Node::new(name, TypeConstruction(NonScalarNativeType::IVec4)),
            Self::FloatAdd => Node::new(name, NativeOperation(Add(GlslType::Float))),
            Self::FloatMul => Node::new(name, NativeOperation(Mul(GlslType::Float))),
            Self::FloatDiv => Node::new(name, NativeOperation(Div(GlslType::Float))),
            Self::IntAdd => Node::new(name, NativeOperation(Add(GlslType::Int))),
            Self::IntMul => Node::new(name, NativeOperation(Mul(GlslType::Int))),
            Self::IntDiv => Node::new(name, NativeOperation(Div(GlslType::Int))),
            Self::FloatSelection => Node::new(name, NativeOperation(Selection(GlslType::Float))),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Vec2 => "Vec2",
            Self::IVec2 => "IVec2",
            Self::Vec3 => "Vec3",
            Self::IVec3 => "IVec3",
            Self::Vec4 => "Vec4",
            Self::IVec4 => "IVec4",
            Self::FloatAdd => "Add floats",
            Self::FloatMul => "Multiply floats",
            Self::FloatDiv => "Divide floats",
            Self::IntAdd => "Add integers",
            Self::IntMul => "Multiply integers",
            Self::IntDiv => "Divide integers",
            Self::FloatSelection => "Select float",
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
