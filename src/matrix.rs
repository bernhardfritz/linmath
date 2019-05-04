use crate::vector::*;
use num_traits::{cast, Float};
#[cfg(test)]
use std::f64::consts::FRAC_PI_2;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

macro_rules! generate_matrix_n {
    ($MatrixN: ident, $VectorN: ident, $($field: ident),+) => {
        #[repr(C)]
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct $MatrixN<T: Float> {
            $(pub $field: $VectorN<T>),+
        }

        impl<T: Float> $MatrixN<T> {
            #[inline]
            pub fn new($($field: $VectorN<T>),+) -> $MatrixN<T> {
                $MatrixN { $($field: $field),+ }
            }

            #[inline]
            fn add_matrix_and_matrix(a: &$MatrixN<T>, b: &$MatrixN<T>) -> $MatrixN<T> {
                $MatrixN::new($(a.$field + b.$field),+)
            }

            #[inline]
            fn sub_matrix_and_matrix(a: &$MatrixN<T>, b: &$MatrixN<T>) -> $MatrixN<T> {
                $MatrixN::new($(a.$field - b.$field),+)
            }

            fn mul_matrix_and_vector(a: &$MatrixN<T>, b: &$VectorN<T>) -> $VectorN<T> {
                let t = $MatrixN::transpose(&a);
                $VectorN::new($($VectorN::dot(&t.$field, &b)),+)
            }

            #[inline]
            fn mul_matrix_and_scalar(a: &$MatrixN<T>, b: &T) -> $MatrixN<T> {
                $MatrixN::new($(a.$field * *b),+)
            }
        }

        impl<T: Float> Add<$MatrixN<T>> for $MatrixN<T> {
            type Output = $MatrixN<T>;

            #[inline]
            fn add(self, rhs: $MatrixN<T>) -> Self::Output {
                $MatrixN::add_matrix_and_matrix(&self, &rhs)
            }
        }

        impl<T: Float> Sub<$MatrixN<T>> for $MatrixN<T> {
            type Output = $MatrixN<T>;

            #[inline]
            fn sub(self, rhs: $MatrixN<T>) -> Self::Output {
                $MatrixN::sub_matrix_and_matrix(&self, &rhs)
            }
        }

        impl<T: Float> Mul<$MatrixN<T>> for $MatrixN<T> {
            type Output = $MatrixN<T>;

            fn mul(self, rhs: $MatrixN<T>) -> Self::Output {
                $MatrixN::mul_matrix_and_matrix(&self, &rhs)
            }
        }

        impl<T: Float> Mul<$VectorN<T>> for $MatrixN<T> {
            type Output = $VectorN<T>;

            fn mul(self, rhs: $VectorN<T>) -> Self::Output {
                $MatrixN::mul_matrix_and_vector(&self, &rhs)
            }
        }

        impl<T: Float> Mul<T> for $MatrixN<T> {
            type Output = $MatrixN<T>;

            #[inline]
            fn mul(self, rhs: T) -> Self::Output {
                $MatrixN::mul_matrix_and_scalar(&self, &rhs)
            }
        }

        impl<T: Float> AddAssign<$MatrixN<T>> for $MatrixN<T> {
            #[inline]
            fn add_assign(&mut self, rhs: $MatrixN<T>) {
                *self = $MatrixN::add_matrix_and_matrix(self, &rhs)
            }
        }

        impl<T: Float> SubAssign<$MatrixN<T>> for $MatrixN<T> {
            #[inline]
            fn sub_assign(&mut self, rhs: $MatrixN<T>) {
                *self = $MatrixN::sub_matrix_and_matrix(self, &rhs)
            }
        }

        impl<T: Float> MulAssign<$MatrixN<T>> for $MatrixN<T> {
            fn mul_assign(&mut self, rhs: $MatrixN<T>) {
                *self = $MatrixN::mul_matrix_and_matrix(self, &rhs)
            }
        }

        impl<T: Float> MulAssign<T> for $MatrixN<T> {
            #[inline]
            fn mul_assign(&mut self, rhs: T) {
                *self = $MatrixN::mul_matrix_and_scalar(self, &rhs)
            }
        }
    };
}

generate_matrix_n!(Matrix2, Vector2, x, y);
generate_matrix_n!(Matrix3, Vector3, x, y, z);
generate_matrix_n!(Matrix4, Vector4, x, y, z, w);

impl<T: Float> Matrix2<T> {
    fn mul_matrix_and_matrix(a: &Matrix2<T>, b: &Matrix2<T>) -> Matrix2<T> {
        let t = Matrix2::transpose(a);
        Matrix2::new(
            Vector2::new(Vector2::dot(&t.x, &b.x), Vector2::dot(&t.y, &b.x)),
            Vector2::new(Vector2::dot(&t.x, &b.y), Vector2::dot(&t.y, &b.y)),
        )
    }

    #[inline]
    pub fn identity() -> Matrix2<T> {
        Matrix2::new(
            Vector2::new(T::one(), T::zero()),
            Vector2::new(T::zero(), T::one()),
        )
    }

