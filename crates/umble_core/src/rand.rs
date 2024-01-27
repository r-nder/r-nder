pub use self::rand::*;
pub use rand;

/// A wrapper function around the `random` function that avoids the need for specifying a type in
/// the case that it cannot be inferred. The primary purpose for this is to simplify the random API
/// for new rust users.
///
/// NOTE: This helper function relies on a thread-local RNG and is currently only available with
/// the "std" feature enabled.
#[cfg(feature = "std")]
pub fn random_f32() -> f32 {
    random()
}

/// A wrapper function around the `random` function that avoids the need for specifying a type in
/// the case that it cannot be inferred. The primary purpose for this is to simplify the random API
/// for new rust users.
///
/// NOTE: This helper function relies on a thread-local RNG and is currently only available with
/// the "std" feature enabled.
#[cfg(feature = "std")]
pub fn random_f64() -> f64 {
    random()
}

/// A function for generating a random value within the given range.
///
/// The generated value may be within the range [min, max). That is, the result is inclusive of
/// `min`, but will never be `max`.
///
/// If the given `min` is greater than the given `max`, they will be swapped before calling
/// `gen_range` internally to avoid triggering a `panic!`.
///
/// This calls `rand::thread_rng().gen_range(min..max)` internally, in turn using the thread-local
/// default random number generator.
///
/// NOTE: This helper function relies on a thread-local RNG and is currently only available with
/// the "std" feature enabled.
#[cfg(feature = "std")]
pub fn random_range<T>(min: T, max: T) -> T
    where
        T: PartialOrd + distributions::uniform::SampleUniform,
{
    let (min, max) = if min <= max {
        (min, max)
    } else {
        (max, min)
    };

    thread_rng().gen_range(min..max)
}