use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};
#[cfg(test)]
use std::f64::consts::FRAC_PI_2;
use crate::vector::*;

macro_rules! generate_matrix_n {
    ($MatrixN: ident, $VectorN: ident, $($field: ident),+) => {
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct $MatrixN {
            $(pub $field: $VectorN),+
        }

        impl $MatrixN {
            #[inline]
            pub fn new($($field: $VectorN),+) -> $MatrixN {
                $MatrixN { $($field: $field),+ }
            }

            #[inline]
            fn add_matrix_and_matrix(a: &$MatrixN, b: &$MatrixN) -> $MatrixN {
                $MatrixN::new($(a.$field + b.$field),+)
            }

            #[inline]
            fn sub_matrix_and_matrix(a: &$MatrixN, b: &$MatrixN) -> $MatrixN {
                $MatrixN::new($(a.$field - b.$field),+)
            }

            fn mul_matrix_and_vector(a: &$MatrixN, b: &$VectorN) -> $VectorN {
                let t = $MatrixN::transpose(&a);
                $VectorN::new($($VectorN::dot(&t.$field, &b)),+)
            }

            #[inline]
            fn mul_matrix_and_scalar(a: &$MatrixN, b: &f64) -> $MatrixN {
                $MatrixN::new($(a.$field * *b),+)
            }

            #[inline]
            fn mul_scalar_and_matrix(a: &f64, b: &$MatrixN) -> $MatrixN {
                $MatrixN::new($(*a * b.$field),+)
            }
        }

        overload_arithmetic_operator!(Add, $MatrixN, $MatrixN, $MatrixN, add, $MatrixN::add_matrix_and_matrix);
        overload_arithmetic_operator!(Sub, $MatrixN, $MatrixN, $MatrixN, sub, $MatrixN::sub_matrix_and_matrix);
        overload_arithmetic_operator!(Mul, $MatrixN, $MatrixN, $MatrixN, mul, $MatrixN::mul_matrix_and_matrix);
        overload_arithmetic_operator!(Mul, $MatrixN, $VectorN, $VectorN, mul, $MatrixN::mul_matrix_and_vector);
        overload_arithmetic_operator!(Mul, $MatrixN, f64, $MatrixN, mul, $MatrixN::mul_matrix_and_scalar);
        overload_arithmetic_operator!(Mul, f64, $MatrixN, $MatrixN, mul, $MatrixN::mul_scalar_and_matrix);
        overload_compound_assignment_operator!(AddAssign, $MatrixN, $MatrixN, add_assign, $MatrixN::add_matrix_and_matrix);
        overload_compound_assignment_operator!(SubAssign, $MatrixN, $MatrixN, sub_assign, $MatrixN::sub_matrix_and_matrix);
        overload_compound_assignment_operator!(MulAssign, $MatrixN, $MatrixN, mul_assign, $MatrixN::mul_matrix_and_matrix);
        overload_compound_assignment_operator!(MulAssign, $MatrixN, f64, mul_assign, $MatrixN::mul_matrix_and_scalar);
    };
}

generate_matrix_n!(Matrix2, Vector2, x, y);
generate_matrix_n!(Matrix3, Vector3, x, y, z);
generate_matrix_n!(Matrix4, Vector4, x, y, z, w);

impl Matrix2 {
    fn mul_matrix_and_matrix(a: &Matrix2, b: &Matrix2) -> Matrix2 {
        let t = Matrix2::transpose(a);
        Matrix2::new(
            Vector2::new(Vector2::dot(&t.x, &b.x), Vector2::dot(&t.y, &b.x)),
            Vector2::new(Vector2::dot(&t.x, &b.y), Vector2::dot(&t.y, &b.y)),
        )
    }

    #[inline]
    pub fn transpose(m: &Matrix2) -> Matrix2 {
        Matrix2::new(
            Vector2::new(m.x.x, m.y.x),
            Vector2::new(m.x.y, m.y.y),
        )
    }

