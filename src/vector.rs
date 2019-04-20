use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

macro_rules! sum {
    ($h:expr) => ($h);
    ($h:expr, $($t:expr),*) => ($h + sum!($($t),*));
}

macro_rules! generate_vector_n {
    ($VectorN: ident, $($field: ident),+) => {
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct $VectorN {
            $(pub $field: f64),+
        }

        impl $VectorN {
            #[inline]
            pub fn new($($field: f64),+) -> $VectorN {
                $VectorN { $($field: $field),+ }
            }

            #[inline]
            fn add_vector_and_vector(a: &$VectorN, b: &$VectorN) -> $VectorN {
                $VectorN::new($(a.$field + b.$field),+)
            }

            #[inline]
            fn sub_vector_and_vector(a: &$VectorN, b: &$VectorN) -> $VectorN {
                $VectorN::new($(a.$field - b.$field),+)
            }

            #[inline]
            fn mul_vector_and_scalar(a: &$VectorN, b: &f64) -> $VectorN {
                $VectorN::new($(a.$field * b),+)
            }

            #[inline]
            fn mul_scalar_and_vector(a: &f64, b: &$VectorN) -> $VectorN {
                $VectorN::new($(a * b.$field),+)
            }

            #[inline]
            pub fn dot(a: &$VectorN, b: &$VectorN) -> f64 {
                sum!($(a.$field * b.$field),+)
            }
        }

        overload_arithmetic_operator!(Add, $VectorN, $VectorN, $VectorN, add, $VectorN::add_vector_and_vector);
        overload_arithmetic_operator!(Sub, $VectorN, $VectorN, $VectorN, sub, $VectorN::sub_vector_and_vector);
        overload_arithmetic_operator!(Mul, $VectorN, f64, $VectorN, mul, $VectorN::mul_vector_and_scalar);
        overload_arithmetic_operator!(Mul, f64, $VectorN, $VectorN, mul, $VectorN::mul_scalar_and_vector);
        overload_compound_assignment_operator!(AddAssign, $VectorN, $VectorN, add_assign, $VectorN::add_vector_and_vector);
        overload_compound_assignment_operator!(SubAssign, $VectorN, $VectorN, sub_assign, $VectorN::sub_vector_and_vector);
        overload_compound_assignment_operator!(MulAssign, $VectorN, f64, mul_assign, $VectorN::mul_vector_and_scalar);

        impl From<f64> for $VectorN {
            #[inline]
            fn from(scalar: f64) -> Self {
                $VectorN { $($field: scalar),+ }
            }
        }

        impl AbsDiffEq for $VectorN {
            type Epsilon = f64;

            fn default_epsilon() -> Self::Epsilon {
                f64::default_epsilon()
            }

            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                $(f64::abs_diff_eq(&self.$field, &other.$field, epsilon))&&+
            }
        }

        impl RelativeEq for $VectorN {
            fn default_max_relative() -> Self::Epsilon {
                f64::default_max_relative()
            }

            fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
                $(f64::relative_eq(&self.$field, &other.$field, epsilon, max_relative))&&+
            }
        }

        impl UlpsEq for $VectorN {
            fn default_max_ulps() -> u32 {
                f64::default_max_ulps()
            }

            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                $(f64::ulps_eq(&self.$field, &other.$field, epsilon, max_ulps))&&+
            }
        }
    };
}

generate_vector_n!(Vector2, x, y);
generate_vector_n!(Vector3, x, y, z);
generate_vector_n!(Vector4, x, y, z, w);

impl Vector3 {
    pub fn cross(a: &Vector3, b: &Vector3) -> Vector3 {
        Vector3::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector2_add() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        assert_eq!(a + b, Vector2::new(4.0, 6.0));
    }

    #[test]
    fn vector2_add_assign() {
        let mut a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        a += b;
        assert_eq!(a, Vector2::new(4.0, 6.0));
    }

    #[test]
    fn vector2_sub() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        assert_eq!(a - b, Vector2::new(-2.0, -2.0));
    }

    #[test]
    fn vector2_sub_assign() {
        let mut a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        a -= b;
        assert_eq!(a, Vector2::new(-2.0, -2.0));
    }

    #[test]
    fn vector2_mul() {
        let v = Vector2::new(1.0, 2.0);
        assert_eq!(v * 2.0, Vector2::new(2.0, 4.0));
        assert_eq!(2.0 * v, Vector2::new(2.0, 4.0));
    }

    #[test]
    fn vector2_mul_assign() {
        let mut v = Vector2::new(1.0, 2.0);
        v *= 2.0;
        assert_eq!(v, Vector2::new(2.0, 4.0));
    }

    #[test]
    fn vector2_dot() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        assert_eq!(Vector2::dot(&a, &b), 11.0);
    }

    #[test]
    fn vector3_add() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        assert_eq!(a + b, Vector3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn vector3_add_assign() {
        let mut a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        a += b;
        assert_eq!(a, Vector3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn vector3_sub() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        assert_eq!(a - b, Vector3::new(-3.0, -3.0, -3.0));
    }

    #[test]
    fn vector3_sub_assign() {
        let mut a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        a -= b;
        assert_eq!(a, Vector3::new(-3.0, -3.0, -3.0));
    }

    #[test]
    fn vector3_mul() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v * 2.0, Vector3::new(2.0, 4.0, 6.0));
        assert_eq!(2.0 * v, Vector3::new(2.0, 4.0, 6.0));
    }


    #[test]
    fn vector3_mul_assign() {
        let mut v = Vector3::new(1.0, 2.0, 3.0);
        v *= 2.0;
        assert_eq!(v, Vector3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn vector3_dot() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        assert_eq!(Vector3::dot(&a, &b), 32.0);
    }

    #[test]
    fn vector3_cross() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        assert_eq!(Vector3::cross(&a, &b), Vector3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn vector4_add() {
        let a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vector4::new(5.0, 6.0, 7.0, 8.0);
        assert_eq!(a + b, Vector4::new(6.0, 8.0, 10.0, 12.0));
    }

    #[test]
    fn vector4_add_assign() {
        let mut a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vector4::new(5.0, 6.0, 7.0, 8.0);
        a += b;
        assert_eq!(a, Vector4::new(6.0, 8.0, 10.0, 12.0));
    }

    #[test]
    fn vector4_sub() {
        let a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vector4::new(5.0, 6.0, 7.0, 8.0);
        assert_eq!(a - b, Vector4::new(-4.0, -4.0, -4.0, -4.0));
    }

    #[test]
    fn vector4_sub_assign() {
        let mut a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vector4::new(5.0, 6.0, 7.0, 8.0);
        a -= b;
        assert_eq!(a, Vector4::new(-4.0, -4.0, -4.0, -4.0));
    }

    #[test]
    fn vector4_mul() {
        let v = Vector4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v * 2.0, Vector4::new(2.0, 4.0, 6.0, 8.0));
        assert_eq!(2.0 * v, Vector4::new(2.0, 4.0, 6.0, 8.0));
    }

    #[test]
    fn vector4_mul_assign() {
        let mut v = Vector4::new(1.0, 2.0, 3.0, 4.0);
        v *= 2.0;
        assert_eq!(v, Vector4::new(2.0, 4.0, 6.0, 8.0));
    }

    #[test]
    fn vector4_dot() {
        let a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vector4::new(5.0, 6.0, 7.0, 8.0);
        assert_eq!(Vector4::dot(&a, &b), 70.0);
    }
}
