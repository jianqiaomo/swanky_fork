//! This crate contains base types and traits for our finite field types.
//!
//! See [`FiniteField`] and [`FiniteRing`] to get started.

#![deny(missing_docs)]
mod field;
mod ring;
pub use field::*;
pub use ring::*;

/// Dependent crates might not neccessarily depend on `num_traits`, for example, themsevles.
/// Nonetheless, macros written in _this_ crate need to be able to access `num_traits`, even when
/// those macros are invoked from other crates. To solve this problem, we re-export the crates that
/// our macros need.
#[doc(hidden)]
pub mod __macro_export {
    pub use num_traits;
    pub use rand;
    pub use swanky_serialization;
}
