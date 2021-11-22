use crate::node::*;

use crate::{NativeOperation::*, NodeOperation::*, NonScalarNativeType, ScalarNativeType};

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
    No,
    And,
    Or,
    Xor,
    FLoatInc,
    FloatDec,
    FloatMinus,
    FloatAdd,
    FloatMul,
    FloatDiv,
    FloatSelection,
    FloatEquals,
    FloatGreaterThan,
    FloatGreaterThanEqual,
    Vec2,
    Vec2Inc,
    Vec2Dec,
    Vec2Minus,
    Vec2Add,
    Vec2Mul,
    Vec2Div,
    Vec2Selection,
    Vec2Equals,
    IVec2,
    Vec3,
    IVec3,
    Vec4,
    IVec4,
    IntAdd,
    IntMul,
    IntDiv,
});

impl NodePreset {
    pub fn get_node(&self) -> Node {
        let name = self.name();
        match self {
            Self::No => Node::new(name, NativeOperation(No)),
            Self::And => Node::new(name, NativeOperation(And)),
            Self::Or => Node::new(name, NativeOperation(Or)),
            Self::Xor => Node::new(name, NativeOperation(Xor)),
            Self::FLoatInc => Node::new(name, NativeOperation(Inc(NativeType::Float))),
            Self::FloatDec => Node::new(name, NativeOperation(Dec(NativeType::Float))),
            Self::FloatMinus => Node::new(name, NativeOperation(Minus(NativeType::Float))),
            Self::FloatAdd => Node::new(name, NativeOperation(Add(NativeType::Float))),
            Self::FloatMul => Node::new(name, NativeOperation(Mul(NativeType::Float))),
            Self::FloatDiv => Node::new(name, NativeOperation(Div(NativeType::Float))),
            Self::FloatSelection => Node::new(name, NativeOperation(Selection(NativeType::Float))),
            Self::FloatEquals => Node::new(name, NativeOperation(Equals(NativeType::Float))),
            Self::FloatGreaterThan => {
                Node::new(name, NativeOperation(GreaterThan(ScalarNativeType::Float)))
            }
            Self::FloatGreaterThanEqual => Node::new(
                name,
                NativeOperation(GreaterThanEqual(ScalarNativeType::Float)),
            ),
            Self::Vec2 => Node::new(name, TypeConstruction(NonScalarNativeType::Vec2)),
            Self::Vec2Inc => Node::new(name, NativeOperation(Inc(NativeType::Vec2))),
            Self::Vec2Dec => Node::new(name, NativeOperation(Dec(NativeType::Vec2))),
            Self::Vec2Minus => Node::new(name, NativeOperation(Minus(NativeType::Vec2))),
            Self::Vec2Add => Node::new(name, NativeOperation(Add(NativeType::Vec2))),
            Self::Vec2Mul => Node::new(name, NativeOperation(Mul(NativeType::Vec2))),
            Self::Vec2Div => Node::new(name, NativeOperation(Div(NativeType::Vec2))),
            Self::Vec2Selection => Node::new(name, NativeOperation(Selection(NativeType::Vec2))),
            Self::Vec2Equals => Node::new(name, NativeOperation(Equals(NativeType::Vec2))),
            Self::IVec2 => Node::new(name, TypeConstruction(NonScalarNativeType::IVec2)),
            Self::Vec3 => Node::new(name, TypeConstruction(NonScalarNativeType::Vec3)),
            Self::IVec3 => Node::new(name, TypeConstruction(NonScalarNativeType::IVec3)),
            Self::Vec4 => Node::new(name, TypeConstruction(NonScalarNativeType::Vec4)),
            Self::IVec4 => Node::new(name, TypeConstruction(NonScalarNativeType::IVec4)),
            Self::IntAdd => Node::new(name, NativeOperation(Add(NativeType::Int))),
            Self::IntMul => Node::new(name, NativeOperation(Mul(NativeType::Int))),
            Self::IntDiv => Node::new(name, NativeOperation(Div(NativeType::Int))),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::No => "No",
            Self::And => "And",
            Self::Or => "Or",
            Self::Xor => "Xor",
            Self::FLoatInc => "Float ++",
            Self::FloatDec => "Float --",
            Self::FloatMinus => "-Float",
            Self::FloatAdd => "Float + Float",
            Self::FloatMul => "Float * Float",
            Self::FloatDiv => "Float / Float",
            Self::FloatSelection => "Float Selection",
            Self::FloatEquals => "Float == Float",
            Self::FloatGreaterThan => "Float > Float",
            Self::FloatGreaterThanEqual => "Float >= Float",
            Self::Vec2 => "Vec2",
            Self::Vec2Inc => "Vec ++",
            Self::Vec2Dec => "Vec --",
            Self::Vec2Minus => "-Vec2",
            Self::Vec2Add => "Vec2 + Vec2",
            Self::Vec2Mul => "Vec2 * Vec2",
            Self::Vec2Div => "Vec2 / Vec2",
            Self::Vec2Selection => "Vec2 Selection",
            Self::Vec2Equals => "Vec2 == Vec2",
            Self::IVec2 => "IVec2",
            Self::Vec3 => "Vec3",
            Self::IVec3 => "IVec3",
            Self::Vec4 => "Vec4",
            Self::IVec4 => "IVec4",
            Self::IntAdd => "Int + Int",
            Self::IntMul => "Int * Int",
            Self::IntDiv => "Int / Int",
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
                "float {} = false ? 0.0 : 0.0; // Float Selection Node",
                node.uuid
            )
        );
    }
}