    #[inline]
    pub fn determinant(m: &Matrix2) -> f64 {
        m.x.x * m.y.y - m.x.y * m.y.x
    }

    #[inline]
    pub fn adjugate(m: &Matrix2) -> Matrix2 {
        Matrix2::new(
            Vector2::new(m.y.y, -m.x.y),
            Vector2::new(-m.y.x, m.x.x),
        )
    }

    pub fn inverse(m: &Matrix2) -> Option<Matrix2> {
        let det = Matrix2::determinant(&m);
        if det == 0.0 {
            None
        } else {
            let invdet = 1.0 / det;
            let adj = Matrix2::adjugate(m);
            Some(invdet * adj)
        }
    }
}

impl From<f64> for Matrix2 {
    #[inline]
    fn from(scalar: f64) -> Self {
        Matrix2::new(
            Vector2::new(scalar, 0.0),
            Vector2::new(0.0, scalar),
        )
    }
}

impl Matrix3 {
    fn mul_matrix_and_matrix(a: &Matrix3, b: &Matrix3) -> Matrix3 {
        let t = Matrix3::transpose(a);
        Matrix3::new(
            Vector3::new(Vector3::dot(&t.x, &b.x), Vector3::dot(&t.y, &b.x), Vector3::dot(&t.z, &b.x)),
            Vector3::new(Vector3::dot(&t.x, &b.y), Vector3::dot(&t.y, &b.y), Vector3::dot(&t.z, &b.y)),
            Vector3::new(Vector3::dot(&t.x, &b.z), Vector3::dot(&t.y, &b.z), Vector3::dot(&t.z, &b.z)),
        )
    }

    #[inline]
    pub fn transpose(m: &Matrix3) -> Matrix3 {
        Matrix3::new(
            Vector3::new(m.x.x, m.y.x, m.z.x),
            Vector3::new(m.x.y, m.y.y, m.z.y),
            Vector3::new(m.x.z, m.y.z, m.z.z),
        )
    }

    pub fn determinant(m: &Matrix3) -> f64 {
        let t = Matrix3::transpose(&m);
        let c = Vector3::cross(&t.x, &t.y);
        Vector3::dot(&c, &t.z)
    }

    fn adjugate(m: &Matrix3) -> Matrix3 {
        let t = Matrix3::transpose(&m);
        Matrix3::new(
            Vector3::cross(&t.y, &t.z),
            Vector3::cross(&t.z, &t.x),
            Vector3::cross(&t.x, &t.y),
        )
    }

    pub fn inverse(m: &Matrix3) -> Option<Matrix3> {
        let det = Matrix3::determinant(&m);
        if det == 0.0 {
            None
        } else {
            let invdet = 1.0 / det;
            let adj = Matrix3::adjugate(&m);
            Some(invdet * adj)
        }
    }

    #[inline]
    pub fn translate(v: &Vector2) -> Matrix3 {
        Matrix3::new(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(v.x, v.y, 1.0),
        )
    }

