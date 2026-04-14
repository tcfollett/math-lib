use num_traits::{Float, FromPrimitive, Num, NumAssign};
use std::fmt::Debug;
use std::iter::{Product, Sum};

pub trait Scalar:
    Copy + Clone + Debug + PartialEq + Num + NumAssign + Sum + Product + Send + Sync + 'static
{
}

impl<T> Scalar for T where
    T: Copy + Clone + Debug + PartialEq + Num + NumAssign + Sum + Product + Send + Sync + 'static
{
}

pub trait RealScalar: Scalar + Float + FromPrimitive {}

impl<T> RealScalar for T where T: Scalar + Float + FromPrimitive {}
