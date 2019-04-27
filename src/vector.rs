use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use num_traits::Float;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

macro_rules! sum {
    ($h:expr) => ($h);
    ($h:expr, $($t:expr),*) => ($h + sum!($($t),*));
}

macro_rules! generate_vector_n {
    ($VectorN: ident, $($field: ident),+) => {
        #[repr(C)]
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct $VectorN<T: Float> {
            $(pub $field: T),+
        }

        impl<T: Float> $VectorN<T> {
            #[inline]
            pub fn new($($field: T),+) -> $VectorN<T> {
                $VectorN { $($field: $field),+ }
            }

            #[inline]
            fn add_vector_and_vector(a: &$VectorN<T>, b: &$VectorN<T>) -> $VectorN<T> {
                $VectorN::new($(a.$field + b.$field),+)
            }

            #[inline]
            fn sub_vector_and_vector(a: &$VectorN<T>, b: &$VectorN<T>) -> $VectorN<T> {
                $VectorN::new($(a.$field - b.$field),+)
            }

            #[inline]
            fn mul_vector_and_scalar(a: &$VectorN<T>, b: &T) -> $VectorN<T> {
                $VectorN::new($(a.$field * *b),+)
            }

            #[inline]
            pub fn dot(a: &$VectorN<T>, b: &$VectorN<T>) -> T {
                sum!($(a.$field * b.$field),+)
            }

            #[inline]
            pub fn length(v: &$VectorN<T>) -> T {
                $VectorN::dot(v, v).sqrt()
            }

            #[inline]
            pub fn normalize(v: &$VectorN<T>) -> $VectorN<T> {
                *v * (T::one() / $VectorN::length(v))
            }
        }

        impl<T: Float> Add<$VectorN<T>> for $VectorN<T> {
            type Output = $VectorN<T>;

            #[inline]
            fn add(self, rhs: $VectorN<T>) -> Self::Output {
                $VectorN::add_vector_and_vector(&self, &rhs)
            }
        }

        impl<T: Float> Sub<$VectorN<T>> for $VectorN<T> {
            type Output = $VectorN<T>;

            #[inline]
            fn sub(self, rhs: $VectorN<T>) -> Self::Output {
                $VectorN::sub_vector_and_vector(&self, &rhs)
            }
        }

        impl<T: Float> Mul<T> for $VectorN<T> {
            type Output = $VectorN<T>;

            #[inline]
            fn mul(self, rhs: T) -> Self::Output {
                $VectorN::mul_vector_and_scalar(&self, &rhs)
            }
        }

        impl<T: Float> AddAssign<$VectorN<T>> for $VectorN<T> {
            #[inline]
            fn add_assign(&mut self, rhs: $VectorN<T>) {
                *self = $VectorN::add_vector_and_vector(self, &rhs)
            }
        }

        impl<T: Float> SubAssign<$VectorN<T>> for $VectorN<T> {
            #[inline]
            fn sub_assign(&mut self, rhs: $VectorN<T>) {
                *self = $VectorN::sub_vector_and_vector(self, &rhs)
            }
        }

        impl<T: Float> MulAssign<T> for $VectorN<T> {
            #[inline]
            fn mul_assign(&mut self, rhs: T) {
                *self = $VectorN::mul_vector_and_scalar(self, &rhs)
            }
        }

        impl<T: AbsDiffEq> AbsDiffEq for $VectorN<T> where
            T::Epsilon: Copy,
            T: Float,
        {
            type Epsilon = T::Epsilon;

            #[inline]
            fn default_epsilon() -> T::Epsilon {
                T::default_epsilon()
            }

            #[inline]
            fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
                $(T::abs_diff_eq(&self.$field, &other.$field, epsilon))&&+
            }
        }

        impl<T: RelativeEq> RelativeEq for $VectorN<T> where
            T::Epsilon: Copy,
            T: Float,
        {
            #[inline]
            fn default_max_relative() -> T::Epsilon {
                T::default_max_relative()
            }

            #[inline]
            fn relative_eq(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
                $(T::relative_eq(&self.$field, &other.$field, epsilon, max_relative))&&+
            }
        }

        impl<T: UlpsEq> UlpsEq for $VectorN<T> where
            T::Epsilon: Copy,
            T: Float,
        {
            #[inline]
            fn default_max_ulps() -> u32 {
                T::default_max_ulps()
            }

            #[inline]
            fn ulps_eq(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
                $(T::ulps_eq(&self.$field, &other.$field, epsilon, max_ulps))&&+
            }
        }
    };
}

generate_vector_n!(Vector2, x, y);
generate_vector_n!(Vector3, x, y, z);
generate_vector_n!(Vector4, x, y, z, w);

impl<T: Float> Vector3<T> {
    #[inline]
    pub fn cross(a: &Vector3<T>, b: &Vector3<T>) -> Vector3<T> {
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
    fn vector2_length() {
        let v = Vector2::new(1.0, 0.0);
        assert_eq!(Vector2::length(&v), 1.0);
    }

    #[test]
    fn vector2_normalize() {
        let v = Vector2::new(2.0, 0.0);
        assert_eq!(Vector2::normalize(&v), Vector2::new(1.0, 0.0))
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
    fn vector3_length() {
        let v = Vector3::new(1.0, 0.0, 0.0);
        assert_eq!(Vector3::length(&v), 1.0);
    }

    #[test]
    fn vector3_normalize() {
        let v = Vector3::new(2.0, 0.0, 0.0);
        assert_eq!(Vector3::normalize(&v), Vector3::new(1.0, 0.0, 0.0))
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

    #[test]
    fn vector4_length() {
        let v = Vector4::new(1.0, 0.0, 0.0, 0.0);
        assert_eq!(Vector4::length(&v), 1.0);
    }

    #[test]
    fn vector4_normalize() {
        let v = Vector4::new(2.0, 0.0, 0.0, 0.0);
        assert_eq!(Vector4::normalize(&v), Vector4::new(1.0, 0.0, 0.0, 0.0))
    }
}
