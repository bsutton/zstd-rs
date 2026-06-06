//! Faithful Rust port of the upstream C compressor.
//!
//! The existing encoder remains the active implementation while this module is
//! built out and checked against the C reference. Keep C-derived behavior here
//! until it has enough parity coverage to replace the current strategy code.

mod fast;
mod params;
mod sequence_store;

#[cfg(test)]
mod fast_tests;
#[cfg(test)]
mod params_tests;
#[cfg(test)]
mod sequence_store_tests;

#[cfg(test)]
pub(crate) use params::{
    CompressionParameters, Strategy, MAX_COMPRESSION_LEVEL, MIN_COMPRESSION_LEVEL,
};
