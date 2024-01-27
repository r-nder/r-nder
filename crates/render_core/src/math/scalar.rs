use core::ops::{AddAssign, DivAssign, MulAssign, Neg, RemAssign, SubAssign};
use num_traits::{Num, NumOps};

/// Implemented for all numeric scalar types used within `math`.
///
/// A base set of traits that must be implemented by all types used to represent scalar values
/// within the `math` module abstractions.
pub trait Scalar where
    Self:
    Clone
    + Copy
    + Num
    + PartialOrd
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + Neg<Output=Self>
{}

impl<T> Scalar for T where
    T: Clone
    + Copy
    + Num
    + NumOps
    + PartialOrd
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + Neg<Output=Self>
{}

/// The default scalar type used for geometry throughout R&nder.
pub type Default = f32;