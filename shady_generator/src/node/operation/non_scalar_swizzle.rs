use crate::{Input, InputField, NativeType, Output};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Vec2Field {
    X,
    Y,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Vec3Field {
    X,
    Y,
    Z,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Vec4Field {
    X,
    Y,
    Z,
    W,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NonScalarSwizzle {
    // Vec2
    Vec2ToVec2([Vec2Field; 2]),
    Vec2ToVec3([Vec2Field; 3]),
    Vec2ToVec4([Vec2Field; 4]),
    // Vec3
    Vec3ToVec2([Vec3Field; 2]),
    Vec3ToVec3([Vec3Field; 3]),
    Vec3ToVec4([Vec3Field; 4]),
    // Vec4
    Vec4ToVec2([Vec4Field; 2]),
    Vec4ToVec3([Vec4Field; 3]),
    Vec4ToVec4([Vec4Field; 4]),
}

// TODO: rename and reorganize
pub trait FieldToGlsl: Sized + Copy + PartialEq {
    fn all_variants() -> Vec<Self>;

    fn to_glsl(&self) -> &'static str;

    fn every_vec2_possibility() -> Vec<[Self; 2]> {
        let mut res = Vec::new();
        for x in Self::all_variants() {
            for y in Self::all_variants() {
                res.push([x, y]);
            }
        }
        res
    }

    fn every_vec3_possibility() -> Vec<[Self; 3]> {
        let mut res = Vec::new();
        for x in Self::all_variants() {
            for y in Self::all_variants() {
                for z in Self::all_variants() {
                    res.push([x, y, z]);
                }
            }
        }
        res
    }

    fn every_vec4_possibility() -> Vec<[Self; 4]> {
        let mut res = Vec::new();
        for x in Self::all_variants() {
            for y in Self::all_variants() {
                for z in Self::all_variants() {
                    for w in Self::all_variants() {
                        res.push([x, y, z, w]);
                    }
                }
            }
        }
        res
    }
}

impl FieldToGlsl for Vec4Field {
    fn all_variants() -> Vec<Self> {
        vec![Self::X, Self::Y, Self::Z, Self::W]
    }

    fn to_glsl(&self) -> &'static str {
        match self {
            Self::X => "x",
            Self::Y => "y",
            Self::Z => "z",
            Self::W => "w",
        }
    }
}

impl FieldToGlsl for Vec3Field {
    fn all_variants() -> Vec<Self> {
        vec![Self::X, Self::Y, Self::Z]
    }

    fn to_glsl(&self) -> &'static str {
        match self {
            Self::X => "x",
            Self::Y => "y",
            Self::Z => "z",
        }
    }
}

impl FieldToGlsl for Vec2Field {
    fn all_variants() -> Vec<Self> {
        vec![Self::X, Self::Y]
    }

    fn to_glsl(&self) -> &'static str {
        match self {
            Self::X => "x",
            Self::Y => "y",
        }
    }
}

fn field_to_glsl<F: FieldToGlsl>(arr: &[F]) -> String {
    let vec: Vec<&str> = arr.iter().map(|f| f.to_glsl()).collect();
    vec.join("")
}

