use crate::FloatingNativeType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NativeFunction {
    /// The `radians` function converts degrees to radians.
    /// The input parameter can be a floating scalar or a float vector.
    /// In case of a float vector all components are converted from degrees to radians separately.
    Radians(FloatingNativeType),
    /// The `degrees` function converts radians to degrees.
    /// The input parameter can be a floating scalar or a float vector.
    /// In case of a float vector every component is converted from radians to degrees separately.
    Degrees(FloatingNativeType),
    /// The `sin` function returns the sine of an angle in radians.
    /// The input parameter can be a floating scalar or a float vector.
    /// In case of a float vector the sine is calculated separately for every component
    Sine(FloatingNativeType),
    /// The `cos` function returns the sine of an angle in radians.
    /// The input parameter can be a floating scalar or a float vector.
    /// In case of a float vector the sine is calculated separately for every component
    Cosine(FloatingNativeType),
    /// The `tan` function returns the sine of an angle in radians.
    /// The input parameter can be a floating scalar or a float vector.
    /// In case of a float vector the sine is calculated separately for every component
    Tangent(FloatingNativeType),
    /// The `asin` function returns the sine of an angle in radians.
    /// The input parameter can be a floating scalar or a float vector.
    /// In case of a float vector the sine is calculated separately for every component
    ArcSine(FloatingNativeType),
    /// The `acos` function returns the sine of an angle in radians.
    /// The input parameter can be a floating scalar or a float vector.
    /// In case of a float vector the sine is calculated separately for every component
    ArcCosine(FloatingNativeType),
    /// The `atan` function returns the sine of an angle in radians.
    /// The input parameter can be a floating scalar or a float vector.
    /// In case of a float vector the sine is calculated separately for every component
    ArcTangent(FloatingNativeType),
    /// variation of the `atan` function. For a point with Cartesian coordinates (x, y)
    /// the function returns the angle θ of the same point with polar coordinates (r, θ).
    ArcTangent2(FloatingNativeType),
    Power(FloatingNativeType),
    Exponential(FloatingNativeType),
    Exponential2(FloatingNativeType),
    Logarithm(FloatingNativeType),
    Logarithm2(FloatingNativeType),
    SquareRoot(FloatingNativeType),
    InverseSquareRoot(FloatingNativeType),
    Absolute(FloatingNativeType),
    Sign(FloatingNativeType),
    Floor(FloatingNativeType),
    Ceiling(FloatingNativeType),
    FractionalPart(FloatingNativeType),
    Modulo(FloatingNativeType),
    FloatModulo(FloatingNativeType),
    Minimum(FloatingNativeType),
    FloatMinimum(FloatingNativeType),
    Maximum(FloatingNativeType),
    FloatMaximum(FloatingNativeType),
    Clamp(FloatingNativeType),
    FloatClamp(FloatingNativeType),
    Mix(FloatingNativeType),
    FloatMix(FloatingNativeType),
    Step(FloatingNativeType),
    FloatStep(FloatingNativeType),
    SmoothStep(FloatingNativeType),
    FloatSmoothStep(FloatingNativeType),
    Distance(FloatingNativeType),
    Len(FloatingNativeType),
    DotProduct(FloatingNativeType),
    CrossProduct(FloatingNativeType),
    Normalize(FloatingNativeType),
    Refract(FloatingNativeType),
}