    #[inline]
    pub fn transpose(m: &Matrix2<T>) -> Matrix2<T> {
        Matrix2::new(Vector2::new(m.x.x, m.y.x), Vector2::new(m.x.y, m.y.y))
    }

    #[inline]
    pub fn determinant(m: &Matrix2<T>) -> T {
        m.x.x * m.y.y - m.x.y * m.y.x
    }

    #[inline]
    pub fn adjugate(m: &Matrix2<T>) -> Matrix2<T> {
        Matrix2::new(Vector2::new(m.y.y, -m.x.y), Vector2::new(-m.y.x, m.x.x))
    }

    pub fn inverse(m: &Matrix2<T>) -> Option<Matrix2<T>> {
        let det = Matrix2::determinant(&m);
        if det == T::zero() {
            None
        } else {
            let invdet = T::one() / det;
            let adj = Matrix2::adjugate(m);
            Some(adj * invdet)
        }
    }
}

impl<T: Float> Matrix3<T> {
    fn mul_matrix_and_matrix(a: &Matrix3<T>, b: &Matrix3<T>) -> Matrix3<T> {
        let t = Matrix3::transpose(a);
        Matrix3::new(
            Vector3::new(
                Vector3::dot(&t.x, &b.x),
                Vector3::dot(&t.y, &b.x),
                Vector3::dot(&t.z, &b.x),
            ),
            Vector3::new(
                Vector3::dot(&t.x, &b.y),
                Vector3::dot(&t.y, &b.y),
                Vector3::dot(&t.z, &b.y),
            ),
            Vector3::new(
                Vector3::dot(&t.x, &b.z),
                Vector3::dot(&t.y, &b.z),
                Vector3::dot(&t.z, &b.z),
            ),
        )
    }

    #[inline]
    pub fn identity() -> Matrix3<T> {
        Matrix3::new(
            Vector3::new(T::one(), T::zero(), T::zero()),
            Vector3::new(T::zero(), T::one(), T::zero()),
            Vector3::new(T::zero(), T::zero(), T::one()),
        )
    }

    #[inline]
    pub fn transpose(m: &Matrix3<T>) -> Matrix3<T> {
        Matrix3::new(
            Vector3::new(m.x.x, m.y.x, m.z.x),
            Vector3::new(m.x.y, m.y.y, m.z.y),
            Vector3::new(m.x.z, m.y.z, m.z.z),
        )
    }

    pub fn determinant(m: &Matrix3<T>) -> T {
        let t = Matrix3::transpose(&m);
        let c = Vector3::cross(&t.x, &t.y);
        Vector3::dot(&c, &t.z)
    }

    fn adjugate(m: &Matrix3<T>) -> Matrix3<T> {
        let t = Matrix3::transpose(&m);
        Matrix3::new(
            Vector3::cross(&t.y, &t.z),
            Vector3::cross(&t.z, &t.x),
            Vector3::cross(&t.x, &t.y),
        )
    }

    pub fn inverse(m: &Matrix3<T>) -> Option<Matrix3<T>> {
        let det = Matrix3::determinant(&m);
        if det == T::zero() {
            None
        } else {
            let invdet = T::one() / det;
            let adj = Matrix3::adjugate(&m);
            Some(adj * invdet)
        }
    }

    #[inline]
    pub fn translate(v: &Vector2<T>) -> Matrix3<T> {
        Matrix3::new(
            Vector3::new(T::one(), T::zero(), T::zero()),
            Vector3::new(T::zero(), T::one(), T::zero()),
            Vector3::new(v.x, v.y, T::one()),
        )
    }

    pub fn rotate(rad: T) -> Matrix3<T> {
        let (s, c) = rad.sin_cos();
        Matrix3::new(
            Vector3::new(c, s, T::zero()),
            Vector3::new(-s, c, T::zero()),
            Vector3::new(T::zero(), T::zero(), T::one()),
        )
    }

    #[inline]
    pub fn scale(v: &Vector2<T>) -> Matrix3<T> {
        Matrix3::new(
            Vector3::new(v.x, T::zero(), T::zero()),
            Vector3::new(T::zero(), v.y, T::zero()),
            Vector3::new(T::zero(), T::zero(), T::one()),
        )
    }

    pub fn ortho(left: T, right: T, bottom: T, top: T) -> Matrix3<T> {
        let two: T = cast(2).unwrap();
        Matrix3::new(
            Vector3::new(two / (right - left), T::zero(), T::zero()),
            Vector3::new(T::zero(), two / (top - bottom), T::zero()),
            Vector3::new(T::zero(), T::zero(), -T::one()),
        )
    }
}

