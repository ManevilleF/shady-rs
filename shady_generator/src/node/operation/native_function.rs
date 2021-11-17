use crate::{FloatingNativeType, GlslType, Input, InputField, Output};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NativeFunction {
    /// The `radians` function converts degrees to radians.
    Radians(FloatingNativeType),
    /// The `degrees` function converts radians to degrees.
    Degrees(FloatingNativeType),
    /// The `sin` function returns the sine of an angle in radians.
    Sine(FloatingNativeType),
    /// The `cos` function returns the sine of an angle in radians.
    Cosine(FloatingNativeType),
    /// The `tan` function returns the sine of an angle in radians.
    Tangent(FloatingNativeType),
    /// The `asin` function returns the sine of an angle in radians.
    ArcSine(FloatingNativeType),
    /// The `acos` function returns the sine of an angle in radians.
    ArcCosine(FloatingNativeType),
    /// The `atan` function returns the sine of an angle in radians.
    ArcTangent(FloatingNativeType),
    /// variation of the `atan` function. For a point with Cartesian coordinates (x, y)
    /// the function returns the angle θ of the same point with polar coordinates (r, θ).
    ArcTangent2(FloatingNativeType),
    /// The `power` function returns x raised to the power of y.
    Power(FloatingNativeType),
    /// The `exp` function returns the constant `e` raised to the power of x.
    Exponential(FloatingNativeType),
    /// The `exp2` function returns 2 raised to the power of x.
    Exponential2(FloatingNativeType),
    /// The `log` function returns the power to which the constant `e` has to be raised to produce x.
    Logarithm(FloatingNativeType),
    /// The `log2` function returns the power to which 2 has to be raised to produce x.
    Logarithm2(FloatingNativeType),
    /// The `sqrt` function returns the square root of x.
    SquareRoot(FloatingNativeType),
    /// The `inversesqrt` function returns the inverse square root of x, i.e. the reciprocal of the square root.
    InverseSquareRoot(FloatingNativeType),
    /// The `abs` function returns the absolute value of x, i.e. x when x is positive or zero and -x for negative x.
    Absolute(FloatingNativeType),
    /// The `sign` function returns 1.0 when x is positive, 0.0 when x is zero and -1.0 when x is negative
    Sign(FloatingNativeType),
    /// The `floor` function returns the largest integer number that is smaller or equal to x.
    Floor(FloatingNativeType),
    /// The `ceiling` function returns the smallest number that is larger or equal to x.
    Ceiling(FloatingNativeType),
    /// The `fract` function returns the fractional part of x, i.e. x minus floor(x).
    FractionalPart(FloatingNativeType),
    /// The `mod` function returns x minus the product of y and floor(x/y).
    Modulo(FloatingNativeType),
    /// variation of the `mod` function where the second parameter is always a floating scalar
    FloatModulo(FloatingNativeType),
    /// The `min` function returns the smaller of the two arguments.
    Minimum(FloatingNativeType),
    /// variation of the `min` function where the second parameter is always a floating scalar
    FloatMinimum(FloatingNativeType),
    /// The `max` function returns the larger of the two arguments.
    Maximum(FloatingNativeType),
    /// variation of the `max` function where the second parameter is always a floating scalar.
    FloatMaximum(FloatingNativeType),
    /// The `clamp` function returns x if it is larger than minVal and smaller than maxVal.
    /// In case x is smaller than minVal, minVal is returned. If x is larger than maxVal, maxVal is returned.
    Clamp(FloatingNativeType),
    /// variation of the `clamp` function where the second and third parameters are always a floating scalars.
    FloatClamp(FloatingNativeType),
    /// The `mix` function returns the linear blend of x and y, i.e. the product of x and (1 - a) plus the product of y and a.
    Mix(FloatingNativeType),
    /// variation of the `mix` function where the third parameter is always a floating scalar.
    FloatMix(FloatingNativeType),
    /// The `step` function returns 0.0 if x is smaller than edge and otherwise 1.0.
    Step(FloatingNativeType),
    /// variation of the `step` function where the edge parameter is always a floating scalar.
    FloatStep(FloatingNativeType),
    /// The `smoothstep` function returns 0.0 if x is smaller than edge0 and 1.0 if x is larger than edge1.
    /// Otherwise the return value is interpolated between 0.0 and 1.0 using Hermite polynomials.
    SmoothStep(FloatingNativeType),
    /// variation of the `smoothstep` function where the edge0 and edge1 parameters are always floating scalars.
    FloatSmoothStep(FloatingNativeType),
    /// The `distance` function returns the distance between two points.
    /// The distance of two points is the length of the vector d = p0 - p1, that starts at p1 and points to p0.
    Distance(FloatingNativeType),
    /// The `length` function returns the length of a vector defined by the Euclidean norm,
    /// i.e. the square root of the sum of the squared components.
    Length(FloatingNativeType),
    /// The `dot` function returns the dot product of the two input parameters,
    /// i.e. the sum of the component-wise products.
    /// If x and y are the same the square root of the dot product is equivalent to the length of the vector.
    DotProduct(FloatingNativeType),
    /// The `cross` function returns the cross product of the two input parameters,
    /// i.e. a vector that is perpendicular to the plane containing x and y
    /// and has a magnitude that is equal to the area of the parallelogram that x and y span.
    CrossProduct,
    /// The `normalize` function returns a vector with length 1.0 that is parallel to x, i.e. x divided by its length.
    Normalize(FloatingNativeType),
    /// The `faceforward` function returns a vector that points in the same direction as a reference vector.
    /// The function has three input parameters of the type floating scalar or float vector: N, the vector to orient,
    /// I, the incident vector, and Nref, the reference vector. If the dot product of I and Nref is smaller than zero the return value is N.
    /// Otherwise -N is returned.
    FaceForward(FloatingNativeType),
    /// The `reflect` function returns a vector that points in the direction of reflection.
    /// The function has two input parameters of the type floating scalar or float vector: I,
    /// the incident vector, and N, the normal vector of the reflecting surface.
    ///
    /// Side note: To obtain the desired result the vector N has to be normalized.
    /// The reflection vector always has the same length as the incident vector.
    /// From this it follows that the reflection vector is normalized if N and I are both normalized.
    Reflect(FloatingNativeType),
    /// The `refract` function returns a vector that points in the direction of refraction.
    /// The function has two input parameters of the type floating scalar or float vector and
    /// one input parameter of the type floating scalar: I, the incident vector, N,
    /// the normal vector of the refracting surface, and eta, the ratio of indices of refraction.
    ///
    /// Side note: To obtain the desired result the vectors I and N have to be normalized.
    Refract(FloatingNativeType),
    /// The `texture2D` function returns a texel, i.e. the (color) value of the texture for the given coordinates.
    /// The function has one input parameter of the type sampler2D and one input parameter of the type vec2 : sampler,
    /// the uniform the texture is bound to, and coord, the 2-dimensional coordinates of the texel to look up.
    Texture2d,
    /// The `texture2D` function returns a texel, i.e. the (color) value of the texture for the given coordinates.
    /// The function has one input parameter of the type sampler2D and one input parameter of the type vec2 : sampler,
    /// the uniform the texture is bound to, and coord, the 2-dimensional coordinates of the texel to look up.
    ///
    /// Third input parameter of the type float: bias.
    /// After calculating the appropriate level of detail for a texture with mipmaps the bias is added
    /// before the actual texture lookup operation is executed.
    Texture2dBias,
    /// The textureCube function returns a texel, i.e. the (color) value of the texture for the given coordinates.
    /// The function has one input parameter of the type samplerCube and one input parameter of the type vec3 : sampler,
    /// the uniform the texture is bound to, and coord, the 3-dimensional coordinates of the texel to look up.
    TextureCube,
    /// The textureCube function returns a texel, i.e. the (color) value of the texture for the given coordinates.
    /// The function has one input parameter of the type samplerCube and one input parameter of the type vec3 : sampler,
    /// the uniform the texture is bound to, and coord, the 3-dimensional coordinates of the texel to look up.
    ///
    /// Third input parameter of the type float: bias.
    /// After calculating the appropriate level of detail for a texture with mipmaps the bias is added
    /// before the actual texture lookup operation is executed.
    TextureCubeBias,
}

