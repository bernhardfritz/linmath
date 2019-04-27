#[cfg(test)]
#[macro_use]
extern crate approx;
#[cfg(not(test))]
extern crate approx;
extern crate num_traits;

pub use matrix::*;
pub use vector::*;

mod matrix;
mod vector;
