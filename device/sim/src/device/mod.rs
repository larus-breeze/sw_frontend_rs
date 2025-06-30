#[cfg(feature = "air")]
mod air;

#[cfg(feature = "v1")]
mod v1;

#[cfg(feature = "v2")]
mod v2;

#[cfg(feature = "air")]
pub use air::src::dev_const;

#[cfg(feature = "v1")]
pub use v1::src::dev_const;

#[cfg(feature = "v2")]
pub use v2::src::dev_const;
