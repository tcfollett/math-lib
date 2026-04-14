#![forbid(unsafe_code)]
#![doc = "Dense vectors and matrices with foundational linear algebra routines."]

mod matrix;
mod vector;

pub use matrix::{LUDecomposition, Matrix, QRDecomposition};
pub use vector::Vector;