impl NativeFunction {
    pub fn function_name(&self) -> &'static str {
        match self {
            NativeFunction::Radians(_) => "radians",
            NativeFunction::Degrees(_) => "degrees",
            NativeFunction::Sine(_) => "sin",
            NativeFunction::Cosine(_) => "cos",
            NativeFunction::Tangent(_) => "tan",
            NativeFunction::ArcSine(_) => "asin",
            NativeFunction::ArcCosine(_) => "acos",
            NativeFunction::ArcTangent(_) | NativeFunction::ArcTangent2(_) => "atan",
            NativeFunction::Power(_) => "pow",
            NativeFunction::Exponential(_) => "exp",
            NativeFunction::Exponential2(_) => "exp2",
            NativeFunction::Logarithm(_) => "log",
            NativeFunction::Logarithm2(_) => "log2",
            NativeFunction::SquareRoot(_) => "sqrt",
            NativeFunction::InverseSquareRoot(_) => "inversesqrt",
            NativeFunction::Absolute(_) => "abs",
            NativeFunction::Sign(_) => "sign",
            NativeFunction::Floor(_) => "floor",
            NativeFunction::Ceiling(_) => "ceil",
            NativeFunction::FractionalPart(_) => "fract",
            NativeFunction::Modulo(_) | NativeFunction::FloatModulo(_) => "mod",
            NativeFunction::Minimum(_) | NativeFunction::FloatMinimum(_) => "min",
            NativeFunction::Maximum(_) | NativeFunction::FloatMaximum(_) => "max",
            NativeFunction::Clamp(_) | NativeFunction::FloatClamp(_) => "clamp",
            NativeFunction::Mix(_) | NativeFunction::FloatMix(_) => "mix",
            NativeFunction::Step(_) | NativeFunction::FloatStep(_) => "step",
            NativeFunction::SmoothStep(_) | NativeFunction::FloatSmoothStep(_) => "smoothstep",
            NativeFunction::Distance(_) => "distance",
            NativeFunction::Length(_) => "length",
            NativeFunction::DotProduct(_) => "dot",
            NativeFunction::CrossProduct => "cross",
            NativeFunction::Normalize(_) => "normalize",
            NativeFunction::FaceForward(_) => "faceforward",
            NativeFunction::Reflect(_) => "reflect",
            NativeFunction::Refract(_) => "refract",
            NativeFunction::Texture2d | NativeFunction::Texture2dBias => "texture2d",
            NativeFunction::TextureCube | NativeFunction::TextureCubeBias => "textureCube",
        }
    }

    pub fn input(&self) -> Input {
        match self {
            NativeFunction::Radians(t)
            | NativeFunction::Degrees(t)
            | NativeFunction::Sine(t)
            | NativeFunction::Cosine(t)
            | NativeFunction::Tangent(t)
            | NativeFunction::ArcSine(t)
            | NativeFunction::ArcCosine(t)
            | NativeFunction::ArcTangent(t)
            | NativeFunction::Exponential(t)
            | NativeFunction::Exponential2(t)
            | NativeFunction::Logarithm(t)
            | NativeFunction::Logarithm2(t)
            | NativeFunction::SquareRoot(t)
            | NativeFunction::InverseSquareRoot(t)
            | NativeFunction::Absolute(t)
            | NativeFunction::Sign(t)
            | NativeFunction::Floor(t)
            | NativeFunction::Ceiling(t)
            | NativeFunction::Length(t)
            | NativeFunction::Normalize(t)
            | NativeFunction::FractionalPart(t) => Input {
                fields: vec![("v".to_string(), InputField::new(GlslType::from(*t)))],
            },
            NativeFunction::ArcTangent2(t)
            | NativeFunction::Power(t)
            | NativeFunction::Modulo(t)
            | NativeFunction::Minimum(t)
            | NativeFunction::Maximum(t)
            | NativeFunction::Distance(t)
            | NativeFunction::DotProduct(t)
            | NativeFunction::Reflect(t) => Input {
                fields: vec![
                    ("a".to_string(), InputField::new(GlslType::from(*t))),
                    ("b".to_string(), InputField::new(GlslType::from(*t))),
                ],
            },
            NativeFunction::FloatModulo(t)
            | NativeFunction::FloatMinimum(t)
            | NativeFunction::FloatMaximum(t) => Input {
                fields: vec![
                    ("a".to_string(), InputField::new(GlslType::from(*t))),
                    ("b".to_string(), InputField::new(GlslType::Float)),
                ],
            },
            NativeFunction::Clamp(t) => Input {
                fields: vec![
                    ("v".to_string(), InputField::new(GlslType::from(*t))),
                    ("min".to_string(), InputField::new(GlslType::from(*t))),
                    ("max".to_string(), InputField::new(GlslType::from(*t))),
                ],
            },
            NativeFunction::FloatClamp(t) => Input {
                fields: vec![
                    ("v".to_string(), InputField::new(GlslType::from(*t))),
                    ("min".to_string(), InputField::new(GlslType::Float)),
                    ("max".to_string(), InputField::new(GlslType::Float)),
                ],
            },
            NativeFunction::Mix(t) => Input {
                fields: vec![
                    ("x".to_string(), InputField::new(GlslType::from(*t))),
                    ("y".to_string(), InputField::new(GlslType::from(*t))),
                    ("a".to_string(), InputField::new(GlslType::from(*t))),
                ],
            },
            NativeFunction::FloatMix(t) => Input {
                fields: vec![
                    ("x".to_string(), InputField::new(GlslType::from(*t))),
                    ("y".to_string(), InputField::new(GlslType::from(*t))),
                    ("a".to_string(), InputField::new(GlslType::Float)),
                ],
            },
            NativeFunction::Step(t) => Input {
                fields: vec![
                    ("edge".to_string(), InputField::new(GlslType::from(*t))),
                    ("a".to_string(), InputField::new(GlslType::from(*t))),
                ],
            },
            NativeFunction::FloatStep(t) => Input {
                fields: vec![
                    ("edge".to_string(), InputField::new(GlslType::from(*t))),
                    ("a".to_string(), InputField::new(GlslType::Float)),
                ],
            },
            NativeFunction::SmoothStep(t) => Input {
                fields: vec![
                    ("edge0".to_string(), InputField::new(GlslType::from(*t))),
                    ("edge1".to_string(), InputField::new(GlslType::from(*t))),
                    ("a".to_string(), InputField::new(GlslType::from(*t))),
                ],
            },
            NativeFunction::FloatSmoothStep(t) => Input {
                fields: vec![
                    ("edge0".to_string(), InputField::new(GlslType::Float)),
                    ("edge1".to_string(), InputField::new(GlslType::Float)),
                    ("a".to_string(), InputField::new(GlslType::from(*t))),
                ],
            },
            NativeFunction::CrossProduct => Input {
                fields: vec![
                    ("a".to_string(), InputField::new(GlslType::Vec3)),
                    ("b".to_string(), InputField::new(GlslType::Vec3)),
                ],
            },
            NativeFunction::FaceForward(t) => Input {
                fields: vec![
                    ("N".to_string(), InputField::new(GlslType::from(*t))),
                    ("I".to_string(), InputField::new(GlslType::from(*t))),
                    ("Nref".to_string(), InputField::new(GlslType::from(*t))),
                ],
            },
            NativeFunction::Refract(t) => Input {
                fields: vec![
                    ("I".to_string(), InputField::new(GlslType::from(*t))),
                    ("N".to_string(), InputField::new(GlslType::from(*t))),
                    ("eta".to_string(), InputField::new(GlslType::Float)),
                ],
            },
            NativeFunction::Texture2d => Input {
                fields: vec![
                    ("sampler".to_string(), InputField::new(GlslType::Sampler2d)),
                    ("coords".to_string(), InputField::new(GlslType::Vec2)),
                ],
            },
            NativeFunction::Texture2dBias => Input {
                fields: vec![
                    ("sampler".to_string(), InputField::new(GlslType::Sampler2d)),
                    ("coords".to_string(), InputField::new(GlslType::Vec2)),
                    ("bias".to_string(), InputField::new(GlslType::Float)),
                ],
            },
            NativeFunction::TextureCube => Input {
                fields: vec![
                    (
                        "sampler".to_string(),
                        InputField::new(GlslType::SamplerCube),
                    ),
                    ("coords".to_string(), InputField::new(GlslType::Vec3)),
                ],
            },
            NativeFunction::TextureCubeBias => Input {
                fields: vec![
                    (
                        "sampler".to_string(),
                        InputField::new(GlslType::SamplerCube),
                    ),
                    ("coords".to_string(), InputField::new(GlslType::Vec3)),
                    ("bias".to_string(), InputField::new(GlslType::Float)),
                ],
            },
        }
    }

    pub fn output(&self) -> Output {
        match self {
            NativeFunction::Radians(t)
            | NativeFunction::Degrees(t)
            | NativeFunction::Sine(t)
            | NativeFunction::Cosine(t)
            | NativeFunction::Tangent(t)
            | NativeFunction::ArcSine(t)
            | NativeFunction::ArcCosine(t)
            | NativeFunction::ArcTangent(t)
            | NativeFunction::ArcTangent2(t)
            | NativeFunction::Power(t)
            | NativeFunction::Exponential(t)
            | NativeFunction::Exponential2(t)
            | NativeFunction::Logarithm(t)
            | NativeFunction::Logarithm2(t)
            | NativeFunction::SquareRoot(t)
            | NativeFunction::InverseSquareRoot(t)
            | NativeFunction::Absolute(t)
            | NativeFunction::Sign(t)
            | NativeFunction::Floor(t)
            | NativeFunction::Ceiling(t)
            | NativeFunction::FractionalPart(t)
            | NativeFunction::Modulo(t)
            | NativeFunction::FloatModulo(t)
            | NativeFunction::Minimum(t)
            | NativeFunction::FloatMinimum(t)
            | NativeFunction::Maximum(t)
            | NativeFunction::FloatMaximum(t)
            | NativeFunction::Clamp(t)
            | NativeFunction::FloatClamp(t)
            | NativeFunction::Mix(t)
            | NativeFunction::FloatMix(t)
            | NativeFunction::Step(t)
            | NativeFunction::FloatStep(t)
            | NativeFunction::SmoothStep(t)
            | NativeFunction::FloatSmoothStep(t)
            | NativeFunction::Normalize(t)
            | NativeFunction::FaceForward(t)
            | NativeFunction::Reflect(t)
            | NativeFunction::Refract(t) => Output::GlslType {
                field_name: "o".to_string(),
                glsl_type: GlslType::from(*t),
            },
            NativeFunction::Distance(_)
            | NativeFunction::Length(_)
            | NativeFunction::DotProduct(_) => Output::GlslType {
                field_name: "o".to_string(),
                glsl_type: GlslType::Float,
            },
            NativeFunction::CrossProduct => Output::GlslType {
                field_name: "o".to_string(),
                glsl_type: GlslType::Vec3,
            },
            NativeFunction::Texture2d
            | NativeFunction::Texture2dBias
            | NativeFunction::TextureCube
            | NativeFunction::TextureCubeBias => Output::GlslType {
                field_name: "o".to_string(),
                glsl_type: GlslType::Vec4,
            },
        }
    }
}
