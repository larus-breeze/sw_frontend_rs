#[cfg(feature = "air")]
mod air;

#[cfg(feature = "v1")]
mod v1;

#[cfg(all(feature = "v2", not(any(feature = "v1", feature = "air"))))]
mod v2;

#[cfg(feature = "air")]
pub use air::src::dev_const;

#[cfg(feature = "v1")]
pub use v1::src::dev_const;

#[cfg(all(feature = "v2", not(any(feature = "v1", feature = "air"))))]
pub use v2::src::dev_const;
