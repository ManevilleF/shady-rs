use bevy::utils::HashMap;
use shady_generator::{ConstantValue, InputProperty, NativeType};
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RenderPhase {
    Transparent,
    Opaque,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BuiltinValue {
    VertexPosition,
    Color([f32; 4]),
    Time,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PreviewValue {
    Unset,
    ConstantValue(ConstantValue),
    BuiltinValue(BuiltinValue),
}

#[derive(Debug, Clone)]
pub struct InputPreview {
    pub expected_type: NativeType,
    pub preview_value: PreviewValue,
}

#[derive(Debug, Clone)]
pub struct PreviewMaterial {
    pub input_values: HashMap<String, InputPreview>,
    pub render_phase: RenderPhase,
}

impl PreviewMaterial {
    pub fn insert_input(&mut self, property: &InputProperty) {
        self.input_values.insert(
            property.reference.clone(),
            InputPreview {
                expected_type: property.native_type,
                preview_value: PreviewValue::Unset,
            },
        );
    }
}

impl PreviewValue {
    pub fn available_values(native_type: NativeType) -> Vec<Self> {
        match native_type {
            NativeType::Bool => vec![Self::ConstantValue(ConstantValue::Bool(false))],
            NativeType::Int => vec![Self::ConstantValue(ConstantValue::Int(1))],
            NativeType::UInt => vec![Self::ConstantValue(ConstantValue::UInt(1))],
            NativeType::Float => vec![
                Self::ConstantValue(ConstantValue::Float(1.)),
                Self::BuiltinValue(BuiltinValue::Time),
            ],
            NativeType::Double => vec![Self::ConstantValue(ConstantValue::Double(1.))],
            NativeType::Vec2 => vec![Self::ConstantValue(ConstantValue::Vec2([1., 1.]))],
            NativeType::IVec2 => vec![Self::ConstantValue(ConstantValue::IVec2([1, 1]))],
            NativeType::Vec3 => vec![
                Self::ConstantValue(ConstantValue::Vec3([1., 1., 1.])),
                Self::BuiltinValue(BuiltinValue::VertexPosition),
            ],
            NativeType::IVec3 => vec![Self::ConstantValue(ConstantValue::IVec3([1, 1, 1]))],
            NativeType::Vec4 => vec![
                Self::ConstantValue(ConstantValue::Vec4([1., 1., 1., 1.])),
                Self::BuiltinValue(BuiltinValue::Color([1., 1., 1., 1.])),
            ],
            NativeType::IVec4 => vec![Self::ConstantValue(ConstantValue::IVec4([1, 1, 1, 1]))],
            // TODO: Add handle for textures
            NativeType::Sampler2d | NativeType::SamplerCube => vec![],
        }
    }

    pub fn selection_name(&self) -> &'static str {
        match self {
            PreviewValue::Unset => "Unset",
            PreviewValue::ConstantValue(c) => match c {
                ConstantValue::Bool(_) => "Bool",
                ConstantValue::Int(_) => "Int",
                ConstantValue::UInt(_) => "UInt",
                ConstantValue::Float(_) => "Float",
                ConstantValue::Double(_) => "Double",
                ConstantValue::Vec2(_) => "Vec2",
                ConstantValue::IVec2(_) => "IVec2",
                ConstantValue::Vec3(_) => "Vec3",
                ConstantValue::IVec3(_) => "IVec3",
                ConstantValue::Vec4(_) => "Vec4",
                ConstantValue::IVec4(_) => "IVec4",
            },
            PreviewValue::BuiltinValue(b) => match b {
                BuiltinValue::VertexPosition => "Vertex position",
                BuiltinValue::Color(_) => "Color",
                BuiltinValue::Time => "Time",
            },
        }
    }
}

impl Display for RenderPhase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RenderPhase::Transparent => "Transparent",
                RenderPhase::Opaque => "Opaque",
            }
        )
    }
}
impl Default for PreviewMaterial {
    fn default() -> Self {
        Self {
            input_values: Default::default(),
            render_phase: RenderPhase::Opaque,
        }
    }
}
