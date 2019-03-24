mod linmath;
use self::linmath::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector2_add_vector() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        let c = a + b;
        assert_eq!(c, Vector2::new(4.0, 6.0));
    }

    #[test]
    fn vector2_add_scalar() {
        let a = Vector2::new(1.0, 2.0);
        let b = 1.0;
        let c = a + b.into();
        assert_eq!(c, Vector2::new(2.0, 3.0));
    }

    #[test]
    fn vector2_add_assign_vector() {
        let mut a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        a += b;
        assert_eq!(a, Vector2::new(4.0, 6.0));
    }

    #[test]
    fn vector2_add_assign_scalar() {
        let mut a = Vector2::new(1.0, 2.0);
        let b = 1.0;
        a += b.into();
        assert_eq!(a, Vector2::new(2.0, 3.0));
    }

    #[test]
    fn vector2_sub_vector() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        let c = a - b;
        assert_eq!(c, Vector2::new(-2.0, -2.0));
    }

    #[test]
    fn vector2_sub_scalar() {
        let a = Vector2::new(1.0, 2.0);
        let b = 1.0;
        let c = a - b.into();
        assert_eq!(c, Vector2::new(0.0, 1.0));
    }

    #[test]
    fn vector2_sub_assign_vector() {
        let mut a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        a -= b;
        assert_eq!(a, Vector2::new(-2.0, -2.0));
    }

    #[test]
    fn vector2_sub_assign_scalar() {
        let mut a = Vector2::new(1.0, 2.0);
        let b = 1.0;
        a -= b.into();
        assert_eq!(a, Vector2::new(0.0, 1.0));
    }

    #[test]
    fn vector2_mul_vector() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        let c = a * b;
        assert_eq!(c, Vector2::new(3.0, 8.0));
    }

    #[test]
    fn vector2_mul_scalar() {
        let a = Vector2::new(1.0, 2.0);
        let b = 2.0;
        let c = a * b.into();
        assert_eq!(c, Vector2::new(2.0, 4.0));
    }

    #[test]
    fn vector2_mul_assign_vector() {
        let mut a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        a *= b;
        assert_eq!(a, Vector2::new(3.0, 8.0));
    }

    #[test]
    fn vector2_mul_assign_scalar() {
        let mut a = Vector2::new(1.0, 2.0);
        let b = 2.0;
        a *= b.into();
        assert_eq!(a, Vector2::new(2.0, 4.0));
    }

    #[test]
    fn vector2_div_vector() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        let c = a / b;
        assert_eq!(c, Vector2::new(1.0 / 3.0, 0.5));
    }

    #[test]
    fn vector2_div_scalar() {
        let a = Vector2::new(1.0, 2.0);
        let b = 2.0;
        let c = a / b.into();
        assert_eq!(c, Vector2::new(0.5, 1.0));
    }

    #[test]
    fn vector2_div_assign() {
        let mut a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        a /= b;
        assert_eq!(a, Vector2::new(1.0 / 3.0, 0.5));
    }

    #[test]
    fn vector2_div_assign_scalar() {
        let mut a = Vector2::new(1.0, 2.0);
        let b = 2.0;
        a /= b.into();
        assert_eq!(a, Vector2::new(0.5, 1.0));
    }

    fn vector2_dot() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(3.0, 4.0);
        let c = Vector2::dot(&a, &b);
        assert_eq!(c, 11.0);
    }

    #[test]
    fn vector3_add_vector() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        let c = a + b;
        assert_eq!(c, Vector3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn vector3_add_scalar() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = 1.0;
        let c = a + b.into();
        assert_eq!(c, Vector3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn vector3_add_assign_vector() {
        let mut a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        a += b;
        assert_eq!(a, Vector3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn vector3_add_assign_scalar() {
        let mut a = Vector3::new(1.0, 2.0, 3.0);
        let b = 1.0;
        a += b.into();
        assert_eq!(a, Vector3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn vector3_sub_vector() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        let c = a - b;
        assert_eq!(c, Vector3::new(-3.0, -3.0, -3.0));
    }

    #[test]
    fn vector3_sub_scalar() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = 1.0;
        let c = a - b.into();
        assert_eq!(c, Vector3::new(0.0, 1.0, 2.0));
    }

    #[test]
    fn vector3_sub_assign_vector() {
        let mut a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        a -= b;
        assert_eq!(a, Vector3::new(-3.0, -3.0, -3.0));
    }

    #[test]
    fn vector3_sub_assign_scalar() {
        let mut a = Vector3::new(1.0, 2.0, 3.0);
        let b = 1.0;
        a -= b.into();
        assert_eq!(a, Vector3::new(0.0, 1.0, 2.0));
    }

    #[test]
    fn vector3_mul_vector() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        let c = a * b;
        assert_eq!(c, Vector3::new(4.0, 10.0, 18.0));
    }

    #[test]
    fn vector3_mul_scalar() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = 2.0;
        let c = a * b.into();
        assert_eq!(c, Vector3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn vector3_mul_assign_vector() {
        let mut a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        a *= b;
        assert_eq!(a, Vector3::new(4.0, 10.0, 18.0));
    }

    #[test]
    fn vector3_mul_assign_scalar() {
        let mut a = Vector3::new(1.0, 2.0, 3.0);
        let b = 2.0;
        a *= b.into();
        assert_eq!(a, Vector3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn vector3_div_vector() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        let c = a / b;
        assert_eq!(c, Vector3::new(0.25, 0.4, 0.5));
    }

    #[test]
    fn vector3_div_scalar() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = 2.0;
        let c = a / b.into();
        assert_eq!(c, Vector3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn vector3_div_assign_vector() {
        let mut a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        a /= b;
        assert_eq!(a, Vector3::new(0.25, 0.4, 0.5));
    }

    #[test]
    fn vector3_div_assign_scalar() {
        let mut a = Vector3::new(1.0, 2.0, 3.0);
        let b = 2.0;
        a /= b.into();
        assert_eq!(a, Vector3::new(0.5, 1.0, 1.5));
    }
    #[test]
    fn vector3_dot() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        let c = Vector3::dot(&a, &b);
        assert_eq!(c, 32.0);
    }

    #[test]
    fn vector3_cross() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        let c = Vector3::cross(&a, &b);
        assert_eq!(c, Vector3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn vector4_add_vector() {
        let a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vector4::new(5.0, 6.0, 7.0, 8.0);
        let c = a + b;
        assert_eq!(c, Vector4::new(6.0, 8.0, 10.0, 12.0));
    }

    #[test]
    fn vector4_add_scalar() {
        let a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = 1.0;
        let c = a + b.into();
        assert_eq!(c, Vector4::new(2.0, 3.0, 4.0, 5.0));
    }

    #[test]
    fn vector4_add_assign_vector() {
        let mut a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vector4::new(5.0, 6.0, 7.0, 8.0);
        a += b;
        assert_eq!(a, Vector4::new(6.0, 8.0, 10.0, 12.0));
    }

    #[test]
    fn vector4_add_assign_scalar() {
        let mut a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = 1.0;
        a += b.into();
        assert_eq!(a, Vector4::new(2.0, 3.0, 4.0, 5.0));
    }

    #[test]
    fn vector4_sub_vector() {
        let a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vector4::new(5.0, 6.0, 7.0, 8.0);
        let c = a - b;
        assert_eq!(c, Vector4::new(-4.0, -4.0, -4.0, -4.0));
    }

    #[test]
    fn vector4_sub_scalar() {
        let a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = 1.0;
        let c = a - b.into();
        assert_eq!(c, Vector4::new(0.0, 1.0, 2.0, 3.0));
    }

    #[test]
    fn vector4_sub_assign_vector() {
        let mut a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vector4::new(5.0, 6.0, 7.0, 8.0);
        a -= b;
        assert_eq!(a, Vector4::new(-4.0, -4.0, -4.0, -4.0));
    }

    #[test]
    fn vector4_sub_assign_scalar() {
        let mut a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = 1.0;
        a -= b.into();
        assert_eq!(a, Vector4::new(0.0, 1.0, 2.0, 3.0));
    }

    #[test]
    fn vector4_mul_vector() {
        let a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vector4::new(5.0, 6.0, 7.0, 8.0);
        let c = a * b;
        assert_eq!(c, Vector4::new(5.0, 12.0, 21.0, 32.0));
    }

    #[test]
    fn vector4_mul_scalar() {
        let a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = 2.0;
        let c = a * b.into();
        assert_eq!(c, Vector4::new(2.0, 4.0, 6.0, 8.0));
    }

    #[test]
    fn vector4_mul_assign_vector() {
        let mut a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vector4::new(5.0, 6.0, 7.0, 8.0);
        a *= b;
        assert_eq!(a, Vector4::new(5.0, 12.0, 21.0, 32.0));
    }

    #[test]
    fn vector4_mul_assign_scalar() {
        let mut a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = 2.0;
        a *= b.into();
        assert_eq!(a, Vector4::new(2.0, 4.0, 6.0, 8.0));
    }

    #[test]
    fn vector4_div_vector() {
        let a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vector4::new(5.0, 6.0, 7.0, 8.0);
        let c = a / b;
        assert_eq!(c, Vector4::new(0.2, 2.0 / 6.0, 3.0 / 7.0, 0.5));
    }

    #[test]
    fn vector4_div_scalar() {
        let a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = 2.0;
        let c = a / b.into();
        assert_eq!(c, Vector4::new(0.5, 1.0, 1.5, 2.0));
    }

    #[test]
    fn vector4_div_assign_vector() {
        let mut a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vector4::new(5.0, 6.0, 7.0, 8.0);
        a /= b;
        assert_eq!(a, Vector4::new(0.2, 2.0 / 6.0, 3.0 / 7.0, 0.5));
    }

    #[test]
    fn vector4_div_assign_scalar() {
        let mut a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = 2.0;
        a /= b.into();
        assert_eq!(a, Vector4::new(0.5, 1.0, 1.5, 2.0));
    }

    #[test]
    fn vector4_dot() {
        let a = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vector4::new(5.0, 6.0, 7.0, 8.0);
        let c = Vector4::dot(&a, &b);
        assert_eq!(c, 70.0);
    }
}