    #[inline]
    pub fn rotate(rad: f64) -> Matrix3 {
        Matrix3::new(
            Vector3::new(rad.cos(), rad.sin(), 0.0),
            Vector3::new(-rad.sin(), rad.cos(), 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        )
    }

    #[inline]
    pub fn scale(v: &Vector2) -> Matrix3 {
        Matrix3::new(
            Vector3::new(v.x, 0.0, 0.0),
            Vector3::new(0.0, v.y, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        )
    }
}

impl From<f64> for Matrix3 {
    #[inline]
    fn from(scalar: f64) -> Self {
        Matrix3::new(
            Vector3::new(scalar, 0.0, 0.0),
            Vector3::new(0.0, scalar, 0.0),
            Vector3::new(0.0, 0.0, scalar),
        )
    }
}

impl Matrix4 {
    fn mul_matrix_and_matrix(a: &Matrix4, b: &Matrix4) -> Matrix4 {
        let t = Matrix4::transpose(a);
        Matrix4::new(
            Vector4::new(Vector4::dot(&t.x, &b.x), Vector4::dot(&t.y, &b.x), Vector4::dot(&t.z, &b.x), Vector4::dot(&t.w, &b.x)),
            Vector4::new(Vector4::dot(&t.x, &b.y), Vector4::dot(&t.y, &b.y), Vector4::dot(&t.z, &b.y), Vector4::dot(&t.w, &b.y)),
            Vector4::new(Vector4::dot(&t.x, &b.z), Vector4::dot(&t.y, &b.z), Vector4::dot(&t.z, &b.z), Vector4::dot(&t.w, &b.z)),
            Vector4::new(Vector4::dot(&t.x, &b.w), Vector4::dot(&t.y, &b.w), Vector4::dot(&t.z, &b.w), Vector4::dot(&t.w, &b.w)),
        )
    }

    #[inline]
    pub fn transpose(m: &Matrix4) -> Matrix4 {
        Matrix4::new(
            Vector4::new(m.x.x, m.y.x, m.z.x, m.w.x),
            Vector4::new(m.x.y, m.y.y, m.z.y, m.w.y),
            Vector4::new(m.x.z, m.y.z, m.z.z, m.w.z),
            Vector4::new(m.x.w, m.y.w, m.z.w, m.w.w),
        )
    }

    pub fn determinant(m: &Matrix4) -> f64 {
        let a = Matrix3::new(Vector3::new(m.y.y, m.y.z, m.y.w), Vector3::new(m.z.y, m.z.z, m.z.w), Vector3::new(m.w.y, m.w.z, m.w.w));
        let b = Matrix3::new(Vector3::new(m.x.y, m.x.z, m.x.w), Vector3::new(m.z.y, m.z.z, m.z.w), Vector3::new(m.w.y, m.w.z, m.w.w));
        let c = Matrix3::new(Vector3::new(m.x.y, m.x.z, m.x.w), Vector3::new(m.y.y, m.y.z, m.y.w), Vector3::new(m.w.y, m.w.z, m.w.w));
        let d = Matrix3::new(Vector3::new(m.x.y, m.x.z, m.x.w), Vector3::new(m.y.y, m.y.z, m.y.w), Vector3::new(m.z.y, m.z.z, m.z.w));
        m.x.x * Matrix3::determinant(&a) - m.y.x * Matrix3::determinant(&b) + m.z.x * Matrix3::determinant(&c) - m.w.x * Matrix3::determinant(&d)
    }

    fn comatrix(m: &Matrix4) -> Matrix4 {
        let c00 = Matrix3::new(Vector3::new(m.y.y, m.y.z, m.y.w), Vector3::new(m.z.y, m.z.z, m.z.w), Vector3::new(m.w.y, m.w.z, m.w.w));
        let c01 = Matrix3::new(Vector3::new(m.x.y, m.x.z, m.x.w), Vector3::new(m.z.y, m.z.z, m.z.w), Vector3::new(m.w.y, m.w.z, m.w.w));
        let c02 = Matrix3::new(Vector3::new(m.x.y, m.x.z, m.x.w), Vector3::new(m.y.y, m.y.z, m.y.w), Vector3::new(m.w.y, m.w.z, m.w.w));
        let c03 = Matrix3::new(Vector3::new(m.x.y, m.x.z, m.x.w), Vector3::new(m.y.y, m.y.z, m.y.w), Vector3::new(m.z.y, m.z.z, m.z.w));
        let c10 = Matrix3::new(Vector3::new(m.y.x, m.y.z, m.y.w), Vector3::new(m.z.x, m.z.z, m.z.w), Vector3::new(m.w.x, m.w.z, m.w.w));
        let c11 = Matrix3::new(Vector3::new(m.x.x, m.x.z, m.x.w), Vector3::new(m.z.x, m.z.z, m.z.w), Vector3::new(m.w.x, m.w.z, m.w.w));
        let c12 = Matrix3::new(Vector3::new(m.x.x, m.x.z, m.x.w), Vector3::new(m.y.x, m.y.z, m.y.w), Vector3::new(m.w.x, m.w.z, m.w.w));
        let c13 = Matrix3::new(Vector3::new(m.x.x, m.x.z, m.x.w), Vector3::new(m.y.x, m.y.z, m.y.w), Vector3::new(m.z.x, m.z.z, m.z.w));
        let c20 = Matrix3::new(Vector3::new(m.y.x, m.y.y, m.y.w), Vector3::new(m.z.x, m.z.y, m.z.w), Vector3::new(m.w.x, m.w.y, m.w.w));
        let c21 = Matrix3::new(Vector3::new(m.x.x, m.x.y, m.x.w), Vector3::new(m.z.x, m.z.y, m.z.w), Vector3::new(m.w.x, m.w.y, m.w.w));
        let c22 = Matrix3::new(Vector3::new(m.x.x, m.x.y, m.x.w), Vector3::new(m.y.x, m.y.y, m.y.w), Vector3::new(m.w.x, m.w.y, m.w.w));
        let c23 = Matrix3::new(Vector3::new(m.x.x, m.x.y, m.x.w), Vector3::new(m.y.x, m.y.y, m.y.w), Vector3::new(m.z.x, m.z.y, m.z.w));
        let c30 = Matrix3::new(Vector3::new(m.y.x, m.y.y, m.y.z), Vector3::new(m.z.x, m.z.y, m.z.z), Vector3::new(m.w.x, m.w.y, m.w.z));
        let c31 = Matrix3::new(Vector3::new(m.x.x, m.x.y, m.x.z), Vector3::new(m.z.x, m.z.y, m.z.z), Vector3::new(m.w.x, m.w.y, m.w.z));
        let c32 = Matrix3::new(Vector3::new(m.x.x, m.x.y, m.x.z), Vector3::new(m.y.x, m.y.y, m.y.z), Vector3::new(m.w.x, m.w.y, m.w.z));
        let c33 = Matrix3::new(Vector3::new(m.x.x, m.x.y, m.x.z), Vector3::new(m.y.x, m.y.y, m.y.z), Vector3::new(m.z.x, m.z.y, m.z.z));
        Matrix4::new(
            Vector4::new(Matrix3::determinant(&c00), -Matrix3::determinant(&c10), Matrix3::determinant(&c20), -Matrix3::determinant(&c30)),
            Vector4::new(-Matrix3::determinant(&c01), Matrix3::determinant(&c11), -Matrix3::determinant(&c21), Matrix3::determinant(&c31)),
            Vector4::new(Matrix3::determinant(&c02), -Matrix3::determinant(&c12), Matrix3::determinant(&c22), -Matrix3::determinant(&c32)),
            Vector4::new(-Matrix3::determinant(&c03), Matrix3::determinant(&c13), -Matrix3::determinant(&c23), Matrix3::determinant(&c33))
        )
    }

    fn adjugate(m: &Matrix4) -> Matrix4 {
        let c = Matrix4::comatrix(&m);
        Matrix4::transpose(&c)
    }

    pub fn inverse(m: &Matrix4) -> Option<Matrix4> {
        let det = Matrix4::determinant(&m);
        if det == 0.0 {
            None
        } else {
            let invdet = 1.0 / det;
            let adj = Matrix4::adjugate(&m);
            Some(invdet * adj)
        }
    }

    #[inline]
    pub fn translate(v: &Vector3) -> Matrix4 {
        Matrix4::new(
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(v.x, v.y, v.z, 1.0),
        )
    }

    #[inline]
    pub fn rotate_x(rad: f64) -> Matrix4 {
        Matrix4::new(
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, rad.cos(), rad.sin(), 0.0),
            Vector4::new(0.0, -rad.sin(), rad.cos(), 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    #[inline]
    pub fn rotate_y(rad: f64) -> Matrix4 {
        Matrix4::new(
            Vector4::new(rad.cos(), 0.0, -rad.sin(), 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(rad.sin(), 0.0, rad.cos(), 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    #[inline]
    pub fn rotate_z(rad: f64) -> Matrix4 {
        Matrix4::new(
            Vector4::new(rad.cos(), rad.sin(), 0.0, 0.0),
            Vector4::new(-rad.sin(), rad.cos(), 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    #[inline]
    pub fn scale(v: &Vector3) -> Matrix4 {
        Matrix4::new(
            Vector4::new(v.x, 0.0, 0.0, 0.0),
            Vector4::new(0.0, v.y, 0.0, 0.0),
            Vector4::new(0.0, 0.0, v.z, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        )
    }
}

impl From<f64> for Matrix4 {
    #[inline]
    fn from(scalar: f64) -> Self {
        Matrix4::new(
            Vector4::new(scalar, 0.0, 0.0, 0.0),
            Vector4::new(0.0, scalar, 0.0, 0.0),
            Vector4::new(0.0, 0.0, scalar, 0.0),
            Vector4::new(0.0, 0.0, 0.0, scalar),
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
        assert_eq!(a + b, Matrix2::new(Vector2::new(6.0, 8.0), Vector2::new(10.0, 12.0)));
    }

    #[test]
    fn matrix2_add_assign() {
        let mut a = Matrix2::new(Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0));
        let b = Matrix2::new(Vector2::new(5.0, 6.0), Vector2::new(7.0, 8.0));
        a += b;
        assert_eq!(a, Matrix2::new(Vector2::new(6.0, 8.0), Vector2::new(10.0, 12.0)));
    }

    #[test]
    fn matrix2_sub() {
        let a = Matrix2::new(Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0));
        let b = Matrix2::new(Vector2::new(5.0, 6.0), Vector2::new(7.0, 8.0));
        assert_eq!(a - b, Matrix2::new(Vector2::new(-4.0, -4.0), Vector2::new(-4.0, -4.0)));
    }

    #[test]
    fn matrix2_sub_assign() {
        let mut a = Matrix2::new(Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0));
        let b = Matrix2::new(Vector2::new(5.0, 6.0), Vector2::new(7.0, 8.0));
        a -= b;
        assert_eq!(a, Matrix2::new(Vector2::new(-4.0, -4.0), Vector2::new(-4.0, -4.0)));
    }

    #[test]
    fn matrix2_mul_matrix() {
        let a = Matrix2::new(Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0));
        let b = Matrix2::new(Vector2::new(5.0, 6.0), Vector2::new(7.0, 8.0));
        assert_eq!(a * b, Matrix2::new(Vector2::new(23.0, 34.0), Vector2::new(31.0, 46.0)));
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
        assert_eq!(m * 2.0, Matrix2::new(Vector2::new(2.0, 4.0), Vector2::new(6.0, 8.0)));
        assert_eq!(2.0 * m, Matrix2::new(Vector2::new(2.0, 4.0), Vector2::new(6.0, 8.0)));
    }

    #[test]
    fn matrix2_transpose() {
        let m = Matrix2::new(Vector2::new(1.0, 2.0), Vector2::new(3.0, 4.0));
        let t = Matrix2::transpose(&m);
        assert_eq!(t, Matrix2::new(Vector2::new(1.0, 3.0), Vector2::new(2.0, 4.0)));
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
            assert_eq!(mat * invmat, 1.0.into());
        } else {
            assert!(false)
        }
    }

    #[test]
    fn matrix3_add() {
        let a = Matrix3::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(4.0, 5.0, 6.0), Vector3::new(7.0, 8.0, 9.0));
        let b = Matrix3::new(Vector3::new(10.0, 11.0, 12.0), Vector3::new(13.0, 14.0, 15.0), Vector3::new(16.0, 17.0, 18.0));
        assert_eq!(a + b, Matrix3::new(Vector3::new(11.0, 13.0, 15.0), Vector3::new(17.0, 19.0, 21.0), Vector3::new(23.0, 25.0, 27.0)));
    }

    #[test]
    fn matrix3_add_assign() {
        let mut a = Matrix3::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(4.0, 5.0, 6.0), Vector3::new(7.0, 8.0, 9.0));
        let b = Matrix3::new(Vector3::new(10.0, 11.0, 12.0), Vector3::new(13.0, 14.0, 15.0), Vector3::new(16.0, 17.0, 18.0));
        a += b;
        assert_eq!(a, Matrix3::new(Vector3::new(11.0, 13.0, 15.0), Vector3::new(17.0, 19.0, 21.0), Vector3::new(23.0, 25.0, 27.0)));
    }

    #[test]
    fn matrix3_sub() {
        let a = Matrix3::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(4.0, 5.0, 6.0), Vector3::new(7.0, 8.0, 9.0));
        let b = Matrix3::new(Vector3::new(10.0, 11.0, 12.0), Vector3::new(13.0, 14.0, 15.0), Vector3::new(16.0, 17.0, 18.0));
        assert_eq!(a - b, Matrix3::new(Vector3::new(-9.0, -9.0, -9.0), Vector3::new(-9.0, -9.0, -9.0), Vector3::new(-9.0, -9.0, -9.0)));
    }

    #[test]
    fn matrix3_sub_assign() {
        let mut a = Matrix3::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(4.0, 5.0, 6.0), Vector3::new(7.0, 8.0, 9.0));
        let b = Matrix3::new(Vector3::new(10.0, 11.0, 12.0), Vector3::new(13.0, 14.0, 15.0), Vector3::new(16.0, 17.0, 18.0));
        a -= b;
        assert_eq!(a, Matrix3::new(Vector3::new(-9.0, -9.0, -9.0), Vector3::new(-9.0, -9.0, -9.0), Vector3::new(-9.0, -9.0, -9.0)));
    }

    #[test]
    fn matrix3_mul_matrix() {
        let a = Matrix3::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(4.0, 5.0, 6.0), Vector3::new(7.0, 8.0, 9.0));
        let b = Matrix3::new(Vector3::new(10.0, 11.0, 12.0), Vector3::new(13.0, 14.0, 15.0), Vector3::new(16.0, 17.0, 18.0));
        assert_eq!(a * b, Matrix3::new(Vector3::new(138.0, 171.0, 204.0), Vector3::new(174.0, 216.0, 258.0), Vector3::new(210.0, 261.0, 312.0)));
    }

    #[test]
    fn matrix3_mul_vector() {
        let a = Matrix3::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(4.0, 5.0, 6.0), Vector3::new(7.0, 8.0, 9.0));
        let b = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(a * b, Vector3::new(30.0, 36.0, 42.0));
    }

    #[test]
    fn matrix3_mul_scalar() {
        let m = Matrix3::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(4.0, 5.0, 6.0), Vector3::new(7.0, 8.0, 9.0));
        assert_eq!(m * 2.0, Matrix3::new(Vector3::new(2.0, 4.0, 6.0), Vector3::new(8.0, 10.0, 12.0), Vector3::new(14.0, 16.0, 18.0)));
        assert_eq!(2.0 * m, Matrix3::new(Vector3::new(2.0, 4.0, 6.0), Vector3::new(8.0, 10.0, 12.0), Vector3::new(14.0, 16.0, 18.0)));
    }

    #[test]
    fn matrix3_transpose() {
        let m = Matrix3::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(4.0, 5.0, 6.0), Vector3::new(7.0, 8.0, 9.0));
        let t = Matrix3::transpose(&m);
        assert_eq!(t, Matrix3::new(Vector3::new(1.0, 4.0, 7.0), Vector3::new(2.0, 5.0, 8.0), Vector3::new(3.0, 6.0, 9.0)));
    }

