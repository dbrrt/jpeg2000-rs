#![forbid(unsafe_code)]

//! JPEG 2000 building blocks in idiomatic Rust.
//!
//! Your 2016 Python prototype explored wavelets + colour conversion with Haar
//! analysis and visualisation helpers. This crate upgrades the **numeric core**
//! toward real JPEG 2000 Part‑1:
//!
//! - reversible **Le Gall 5 / 3** lifting (lossless path),
//! - the normative **reversible colour transform**, and
//! - the **MQ arithmetic coder** foundation required by EBCOT Tier‑1.
//!
//! Producing decodable JP2/J2K codestreams still requires Tier‑1 bit‑plane passes,
//! Tier‑2 packetisation, and box wrappers—the next layers we expect to grow here.

pub mod color;
pub mod dwt;
pub mod error;
pub mod mq;

pub use error::{Error, Result};
