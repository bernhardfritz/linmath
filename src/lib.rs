pub use crate::vector::*;
pub use crate::matrix::*;
#[macro_use]
mod macros;
mod vector;
mod matrix;
#[cfg(test)]
#[macro_use]
extern crate approx;
#[cfg(not(test))]
extern crate approx;
