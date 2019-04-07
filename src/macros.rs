#[macro_export]
macro_rules! overload_arithmetic_operator {
    ($operator: ident, $lhs: ident, $rhs: ident, $output: ident, $operation: ident, $function: path) => {
        impl $operator<$rhs> for $lhs {
            type Output = $output;

            fn $operation(self, rhs: $rhs) -> $output {
                $function(&self, &rhs)
            }
        }
    }
}

#[macro_export]
macro_rules! overload_compound_assignment_operator {
    ($operator: ident, $lhs: ident, $rhs: ident, $operation: ident, $function: path) => {
        impl $operator<$rhs> for $lhs {
            fn $operation(&mut self, rhs: $rhs) {
                *self = $function(self, &rhs);
            }
        }
    }
}
