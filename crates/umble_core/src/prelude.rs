//! A collection of commonly used items that are generally useful to have in scope.

// // NOTE: These helper functions rely on a thread-local RNG and are currently only available via std.
#[cfg(feature = "std")]
pub use crate::rand::{random_f32, random_f64, random_range};

pub use core::f32::consts::{PI, TAU};
pub use core::f64::consts::{PI as PI_F64, TAU as TAU_F64};