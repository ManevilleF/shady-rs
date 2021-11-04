use crate::glsl::GlslType;
use bevy::math::*;
use bevy::prelude::Color;

pub trait AsGlslPrimitiveType: Sized {
    fn primitive_glsl_type() -> GlslType;

    fn value_to_glsl(&self) -> String;

    fn glsl_instantiate(&self, var_name: &str) -> String {
        format!(
            "{} {} = {};",
            Self::primitive_glsl_type().get_glsl_type(),
            var_name,
            self.value_to_glsl()
        )
    }
}

impl AsGlslPrimitiveType for bool {
    fn primitive_glsl_type() -> GlslType {
        GlslType::Bool
    }

    fn value_to_glsl(&self) -> String {
        self.to_string()
    }
}

impl AsGlslPrimitiveType for f32 {
    fn primitive_glsl_type() -> GlslType {
        GlslType::Float
    }

    fn value_to_glsl(&self) -> String {
        format!("{:.3}", self)
    }
}

impl AsGlslPrimitiveType for f64 {
    fn primitive_glsl_type() -> GlslType {
        GlslType::Double
    }

    fn value_to_glsl(&self) -> String {
        format!("{:.5}", self)
    }
}

impl AsGlslPrimitiveType for i32 {
    fn primitive_glsl_type() -> GlslType {
        GlslType::Int
    }

    fn value_to_glsl(&self) -> String {
        self.to_string()
    }
}

impl AsGlslPrimitiveType for u32 {
    fn primitive_glsl_type() -> GlslType {
        GlslType::UInt
    }

    fn value_to_glsl(&self) -> String {
        self.to_string()
    }
}

impl AsGlslPrimitiveType for Vec2 {
    fn primitive_glsl_type() -> GlslType {
        GlslType::Vec2
    }

    fn value_to_glsl(&self) -> String {
        format!(
            "{}({:.3}, {:.3})",
            Self::primitive_glsl_type(),
            self.x,
            self.y
        )
    }
}

impl AsGlslPrimitiveType for IVec2 {
    fn primitive_glsl_type() -> GlslType {
        GlslType::IVec2
    }

    fn value_to_glsl(&self) -> String {
        format!("{}({}, {})", Self::primitive_glsl_type(), self.x, self.y)
    }
}

impl AsGlslPrimitiveType for Vec3 {
    fn primitive_glsl_type() -> GlslType {
        GlslType::Vec3
    }

    fn value_to_glsl(&self) -> String {
        format!(
            "{}({:.3}, {:.3}, {:.3})",
            Self::primitive_glsl_type(),
            self.x,
            self.y,
            self.z
        )
    }
}

impl AsGlslPrimitiveType for IVec3 {
    fn primitive_glsl_type() -> GlslType {
        GlslType::IVec3
    }

    fn value_to_glsl(&self) -> String {
        format!(
            "{}({}, {}, {})",
            Self::primitive_glsl_type(),
            self.x,
            self.y,
            self.z
        )
    }
}

impl AsGlslPrimitiveType for Vec4 {
    fn primitive_glsl_type() -> GlslType {
        GlslType::Vec4
    }

    fn value_to_glsl(&self) -> String {
        format!(
            "{}({:.3}, {:.3}, {:.3}, {:.3})",
            Self::primitive_glsl_type(),
            self.x,
            self.y,
            self.z,
            self.w
        )
    }
}

impl AsGlslPrimitiveType for IVec4 {
    fn primitive_glsl_type() -> GlslType {
        GlslType::IVec4
    }

    fn value_to_glsl(&self) -> String {
        format!(
            "{}({}, {}, {}, {})",
            Self::primitive_glsl_type(),
            self.x,
            self.y,
            self.z,
            self.w
        )
    }
}

impl AsGlslPrimitiveType for Color {
    fn primitive_glsl_type() -> GlslType {
        GlslType::Vec4
    }

