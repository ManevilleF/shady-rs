use crate::NativeType;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConstantValue {
    /// Boolean type
    Bool(bool),
    /// Signed Integer type
    Int(i32),
    /// Unsigned Integer type
    UInt(u32),
    /// Floating number type
    Float(f32),
    /// Double floating number type (long)
    Double(f64),
    /// 2D Float vector (x, y)
    Vec2([f32; 2]),
    /// 2D Integer Vector (x, y)
    IVec2([i32; 2]),
    /// 3D Float Vector (x, y, z)
    Vec3([f32; 3]),
    /// 3D Integer Vector (x, y, z)
    IVec3([i32; 3]),
    /// 4D Float Vector (x, y, z, w)
    Vec4([f32; 4]),
    /// 4D Integer Vector (x, y, z, w)
    IVec4([i32; 4]),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constant {
    pub name: String,
    pub value: ConstantValue,
}

impl Constant {
    pub fn key(&self) -> String {
        self.name
            .to_ascii_uppercase()
            .replace(" ", "_")
            .trim()
            .to_string()
    }

    pub fn glsl_declaration(&self) -> String {
        format!(
            "const {} {} = {};",
            self.native_type(),
            self.key(),
            self.str_value()
        )
    }
}

impl ConstantValue {
    pub const VARIANTS: &'static [Self] = &[
        Self::Bool(true),
        Self::Int(1),
        Self::UInt(1),
        Self::Float(1.0),
        Self::Double(1.0),
        Self::Vec2([1.0, 1.0]),
        Self::IVec2([1, 1]),
        Self::Vec3([1.0, 1.0, 1.0]),
        Self::IVec3([1, 1, 1]),
        Self::Vec4([1.0, 1.0, 1.0, 1.0]),
        Self::IVec4([1, 1, 1, 1]),
    ];

    pub fn native_type(&self) -> NativeType {
        match self {
            ConstantValue::Bool(_) => NativeType::Bool,
            ConstantValue::Int(_) => NativeType::Int,
            ConstantValue::UInt(_) => NativeType::UInt,
            ConstantValue::Float(_) => NativeType::Float,
            ConstantValue::Double(_) => NativeType::Double,
            ConstantValue::Vec2(_) => NativeType::Vec2,
            ConstantValue::IVec2(_) => NativeType::IVec2,
            ConstantValue::Vec3(_) => NativeType::Vec3,
            ConstantValue::IVec3(_) => NativeType::IVec3,
            ConstantValue::Vec4(_) => NativeType::Vec4,
            ConstantValue::IVec4(_) => NativeType::IVec4,
        }
    }

    fn complex_declaration<T: Display, const SIZE: usize>(v: &[T; SIZE], t: NativeType) -> String {
        let vec: Vec<String> = v.iter().map(ToString::to_string).collect();
        format!("{}({})", t, vec.join(", "))
    }

    pub fn str_value(&self) -> String {
        match self {
            ConstantValue::Bool(v) => v.to_string(),
            ConstantValue::Int(v) => v.to_string(),
            ConstantValue::UInt(v) => v.to_string(),
            ConstantValue::Float(v) => v.to_string(),
            ConstantValue::Double(v) => v.to_string(),
            ConstantValue::Vec2(v) => Self::complex_declaration(v, self.native_type()),
            ConstantValue::IVec2(v) => Self::complex_declaration(v, self.native_type()),
            ConstantValue::Vec3(v) => Self::complex_declaration(v, self.native_type()),
            ConstantValue::IVec3(v) => Self::complex_declaration(v, self.native_type()),
            ConstantValue::Vec4(v) => Self::complex_declaration(v, self.native_type()),
            ConstantValue::IVec4(v) => Self::complex_declaration(v, self.native_type()),
        }
    }
}

impl Default for ConstantValue {
    fn default() -> Self {
        Self::Float(0.0)
    }
}

impl Display for ConstantValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
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
            }
        )
    }
}

impl Deref for Constant {
    type Target = ConstantValue;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