impl<T: Float> Matrix4<T> {
    fn mul_matrix_and_matrix(a: &Matrix4<T>, b: &Matrix4<T>) -> Matrix4<T> {
        let t = Matrix4::transpose(a);
        Matrix4::new(
            Vector4::new(
                Vector4::dot(&t.x, &b.x),
                Vector4::dot(&t.y, &b.x),
                Vector4::dot(&t.z, &b.x),
                Vector4::dot(&t.w, &b.x),
            ),
            Vector4::new(
                Vector4::dot(&t.x, &b.y),
                Vector4::dot(&t.y, &b.y),
                Vector4::dot(&t.z, &b.y),
                Vector4::dot(&t.w, &b.y),
            ),
            Vector4::new(
                Vector4::dot(&t.x, &b.z),
                Vector4::dot(&t.y, &b.z),
                Vector4::dot(&t.z, &b.z),
                Vector4::dot(&t.w, &b.z),
            ),
            Vector4::new(
                Vector4::dot(&t.x, &b.w),
                Vector4::dot(&t.y, &b.w),
                Vector4::dot(&t.z, &b.w),
                Vector4::dot(&t.w, &b.w),
            ),
        )
    }

    #[inline]
    pub fn identity() -> Matrix4<T> {
        Matrix4::new(
            Vector4::new(T::one(), T::zero(), T::zero(), T::zero()),
            Vector4::new(T::zero(), T::one(), T::zero(), T::zero()),
            Vector4::new(T::zero(), T::zero(), T::one(), T::zero()),
            Vector4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }

    #[inline]
    pub fn transpose(m: &Matrix4<T>) -> Matrix4<T> {
        Matrix4::new(
            Vector4::new(m.x.x, m.y.x, m.z.x, m.w.x),
            Vector4::new(m.x.y, m.y.y, m.z.y, m.w.y),
            Vector4::new(m.x.z, m.y.z, m.z.z, m.w.z),
            Vector4::new(m.x.w, m.y.w, m.z.w, m.w.w),
        )
    }

    pub fn determinant(m: &Matrix4<T>) -> T {
        let a = Matrix3::new(
            Vector3::new(m.y.y, m.y.z, m.y.w),
            Vector3::new(m.z.y, m.z.z, m.z.w),
            Vector3::new(m.w.y, m.w.z, m.w.w),
        );
        let b = Matrix3::new(
            Vector3::new(m.x.y, m.x.z, m.x.w),
            Vector3::new(m.z.y, m.z.z, m.z.w),
            Vector3::new(m.w.y, m.w.z, m.w.w),
        );
        let c = Matrix3::new(
            Vector3::new(m.x.y, m.x.z, m.x.w),
            Vector3::new(m.y.y, m.y.z, m.y.w),
            Vector3::new(m.w.y, m.w.z, m.w.w),
        );
        let d = Matrix3::new(
            Vector3::new(m.x.y, m.x.z, m.x.w),
            Vector3::new(m.y.y, m.y.z, m.y.w),
            Vector3::new(m.z.y, m.z.z, m.z.w),
        );
        m.x.x * Matrix3::determinant(&a) - m.y.x * Matrix3::determinant(&b)
            + m.z.x * Matrix3::determinant(&c)
            - m.w.x * Matrix3::determinant(&d)
    }

    fn comatrix(m: &Matrix4<T>) -> Matrix4<T> {
        let c00 = Matrix3::new(
            Vector3::new(m.y.y, m.y.z, m.y.w),
            Vector3::new(m.z.y, m.z.z, m.z.w),
            Vector3::new(m.w.y, m.w.z, m.w.w),
        );
        let c01 = Matrix3::new(
            Vector3::new(m.x.y, m.x.z, m.x.w),
            Vector3::new(m.z.y, m.z.z, m.z.w),
            Vector3::new(m.w.y, m.w.z, m.w.w),
        );
        let c02 = Matrix3::new(
            Vector3::new(m.x.y, m.x.z, m.x.w),
            Vector3::new(m.y.y, m.y.z, m.y.w),
            Vector3::new(m.w.y, m.w.z, m.w.w),
        );
        let c03 = Matrix3::new(
            Vector3::new(m.x.y, m.x.z, m.x.w),
            Vector3::new(m.y.y, m.y.z, m.y.w),
            Vector3::new(m.z.y, m.z.z, m.z.w),
        );
        let c10 = Matrix3::new(
            Vector3::new(m.y.x, m.y.z, m.y.w),
            Vector3::new(m.z.x, m.z.z, m.z.w),
            Vector3::new(m.w.x, m.w.z, m.w.w),
        );
        let c11 = Matrix3::new(
            Vector3::new(m.x.x, m.x.z, m.x.w),
            Vector3::new(m.z.x, m.z.z, m.z.w),
            Vector3::new(m.w.x, m.w.z, m.w.w),
        );
        let c12 = Matrix3::new(
            Vector3::new(m.x.x, m.x.z, m.x.w),
            Vector3::new(m.y.x, m.y.z, m.y.w),
            Vector3::new(m.w.x, m.w.z, m.w.w),
        );
        let c13 = Matrix3::new(
            Vector3::new(m.x.x, m.x.z, m.x.w),
            Vector3::new(m.y.x, m.y.z, m.y.w),
            Vector3::new(m.z.x, m.z.z, m.z.w),
        );
        let c20 = Matrix3::new(
            Vector3::new(m.y.x, m.y.y, m.y.w),
            Vector3::new(m.z.x, m.z.y, m.z.w),
            Vector3::new(m.w.x, m.w.y, m.w.w),
        );
        let c21 = Matrix3::new(
            Vector3::new(m.x.x, m.x.y, m.x.w),
            Vector3::new(m.z.x, m.z.y, m.z.w),
            Vector3::new(m.w.x, m.w.y, m.w.w),
        );
        let c22 = Matrix3::new(
            Vector3::new(m.x.x, m.x.y, m.x.w),
            Vector3::new(m.y.x, m.y.y, m.y.w),
            Vector3::new(m.w.x, m.w.y, m.w.w),
        );
        let c23 = Matrix3::new(
            Vector3::new(m.x.x, m.x.y, m.x.w),
            Vector3::new(m.y.x, m.y.y, m.y.w),
            Vector3::new(m.z.x, m.z.y, m.z.w),
        );
        let c30 = Matrix3::new(
            Vector3::new(m.y.x, m.y.y, m.y.z),
            Vector3::new(m.z.x, m.z.y, m.z.z),
            Vector3::new(m.w.x, m.w.y, m.w.z),
        );
        let c31 = Matrix3::new(
            Vector3::new(m.x.x, m.x.y, m.x.z),
            Vector3::new(m.z.x, m.z.y, m.z.z),
            Vector3::new(m.w.x, m.w.y, m.w.z),
        );
        let c32 = Matrix3::new(
            Vector3::new(m.x.x, m.x.y, m.x.z),
            Vector3::new(m.y.x, m.y.y, m.y.z),
            Vector3::new(m.w.x, m.w.y, m.w.z),
        );
        let c33 = Matrix3::new(
            Vector3::new(m.x.x, m.x.y, m.x.z),
            Vector3::new(m.y.x, m.y.y, m.y.z),
            Vector3::new(m.z.x, m.z.y, m.z.z),
        );
        Matrix4::new(
            Vector4::new(
                Matrix3::determinant(&c00),
                -Matrix3::determinant(&c10),
                Matrix3::determinant(&c20),
                -Matrix3::determinant(&c30),
            ),
            Vector4::new(
                -Matrix3::determinant(&c01),
                Matrix3::determinant(&c11),
                -Matrix3::determinant(&c21),
                Matrix3::determinant(&c31),
            ),
            Vector4::new(
                Matrix3::determinant(&c02),
                -Matrix3::determinant(&c12),
                Matrix3::determinant(&c22),
                -Matrix3::determinant(&c32),
            ),
            Vector4::new(
                -Matrix3::determinant(&c03),
                Matrix3::determinant(&c13),
                -Matrix3::determinant(&c23),
                Matrix3::determinant(&c33),
            ),
        )
    }

    fn adjugate(m: &Matrix4<T>) -> Matrix4<T> {
        let c = Matrix4::comatrix(&m);
        Matrix4::transpose(&c)
    }

    pub fn inverse(m: &Matrix4<T>) -> Option<Matrix4<T>> {
        let det = Matrix4::determinant(&m);
        if det == T::zero() {
            None
        } else {
            let invdet = T::one() / det;
            let adj = Matrix4::adjugate(&m);
            Some(adj * invdet)
        }
    }

    #[inline]
    pub fn translate(v: &Vector3<T>) -> Matrix4<T> {
        Matrix4::new(
            Vector4::new(T::one(), T::zero(), T::zero(), T::zero()),
            Vector4::new(T::zero(), T::one(), T::zero(), T::zero()),
            Vector4::new(T::zero(), T::zero(), T::one(), T::zero()),
            Vector4::new(v.x, v.y, v.z, T::one()),
        )
    }

    pub fn rotate(rad: T, v: &Vector3<T>) -> Matrix4<T> {
        let (s, c) = rad.sin_cos();
        let axis = Vector3::normalize(v);
        let temp = axis * (T::one() - c);
        Matrix4::new(
            Vector4::new(
                c + temp.x * axis.x,
                temp.x * axis.y + s * axis.z,
                temp.x * axis.z - s * axis.y,
                T::zero(),
            ),
            Vector4::new(
                temp.y * axis.x - s * axis.z,
                c + temp.y * axis.y,
                temp.y * axis.z + s * axis.x,
                T::zero(),
            ),
            Vector4::new(
                temp.z * axis.x + s * axis.y,
                temp.z * axis.y - s * axis.x,
                c + temp.z * axis.z,
                T::zero(),
            ),
            Vector4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }

    #[inline]
    pub fn scale(v: &Vector3<T>) -> Matrix4<T> {
        Matrix4::new(
            Vector4::new(v.x, T::zero(), T::zero(), T::zero()),
            Vector4::new(T::zero(), v.y, T::zero(), T::zero()),
            Vector4::new(T::zero(), T::zero(), v.z, T::zero()),
            Vector4::new(T::zero(), T::zero(), T::zero(), T::one()),
        )
    }

    pub fn ortho(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Matrix4<T> {
        let two: T = cast(2).unwrap();
        Matrix4::new(
            Vector4::new(
                two / (right - left),
                T::zero(),
                T::zero(),
                T::zero(),
            ),
            Vector4::new(
                T::zero(),
                two / (top - bottom),
                T::zero(),
                T::zero(),
            ),
            Vector4::new(
                T::zero(),
                T::zero(),
                -two / (far - near),
                T::zero(),
            ),
            Vector4::new(
                -(right + left) / (right - left),
                -(top + bottom) / (top - bottom),
                -(far + near) / (far - near),
                T::one()
            ),
        )
    }

    pub fn perspective(fovy: T, aspect: T, near: T, far: T) -> Matrix4<T> {
        let two: T = cast(2).unwrap();
        let f = (fovy / two).tan().recip();
        Matrix4::new(
            Vector4::new(f / aspect, T::zero(), T::zero(), T::zero()),
            Vector4::new(T::zero(), f, T::zero(), T::zero()),
            Vector4::new(
                T::zero(),
                T::zero(),
                (far + near) / (near - far),
                (two * far * near) / (near - far),
            ),
            Vector4::new(T::zero(), T::zero(), -T::one(), T::zero()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix2_add() {
        let a = Matrix2::new(Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0));
        let b = Matrix2::new(Vector2::new(5.0, 6.0), Vector2::new(7.0, 8.0));
        assert_eq!(
            a + b,
            Matrix2::new(Vector2::new(6.0, 8.0), Vector2::new(10.0, 12.0))
        );
    }

    #[test]
    fn matrix2_add_assign() {
        let mut a = Matrix2::new(Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0));
        let b = Matrix2::new(Vector2::new(5.0, 6.0), Vector2::new(7.0, 8.0));
        a += b;
        assert_eq!(
            a,
            Matrix2::new(Vector2::new(6.0, 8.0), Vector2::new(10.0, 12.0))
        );
    }

    #[test]
    fn matrix2_sub() {
        let a = Matrix2::new(Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0));
        let b = Matrix2::new(Vector2::new(5.0, 6.0), Vector2::new(7.0, 8.0));
        assert_eq!(
            a - b,
            Matrix2::new(Vector2::new(-4.0, -4.0), Vector2::new(-4.0, -4.0))
        );
    }

    #[test]
    fn matrix2_sub_assign() {
        let mut a = Matrix2::new(Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0));
        let b = Matrix2::new(Vector2::new(5.0, 6.0), Vector2::new(7.0, 8.0));
        a -= b;
        assert_eq!(
            a,
            Matrix2::new(Vector2::new(-4.0, -4.0), Vector2::new(-4.0, -4.0))
        );
    }

    #[test]
    fn matrix2_mul_matrix() {
        let a = Matrix2::new(Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0));
        let b = Matrix2::new(Vector2::new(5.0, 6.0), Vector2::new(7.0, 8.0));
        assert_eq!(
            a * b,
            Matrix2::new(Vector2::new(23.0, 34.0), Vector2::new(31.0, 46.0))
        );
    }

    #[test]
    fn matrix2_mul_vector() {
        let a = Matrix2::new(Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0));
        let b = Vector2::new(1.0, 2.0);
        assert_eq!(a * b, Vector2::new(7.0, 10.0))
    }

