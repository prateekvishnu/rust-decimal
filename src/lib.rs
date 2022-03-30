#![doc = include_str!(concat!(env!("OUT_DIR"), "/README-lib.md"))]
#![forbid(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

mod constants;
mod decimal;
mod error;
mod ops;
mod str;

// We purposely place this here for documentation ordering
mod arithmetic_impls;

#[cfg(feature = "rust-fuzz")]
mod fuzz;
#[cfg(feature = "maths")]
mod maths;
#[cfg(any(feature = "db-diesel-mysql"))]
mod mysql;
#[cfg(any(
    feature = "db-tokio-postgres",
    feature = "db-postgres",
    feature = "db-diesel-postgres",
))]
mod postgres;
#[cfg(feature = "rocket-traits")]
mod rocket;
#[cfg(all(
    feature = "serde",
    not(any(
        feature = "serde-with-str",
        feature = "serde-with-float",
        feature = "serde-with-arbitrary-precision"
    ))
))]
mod serde;
/// Serde specific functionality to customize how a decimal is serialized/deserialized (`serde_with`)
#[cfg(all(
    feature = "serde",
    any(
        feature = "serde-with-str",
        feature = "serde-with-float",
        feature = "serde-with-arbitrary-precision"
    )
))]
pub mod serde;

// Experimental variable support. Used for maths only at the moment.
#[cfg(all(feature = "maths", not(feature = "legacy-ops")))]
pub(crate) mod sized;

// Coming soon...
// #[cfg(feature = "legacy-ops")]
// compile_error!("legacy-ops feature no longer supported.");

pub use decimal::{Decimal, RoundingStrategy};
pub use error::Error;
#[cfg(feature = "maths")]
pub use maths::MathematicalOps;

/// A convenience module appropriate for glob imports (`use rust_decimal::prelude::*;`).
pub mod prelude {
    #[cfg(feature = "maths")]
    pub use crate::maths::MathematicalOps;
    pub use crate::{Decimal, RoundingStrategy};
    pub use core::str::FromStr;
    pub use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
}

#[cfg(feature = "diesel")]
#[macro_use]
extern crate diesel;

/// Shortcut for `core::result::Result<T, rust_decimal::Error>`. Useful to distinguish
/// between `rust_decimal` and `std` types.
pub type Result<T> = core::result::Result<T, Error>;

// #[cfg(feature = "legacy-ops")]
// compiler_error!("legacy-ops has been removed as 1.x");