impl NonScalarSwizzle {
    pub const VARIANTS: &'static [Self] = &[
        Self::Vec2ToVec2([Vec2Field::X, Vec2Field::Y]),
        Self::Vec2ToVec3([Vec2Field::X, Vec2Field::Y, Vec2Field::X]),
        Self::Vec2ToVec4([Vec2Field::X, Vec2Field::Y, Vec2Field::X, Vec2Field::Y]),
        Self::Vec3ToVec2([Vec3Field::X, Vec3Field::Y]),
        Self::Vec3ToVec3([Vec3Field::X, Vec3Field::Y, Vec3Field::Z]),
        Self::Vec3ToVec4([Vec3Field::X, Vec3Field::Y, Vec3Field::Z, Vec3Field::Z]),
        Self::Vec4ToVec2([Vec4Field::X, Vec4Field::Y]),
        Self::Vec4ToVec3([Vec4Field::X, Vec4Field::Y, Vec4Field::Z]),
        Self::Vec4ToVec4([Vec4Field::X, Vec4Field::Y, Vec4Field::Z, Vec4Field::W]),
    ];

    pub fn glsl_method(&self) -> String {
        match self {
            NonScalarSwizzle::Vec2ToVec2(arr) => field_to_glsl(arr),
            NonScalarSwizzle::Vec2ToVec3(arr) => field_to_glsl(arr),
            NonScalarSwizzle::Vec2ToVec4(arr) => field_to_glsl(arr),
            NonScalarSwizzle::Vec3ToVec2(arr) => field_to_glsl(arr),
            NonScalarSwizzle::Vec3ToVec3(arr) => field_to_glsl(arr),
            NonScalarSwizzle::Vec3ToVec4(arr) => field_to_glsl(arr),
            NonScalarSwizzle::Vec4ToVec2(arr) => field_to_glsl(arr),
            NonScalarSwizzle::Vec4ToVec3(arr) => field_to_glsl(arr),
            NonScalarSwizzle::Vec4ToVec4(arr) => field_to_glsl(arr),
        }
    }

    pub fn output(&self) -> Output {
        match self {
            NonScalarSwizzle::Vec2ToVec2(_)
            | NonScalarSwizzle::Vec3ToVec2(_)
            | NonScalarSwizzle::Vec4ToVec2(_) => Output::NativeType(NativeType::Vec2),
            NonScalarSwizzle::Vec2ToVec3(_)
            | NonScalarSwizzle::Vec3ToVec3(_)
            | NonScalarSwizzle::Vec4ToVec3(_) => Output::NativeType(NativeType::Vec3),
            NonScalarSwizzle::Vec2ToVec4(_)
            | NonScalarSwizzle::Vec3ToVec4(_)
            | NonScalarSwizzle::Vec4ToVec4(_) => Output::NativeType(NativeType::Vec4),
        }
    }

    pub fn input(&self) -> Input {
        match self {
            NonScalarSwizzle::Vec2ToVec2(_)
            | NonScalarSwizzle::Vec2ToVec3(_)
            | NonScalarSwizzle::Vec2ToVec4(_) => Input {
                fields: vec![("i".to_string(), InputField::new(NativeType::Vec2))],
            },
            NonScalarSwizzle::Vec3ToVec2(_)
            | NonScalarSwizzle::Vec3ToVec3(_)
            | NonScalarSwizzle::Vec3ToVec4(_) => Input {
                fields: vec![("i".to_string(), InputField::new(NativeType::Vec3))],
            },
            NonScalarSwizzle::Vec4ToVec2(_)
            | NonScalarSwizzle::Vec4ToVec3(_)
            | NonScalarSwizzle::Vec4ToVec4(_) => Input {
                fields: vec![("i".to_string(), InputField::new(NativeType::Vec4))],
            },
        }
    }

    pub fn descriptive_name(&self) -> &'static str {
        match self {
            NonScalarSwizzle::Vec2ToVec2(_) => "Vec2 To Vec2",
            NonScalarSwizzle::Vec2ToVec3(_) => "Vec2 To Vec3",
            NonScalarSwizzle::Vec2ToVec4(_) => "Vec2 To Vec4",
            NonScalarSwizzle::Vec3ToVec2(_) => "Vec3 To Vec2",
            NonScalarSwizzle::Vec3ToVec3(_) => "Vec3 To Vec3",
            NonScalarSwizzle::Vec3ToVec4(_) => "Vec3 To Vec4",
            NonScalarSwizzle::Vec4ToVec2(_) => "Vec4 To Vec2",
            NonScalarSwizzle::Vec4ToVec3(_) => "Vec4 To Vec3",
            NonScalarSwizzle::Vec4ToVec4(_) => "Vec4 To Vec4",
        }
    }

    pub fn complete_name(&self) -> String {
        match self {
            NonScalarSwizzle::Vec2ToVec2(_)
            | NonScalarSwizzle::Vec2ToVec3(_)
            | NonScalarSwizzle::Vec2ToVec4(_) => {
                format!("{}.{}", NativeType::Vec2, self.glsl_method())
            }
            NonScalarSwizzle::Vec3ToVec2(_)
            | NonScalarSwizzle::Vec3ToVec3(_)
            | NonScalarSwizzle::Vec3ToVec4(_) => {
                format!("{}.{}", NativeType::Vec3, self.glsl_method())
            }
            NonScalarSwizzle::Vec4ToVec2(_)
            | NonScalarSwizzle::Vec4ToVec3(_)
            | NonScalarSwizzle::Vec4ToVec4(_) => {
                format!("{}.{}", NativeType::Vec4, self.glsl_method())
            }
        }
    }
}

impl Vec4Field {
    pub fn every_vec4_possibility() -> Vec<[Self; 4]> {
        let start_array = [Self::X, Self::Y, Self::Z, Self::W];
        let mut res = Vec::new();
        for x in 0..4 {
            for y in 0..4 {
                for z in 0..4 {
                    for w in 0..4 {
                        res.push([
                            start_array[x],
                            start_array[y],
                            start_array[z],
                            start_array[w],
                        ]);
                    }
                }
            }
        }
        res
    }
}

impl Default for NonScalarSwizzle {
    fn default() -> Self {
        Self::Vec2ToVec3([Vec2Field::X, Vec2Field::Y, Vec2Field::X])
    }
}