    fn value_to_glsl(&self) -> String {
        let (&x, &y, &z, &w) = match self {
            Color::Rgba {
                red,
                green,
                blue,
                alpha,
            } => (red, green, blue, alpha),
            Color::RgbaLinear {
                red,
                green,
                blue,
                alpha,
            } => (red, green, blue, alpha),
            Color::Hsla {
                hue,
                lightness,
                saturation,
                alpha,
            } => (hue, lightness, saturation, alpha),
        };
        Vec4::new(x, y, z, w).value_to_glsl()
    }
}

// TODO handle matrices

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::Vec2;

    mod with_primitives {
        use super::*;

        #[test]
        fn works_for_bool() {
            assert_eq!(bool::primitive_glsl_type().get_glsl_type(), "bool");
            let a = false;
            assert_eq!(a.value_to_glsl(), "false".to_string());
            assert_eq!(a.glsl_instantiate("a"), "bool a = false;".to_string());
            let b = true;
            assert_eq!(b.value_to_glsl(), "true".to_string());
            assert_eq!(b.glsl_instantiate("b"), "bool b = true;".to_string());
        }

        #[test]
        fn works_for_f32() {
            assert_eq!(f32::primitive_glsl_type().get_glsl_type(), "float");
            let a: f32 = 0.;
            assert_eq!(a.value_to_glsl(), "0.000".to_string());
            assert_eq!(a.glsl_instantiate("a"), "float a = 0.000;".to_string());
            let b: f32 = 10.1;
            assert_eq!(b.value_to_glsl(), "10.100".to_string());
            assert_eq!(b.glsl_instantiate("b"), "float b = 10.100;".to_string());
            let c: f32 = 123.321;
            assert_eq!(c.value_to_glsl(), "123.321".to_string());
            assert_eq!(c.glsl_instantiate("c"), "float c = 123.321;".to_string());
            let d: f32 = -12345.12345;
            assert_eq!(d.value_to_glsl(), "-12345.123".to_string());
            assert_eq!(d.glsl_instantiate("d"), "float d = -12345.123;".to_string());
        }

        #[test]
        fn works_for_f64() {
            assert_eq!(f32::primitive_glsl_type().get_glsl_type(), "float");
            let a: f64 = 0.;
            assert_eq!(a.value_to_glsl(), "0.00000".to_string());
            assert_eq!(a.glsl_instantiate("a"), "double a = 0.00000;".to_string());
            let b: f64 = 10.1;
            assert_eq!(b.value_to_glsl(), "10.10000".to_string());
            assert_eq!(b.glsl_instantiate("b"), "double b = 10.10000;".to_string());
            let c: f64 = 123.321;
            assert_eq!(c.value_to_glsl(), "123.32100".to_string());
            assert_eq!(c.glsl_instantiate("c"), "double c = 123.32100;".to_string());
            let d: f64 = -12345.12345;
            assert_eq!(d.value_to_glsl(), "-12345.12345".to_string());
            assert_eq!(
                d.glsl_instantiate("d"),
                "double d = -12345.12345;".to_string()
            );
        }

        #[test]
        fn works_for_i32() {
            assert_eq!(i32::primitive_glsl_type().get_glsl_type(), "int");
            let a: i32 = 0;
            assert_eq!(a.value_to_glsl(), "0".to_string());
            assert_eq!(a.glsl_instantiate("a"), "int a = 0;".to_string());
            let b: i32 = -12345678;
            assert_eq!(b.value_to_glsl(), "-12345678".to_string());
            assert_eq!(b.glsl_instantiate("b"), "int b = -12345678;".to_string());
        }

        #[test]
        fn works_for_u32() {
            assert_eq!(u32::primitive_glsl_type().get_glsl_type(), "uint");
            let a: u32 = 0;
            assert_eq!(a.value_to_glsl(), "0".to_string());
            assert_eq!(a.glsl_instantiate("a"), "uint a = 0;".to_string());
            let b: u32 = 12345678;
            assert_eq!(b.value_to_glsl(), "12345678".to_string());
            assert_eq!(b.glsl_instantiate("b"), "uint b = 12345678;".to_string());
        }

        // TODO Test for vectors
    }
}
