use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

macro_rules! sum {
    ($h:expr) => ($h);
    ($h:expr, $($t:expr),*) => ($h + sum!($($t),*));
}

macro_rules! overload_arithmetic_operator {
    ($operator: ident, $t: ident, $operation: ident, $function: ident) => {
        impl $operator for $t {
            type Output = $t;

            fn $operation(self, rhs: $t) -> $t {
                $t::$function(&self, &rhs)
            }
        }
    }
}

macro_rules! overload_compound_assignment_operator {
    ($operator: ident, $t: ident, $operation: ident, $function: ident) => {
        impl $operator<$t> for $t {
            fn $operation(&mut self, rhs: $t) {
                *self = $t::$function(self, &rhs);
            }
        }
    }
}

macro_rules! generate_vector_n {
    ($VectorN: ident, $($field: ident),+) => {
        #[derive(Debug, PartialEq)]
        pub struct $VectorN {
            $(pub $field: f64),+
        }

        impl $VectorN {
            pub fn new($($field: f64),+) -> $VectorN {
                $VectorN { $($field: $field),+ }
            }

            fn add(a: &$VectorN, b: &$VectorN) -> $VectorN {
                $VectorN { $($field: a.$field + b.$field),+ }
            }

            fn sub(a: &$VectorN, b: &$VectorN) -> $VectorN {
                $VectorN { $($field: a.$field - b.$field),+ }
            }

            fn mul(a: &$VectorN, b: &$VectorN) -> $VectorN {
                $VectorN { $($field: a.$field * b.$field),+ }
            }

            fn div(a: &$VectorN, b: &$VectorN) -> $VectorN {
                $VectorN { $($field: a.$field / b.$field),+ }
            }

            pub fn dot(a: &$VectorN, b: &$VectorN) -> f64 {
                sum!($(a.$field * b.$field),+)
            }
        }

        overload_arithmetic_operator!(Add, $VectorN, add, add);
        overload_arithmetic_operator!(Sub, $VectorN, sub, sub);
        overload_arithmetic_operator!(Mul, $VectorN, mul, mul);
        overload_arithmetic_operator!(Div, $VectorN, div, div);
        overload_compound_assignment_operator!(AddAssign, $VectorN, add_assign, add);
        overload_compound_assignment_operator!(SubAssign, $VectorN, sub_assign, sub);
        overload_compound_assignment_operator!(MulAssign, $VectorN, mul_assign, mul);
        overload_compound_assignment_operator!(DivAssign, $VectorN, div_assign, div);

        impl From<f64> for $VectorN {
            fn from(scalar: f64) -> Self {
                $VectorN { $($field: scalar),+ }
            }
        }
    };
}

generate_vector_n!(Vector2, x, y);
generate_vector_n!(Vector3, x, y, z);
generate_vector_n!(Vector4, x, y, z, w);

impl Vector3 {
    pub fn cross(a: &Vector3, b: &Vector3) -> Vector3 {
        Vector3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }
}