    #[test]
    fn matrix3_determinant() {
        let a = Matrix3::new(Vector3::new(1.0, 2.0, 3.0), Vector3::new(4.0, 5.0, 6.0), Vector3::new(7.0, 8.0, 9.0));
        let b = Matrix3::new(Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let c = Matrix3::determinant(&a);
        let d = Matrix3::determinant(&b);
        assert_eq!(c, 0.0);
        assert_eq!(d, 1.0);
    }

    #[test]
    fn matrix3_inverse() {
        let mat = Matrix3::new(Vector3::new(1.0, 0.0, -2.0), Vector3::new(-1.0, -2.0, -3.0), Vector3::new(1.0, 1.0, 0.0));
        if let Some(invmat) = Matrix3::inverse(&mat) {
            assert_eq!(mat * invmat, 1.0.into());
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
        let a = Matrix4::new(Vector4::new(1.0, 2.0, 3.0, 4.0), Vector4::new(5.0, 6.0, 7.0, 8.0), Vector4::new(9.0, 10.0, 11.0, 12.0), Vector4::new(13.0, 14.0, 15.0, 16.0));
        let b = Matrix4::new(Vector4::new(17.0, 18.0, 19.0, 20.0), Vector4::new(21.0, 22.0, 23.0, 24.0), Vector4::new(25.0, 26.0, 27.0, 28.0), Vector4::new(29.0, 30.0, 31.0, 32.0));
        assert_eq!(a + b, Matrix4::new(Vector4::new(18.0, 20.0, 22.0, 24.0), Vector4::new(26.0, 28.0, 30.0, 32.0), Vector4::new(34.0, 36.0, 38.0, 40.0), Vector4::new(42.0, 44.0, 46.0, 48.0)));
    }

    #[test]
    fn matrix4_sub() {
        let a = Matrix4::new(Vector4::new(1.0, 2.0, 3.0, 4.0), Vector4::new(5.0, 6.0, 7.0, 8.0), Vector4::new(9.0, 10.0, 11.0, 12.0), Vector4::new(13.0, 14.0, 15.0, 16.0));
        let b = Matrix4::new(Vector4::new(17.0, 18.0, 19.0, 20.0), Vector4::new(21.0, 22.0, 23.0, 24.0), Vector4::new(25.0, 26.0, 27.0, 28.0), Vector4::new(29.0, 30.0, 31.0, 32.0));
        assert_eq!(a - b, Matrix4::new(Vector4::new(-16.0, -16.0, -16.0, -16.0), Vector4::new(-16.0, -16.0, -16.0, -16.0), Vector4::new(-16.0, -16.0, -16.0, -16.0), Vector4::new(-16.0, -16.0, -16.0, -16.0)));
    }

    #[test]
    fn matrix4_mul_matrix() {
        let a = Matrix4::new(Vector4::new(1.0, 2.0, 3.0, 4.0), Vector4::new(5.0, 6.0, 7.0, 8.0), Vector4::new(9.0, 10.0, 11.0, 12.0), Vector4::new(13.0, 14.0, 15.0, 16.0));
        let b = Matrix4::new(Vector4::new(17.0, 18.0, 19.0, 20.0), Vector4::new(21.0, 22.0, 23.0, 24.0), Vector4::new(25.0, 26.0, 27.0, 28.0), Vector4::new(29.0, 30.0, 31.0, 32.0));
        assert_eq!(a * b, Matrix4::new(Vector4::new(538.0, 612.0, 686.0, 760.0), Vector4::new(650.0, 740.0, 830.0, 920.0), Vector4::new(762.0, 868.0, 974.0, 1080.0), Vector4::new(874.0, 996.0, 1118.0, 1240.0)));
    }

    #[test]
    fn matrix4_mul_vector() {
        let a = Matrix4::new(Vector4::new(1.0, 2.0, 3.0, 4.0), Vector4::new(5.0, 6.0, 7.0, 8.0), Vector4::new(9.0, 10.0, 11.0, 12.0), Vector4::new(13.0, 14.0, 15.0, 16.0));
        let b = Vector4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(a * b, Vector4::new(90.0, 100.0, 110.0, 120.0));
    }

    #[test]
    fn matrix4_mul_scalar() {
        let m = Matrix4::new(Vector4::new(1.0, 2.0, 3.0, 4.0), Vector4::new(5.0, 6.0, 7.0, 8.0), Vector4::new(9.0, 10.0, 11.0, 12.0), Vector4::new(13.0, 14.0, 15.0, 16.0));
        assert_eq!(m * 2.0, Matrix4::new(Vector4::new(2.0, 4.0, 6.0, 8.0), Vector4::new(10.0, 12.0, 14.0, 16.0), Vector4::new(18.0, 20.0, 22.0, 24.0), Vector4::new(26.0, 28.0, 30.0, 32.0)));
        assert_eq!(2.0 * m, Matrix4::new(Vector4::new(2.0, 4.0, 6.0, 8.0), Vector4::new(10.0, 12.0, 14.0, 16.0), Vector4::new(18.0, 20.0, 22.0, 24.0), Vector4::new(26.0, 28.0, 30.0, 32.0)));
    }

    #[test]
    fn matrix4_transpose() {
        let m = Matrix4::new(Vector4::new(1.0, 2.0, 3.0, 4.0), Vector4::new(5.0, 6.0, 7.0, 8.0), Vector4::new(9.0, 10.0, 11.0, 12.0), Vector4::new(13.0, 14.0, 15.0, 16.0));
        let t = Matrix4::transpose(&m);
        assert_eq!(t, Matrix4::new(Vector4::new(1.0, 5.0, 9.0, 13.0), Vector4::new(2.0, 6.0, 10.0, 14.0), Vector4::new(3.0, 7.0, 11.0, 15.0), Vector4::new(4.0, 8.0, 12.0, 16.0)));
    }

    #[test]
    fn matrix4_determinant() {
        let a = Matrix4::new(Vector4::new(1.0, 2.0, 3.0, 4.0), Vector4::new(5.0, 6.0, 7.0, 8.0), Vector4::new(9.0, 10.0, 11.0, 12.0), Vector4::new(13.0, 14.0, 15.0, 16.0));
        let b = Matrix4::new(Vector4::new(1.0, 0.0, 0.0, 0.0), Vector4::new(0.0, 1.0, 0.0, 0.0), Vector4::new(0.0, 0.0, 1.0, 0.0), Vector4::new(0.0, 0.0, 0.0, 1.0));
        let c = Matrix4::determinant(&a);
        let d = Matrix4::determinant(&b);
        assert_eq!(c, 0.0);
        assert_eq!(d, 1.0);
    }

    #[test]
    fn matrix4_inverse() {
        let mat = Matrix4::new(Vector4::new(1.0, 0.0, 2.0, 2.0), Vector4::new(0.0, 2.0, 1.0, 0.0), Vector4::new(0.0, 1.0, 0.0, 1.0), Vector4::new(1.0, 2.0, 1.0, 4.0));
        if let Some(invmat) = Matrix4::inverse(&mat) {
            assert_eq!(mat * invmat, 1.0.into());
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
        let r = Matrix4::rotate_x(FRAC_PI_2);
        let p = Vector4::new(0.0, 1.0, 0.0, 1.0); // position
        let d = Vector4::new(0.0, 1.0, 0.0, 0.0); // direction
        assert_ulps_eq!(r * p, Vector4::new(0.0, 0.0, 1.0, 1.0));
        assert_ulps_eq!(r * d, Vector4::new(0.0, 0.0, 1.0, 0.0));
    }

    #[test]
    fn matrix4_rotate_y() {
        let r = Matrix4::rotate_y(FRAC_PI_2);
        let p = Vector4::new(1.0, 0.0, 0.0, 1.0); // position
        let d = Vector4::new(1.0, 0.0, 0.0, 0.0); // direction
        assert_ulps_eq!(r * p, Vector4::new(0.0, 0.0, -1.0, 1.0));
        assert_ulps_eq!(r * d, Vector4::new(0.0, 0.0, -1.0, 0.0));
    }

    #[test]
    fn matrix4_rotate_z() {
        let r = Matrix4::rotate_z(FRAC_PI_2);
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