    #[test]
    fn matrix2_mul_scalar() {
        let m = Matrix2::new(Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0));
        assert_eq!(
            m * 2.0,
            Matrix2::new(Vector2::new(2.0, 4.0), Vector2::new(6.0, 8.0))
        );
    }

    #[test]
    fn matrix2_transpose() {
        let m = Matrix2::new(Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0));
        let t = Matrix2::transpose(&m);
        assert_eq!(
            t,
            Matrix2::new(Vector2::new(1.0, 3.0), Vector2::new(2.0, 4.0))
        );
    }

    #[test]
    fn matrix2_determinant() {
        let a = Matrix2::new(Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0));
        let b = Matrix2::new(Vector2::new(1.0, 0.0), Vector2::new(0.0, 1.0));
        let c = Matrix2::determinant(&a);
        let d = Matrix2::determinant(&b);
        assert_eq!(c, -2.0);
        assert_eq!(d, 1.0);
    }

    #[test]
    fn matrix2_inverse() {
        let mat = Matrix2::new(Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0));
        if let Some(invmat) = Matrix2::inverse(&mat) {
            assert_eq!(mat * invmat, Matrix2::identity());
        } else {
            assert!(false)
        }
    }

    #[test]
    fn matrix3_add() {
        let a = Matrix3::new(
            Vector3::new(1.0, 2.0, 3.0),
            Vector3::new(4.0, 5.0, 6.0),
            Vector3::new(7.0, 8.0, 9.0),
        );
        let b = Matrix3::new(
            Vector3::new(10.0, 11.0, 12.0),
            Vector3::new(13.0, 14.0, 15.0),
            Vector3::new(16.0, 17.0, 18.0),
        );
        assert_eq!(
            a + b,
            Matrix3::new(
                Vector3::new(11.0, 13.0, 15.0),
                Vector3::new(17.0, 19.0, 21.0),
                Vector3::new(23.0, 25.0, 27.0)
            )
        );
    }

    #[test]
    fn matrix3_add_assign() {
        let mut a = Matrix3::new(
            Vector3::new(1.0, 2.0, 3.0),
            Vector3::new(4.0, 5.0, 6.0),
            Vector3::new(7.0, 8.0, 9.0),
        );
        let b = Matrix3::new(
            Vector3::new(10.0, 11.0, 12.0),
            Vector3::new(13.0, 14.0, 15.0),
            Vector3::new(16.0, 17.0, 18.0),
        );
        a += b;
        assert_eq!(
            a,
            Matrix3::new(
                Vector3::new(11.0, 13.0, 15.0),
                Vector3::new(17.0, 19.0, 21.0),
                Vector3::new(23.0, 25.0, 27.0)
            )
        );
    }

    #[test]
    fn matrix3_sub() {
        let a = Matrix3::new(
            Vector3::new(1.0, 2.0, 3.0),
            Vector3::new(4.0, 5.0, 6.0),
            Vector3::new(7.0, 8.0, 9.0),
        );
        let b = Matrix3::new(
            Vector3::new(10.0, 11.0, 12.0),
            Vector3::new(13.0, 14.0, 15.0),
            Vector3::new(16.0, 17.0, 18.0),
        );
        assert_eq!(
            a - b,
            Matrix3::new(
                Vector3::new(-9.0, -9.0, -9.0),
                Vector3::new(-9.0, -9.0, -9.0),
                Vector3::new(-9.0, -9.0, -9.0)
            )
        );
    }

    #[test]
    fn matrix3_sub_assign() {
        let mut a = Matrix3::new(
            Vector3::new(1.0, 2.0, 3.0),
            Vector3::new(4.0, 5.0, 6.0),
            Vector3::new(7.0, 8.0, 9.0),
        );
        let b = Matrix3::new(
            Vector3::new(10.0, 11.0, 12.0),
            Vector3::new(13.0, 14.0, 15.0),
            Vector3::new(16.0, 17.0, 18.0),
        );
        a -= b;
        assert_eq!(
            a,
            Matrix3::new(
                Vector3::new(-9.0, -9.0, -9.0),
                Vector3::new(-9.0, -9.0, -9.0),
                Vector3::new(-9.0, -9.0, -9.0)
            )
        );
    }

    #[test]
    fn matrix3_mul_matrix() {
        let a = Matrix3::new(
            Vector3::new(1.0, 2.0, 3.0),
            Vector3::new(4.0, 5.0, 6.0),
            Vector3::new(7.0, 8.0, 9.0),
        );
        let b = Matrix3::new(
            Vector3::new(10.0, 11.0, 12.0),
            Vector3::new(13.0, 14.0, 15.0),
            Vector3::new(16.0, 17.0, 18.0),
        );
        assert_eq!(
            a * b,
            Matrix3::new(
                Vector3::new(138.0, 171.0, 204.0),
                Vector3::new(174.0, 216.0, 258.0),
                Vector3::new(210.0, 261.0, 312.0)
            )
        );
    }

    #[test]
    fn matrix3_mul_vector() {
        let a = Matrix3::new(
            Vector3::new(1.0, 2.0, 3.0),
            Vector3::new(4.0, 5.0, 6.0),
            Vector3::new(7.0, 8.0, 9.0),
        );
        let b = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(a * b, Vector3::new(30.0, 36.0, 42.0));
    }

    #[test]
    fn matrix3_mul_scalar() {
        let m = Matrix3::new(
            Vector3::new(1.0, 2.0, 3.0),
            Vector3::new(4.0, 5.0, 6.0),
            Vector3::new(7.0, 8.0, 9.0),
        );
        assert_eq!(
            m * 2.0,
            Matrix3::new(
                Vector3::new(2.0, 4.0, 6.0),
                Vector3::new(8.0, 10.0, 12.0),
                Vector3::new(14.0, 16.0, 18.0)
            )
        );
    }

    #[test]
    fn matrix3_transpose() {
        let m = Matrix3::new(
            Vector3::new(1.0, 2.0, 3.0),
            Vector3::new(4.0, 5.0, 6.0),
            Vector3::new(7.0, 8.0, 9.0),
        );
        let t = Matrix3::transpose(&m);
        assert_eq!(
            t,
            Matrix3::new(
                Vector3::new(1.0, 4.0, 7.0),
                Vector3::new(2.0, 5.0, 8.0),
                Vector3::new(3.0, 6.0, 9.0)
            )
        );
    }

    #[test]
    fn matrix3_determinant() {
        let a = Matrix3::new(
            Vector3::new(1.0, 2.0, 3.0),
            Vector3::new(4.0, 5.0, 6.0),
            Vector3::new(7.0, 8.0, 9.0),
        );
        let b = Matrix3::new(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        );
        let c = Matrix3::determinant(&a);
        let d = Matrix3::determinant(&b);
        assert_eq!(c, 0.0);
        assert_eq!(d, 1.0);
    }

    #[test]
    fn matrix3_inverse() {
        let mat = Matrix3::new(
            Vector3::new(1.0, 0.0, -2.0),
            Vector3::new(-1.0, -2.0, -3.0),
            Vector3::new(1.0, 1.0, 0.0),
        );
        if let Some(invmat) = Matrix3::inverse(&mat) {
            assert_eq!(mat * invmat, Matrix3::identity());
        } else {
            assert!(false)
        }
    }

    #[test]
    fn matrix3_translate() {
        let t = Matrix3::translate(&Vector2::new(1.0, 0.0));
        let p = Vector3::new(0.0, 0.0, 1.0); // position
        let d = Vector3::new(1.0, 0.0, 0.0); // direction
        assert_eq!(t * p, Vector3::new(1.0, 0.0, 1.0));
        assert_eq!(t * d, Vector3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn matrix3_rotate() {
        let r = Matrix3::rotate(FRAC_PI_2);
        let p = Vector3::new(1.0, 0.0, 1.0); // position
        let d = Vector3::new(1.0, 0.0, 0.0); // direction
        assert_ulps_eq!(r * p, Vector3::new(0.0, 1.0, 1.0));
        assert_ulps_eq!(r * d, Vector3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn matrix3_scale() {
        let s = Matrix3::scale(&Vector2::new(2.0, 1.0));
        let p = Vector3::new(1.0, 0.0, 1.0); // position
        let d = Vector3::new(1.0, 0.0, 0.0); // direction
        assert_eq!(s * p, Vector3::new(2.0, 0.0, 1.0));
        assert_eq!(s * d, Vector3::new(2.0, 0.0, 0.0));
    }

    #[test]
    fn matrix4_add() {
        let a = Matrix4::new(
            Vector4::new(1.0, 2.0, 3.0, 4.0),
            Vector4::new(5.0, 6.0, 7.0, 8.0),
            Vector4::new(9.0, 10.0, 11.0, 12.0),
            Vector4::new(13.0, 14.0, 15.0, 16.0),
        );
        let b = Matrix4::new(
            Vector4::new(17.0, 18.0, 19.0, 20.0),
            Vector4::new(21.0, 22.0, 23.0, 24.0),
            Vector4::new(25.0, 26.0, 27.0, 28.0),
            Vector4::new(29.0, 30.0, 31.0, 32.0),
        );
        assert_eq!(
            a + b,
            Matrix4::new(
                Vector4::new(18.0, 20.0, 22.0, 24.0),
                Vector4::new(26.0, 28.0, 30.0, 32.0),
                Vector4::new(34.0, 36.0, 38.0, 40.0),
                Vector4::new(42.0, 44.0, 46.0, 48.0)
            )
        );
    }

    #[test]
    fn matrix4_sub() {
        let a = Matrix4::new(
            Vector4::new(1.0, 2.0, 3.0, 4.0),
            Vector4::new(5.0, 6.0, 7.0, 8.0),
            Vector4::new(9.0, 10.0, 11.0, 12.0),
            Vector4::new(13.0, 14.0, 15.0, 16.0),
        );
        let b = Matrix4::new(
            Vector4::new(17.0, 18.0, 19.0, 20.0),
            Vector4::new(21.0, 22.0, 23.0, 24.0),
            Vector4::new(25.0, 26.0, 27.0, 28.0),
            Vector4::new(29.0, 30.0, 31.0, 32.0),
        );
        assert_eq!(
            a - b,
            Matrix4::new(
                Vector4::new(-16.0, -16.0, -16.0, -16.0),
                Vector4::new(-16.0, -16.0, -16.0, -16.0),
                Vector4::new(-16.0, -16.0, -16.0, -16.0),
                Vector4::new(-16.0, -16.0, -16.0, -16.0)
            )
        );
    }

    #[test]
    fn matrix4_mul_matrix() {
        let a = Matrix4::new(
            Vector4::new(1.0, 2.0, 3.0, 4.0),
            Vector4::new(5.0, 6.0, 7.0, 8.0),
            Vector4::new(9.0, 10.0, 11.0, 12.0),
            Vector4::new(13.0, 14.0, 15.0, 16.0),
        );
        let b = Matrix4::new(
            Vector4::new(17.0, 18.0, 19.0, 20.0),
            Vector4::new(21.0, 22.0, 23.0, 24.0),
            Vector4::new(25.0, 26.0, 27.0, 28.0),
            Vector4::new(29.0, 30.0, 31.0, 32.0),
        );
        assert_eq!(
            a * b,
            Matrix4::new(
                Vector4::new(538.0, 612.0, 686.0, 760.0),
                Vector4::new(650.0, 740.0, 830.0, 920.0),
                Vector4::new(762.0, 868.0, 974.0, 1080.0),
                Vector4::new(874.0, 996.0, 1118.0, 1240.0)
            )
        );
    }

    #[test]
    fn matrix4_mul_vector() {
        let a = Matrix4::new(
            Vector4::new(1.0, 2.0, 3.0, 4.0),
            Vector4::new(5.0, 6.0, 7.0, 8.0),
            Vector4::new(9.0, 10.0, 11.0, 12.0),
            Vector4::new(13.0, 14.0, 15.0, 16.0),
        );
        let b = Vector4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(a * b, Vector4::new(90.0, 100.0, 110.0, 120.0));
    }

    #[test]
    fn matrix4_mul_scalar() {
        let m = Matrix4::new(
            Vector4::new(1.0, 2.0, 3.0, 4.0),
            Vector4::new(5.0, 6.0, 7.0, 8.0),
            Vector4::new(9.0, 10.0, 11.0, 12.0),
            Vector4::new(13.0, 14.0, 15.0, 16.0),
        );
        assert_eq!(
            m * 2.0,
            Matrix4::new(
                Vector4::new(2.0, 4.0, 6.0, 8.0),
                Vector4::new(10.0, 12.0, 14.0, 16.0),
                Vector4::new(18.0, 20.0, 22.0, 24.0),
                Vector4::new(26.0, 28.0, 30.0, 32.0)
            )
        );
    }

    #[test]
    fn matrix4_transpose() {
        let m = Matrix4::new(
            Vector4::new(1.0, 2.0, 3.0, 4.0),
            Vector4::new(5.0, 6.0, 7.0, 8.0),
            Vector4::new(9.0, 10.0, 11.0, 12.0),
            Vector4::new(13.0, 14.0, 15.0, 16.0),
        );
        let t = Matrix4::transpose(&m);
        assert_eq!(
            t,
            Matrix4::new(
                Vector4::new(1.0, 5.0, 9.0, 13.0),
                Vector4::new(2.0, 6.0, 10.0, 14.0),
                Vector4::new(3.0, 7.0, 11.0, 15.0),
                Vector4::new(4.0, 8.0, 12.0, 16.0)
            )
        );
    }

    #[test]
    fn matrix4_determinant() {
        let a = Matrix4::new(
            Vector4::new(1.0, 2.0, 3.0, 4.0),
            Vector4::new(5.0, 6.0, 7.0, 8.0),
            Vector4::new(9.0, 10.0, 11.0, 12.0),
            Vector4::new(13.0, 14.0, 15.0, 16.0),
        );
        let b = Matrix4::new(
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        );
        let c = Matrix4::determinant(&a);
        let d = Matrix4::determinant(&b);
        assert_eq!(c, 0.0);
        assert_eq!(d, 1.0);
    }

    #[test]
    fn matrix4_inverse() {
        let mat = Matrix4::new(
            Vector4::new(1.0, 0.0, 2.0, 2.0),
            Vector4::new(0.0, 2.0, 1.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 1.0),
            Vector4::new(1.0, 2.0, 1.0, 4.0),
        );
        if let Some(invmat) = Matrix4::inverse(&mat) {
            assert_eq!(mat * invmat, Matrix4::identity());
        } else {
            assert!(false)
        }
    }

    #[test]
    fn matrix4_translate() {
        let t = Matrix4::translate(&Vector3::new(1.0, 0.0, 0.0));
        let p = Vector4::new(0.0, 0.0, 0.0, 1.0); // position
        let d = Vector4::new(1.0, 0.0, 0.0, 0.0); // direction
        assert_eq!(t * p, Vector4::new(1.0, 0.0, 0.0, 1.0));
        assert_eq!(t * d, Vector4::new(1.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn matrix4_rotate_x() {
        let r = Matrix4::rotate(FRAC_PI_2, &Vector3::new(1.0, 0.0, 0.0));
        let p = Vector4::new(0.0, 1.0, 0.0, 1.0); // position
        let d = Vector4::new(0.0, 1.0, 0.0, 0.0); // direction
        assert_ulps_eq!(r * p, Vector4::new(0.0, 0.0, 1.0, 1.0));
        assert_ulps_eq!(r * d, Vector4::new(0.0, 0.0, 1.0, 0.0));
    }

    #[test]
    fn matrix4_rotate_y() {
        let r = Matrix4::rotate(FRAC_PI_2, &Vector3::new(0.0, 1.0, 0.0));
        let p = Vector4::new(1.0, 0.0, 0.0, 1.0); // position
        let d = Vector4::new(1.0, 0.0, 0.0, 0.0); // direction
        assert_ulps_eq!(r * p, Vector4::new(0.0, 0.0, -1.0, 1.0));
        assert_ulps_eq!(r * d, Vector4::new(0.0, 0.0, -1.0, 0.0));
    }

    #[test]
    fn matrix4_rotate_z() {
        let r = Matrix4::rotate(FRAC_PI_2, &Vector3::new(0.0, 0.0, 1.0));
        let p = Vector4::new(1.0, 0.0, 0.0, 1.0); // position
        let d = Vector4::new(1.0, 0.0, 0.0, 0.0); // direction
        assert_ulps_eq!(r * p, Vector4::new(0.0, 1.0, 0.0, 1.0));
        assert_ulps_eq!(r * d, Vector4::new(0.0, 1.0, 0.0, 0.0));
    }

    #[test]
    fn matrix4_scale() {
        let s = Matrix4::scale(&Vector3::new(2.0, 1.0, 1.0));
        let p = Vector4::new(1.0, 0.0, 0.0, 1.0); // position
        let d = Vector4::new(1.0, 0.0, 0.0, 0.0); // direction
        assert_eq!(s * p, Vector4::new(2.0, 0.0, 0.0, 1.0));
        assert_eq!(s * d, Vector4::new(2.0, 0.0, 0.0, 0.0));
    }
}
