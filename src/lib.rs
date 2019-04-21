#[cfg(test)]
#[macro_use]
extern crate approx;
#[cfg(not(test))]
extern crate approx;
extern crate num_traits;

pub use vector::*;
pub use matrix::*;

mod vector;
mod matrix;