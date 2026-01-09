//! # TJpgDec-rs - Tiny JPEG Decompressor
//! 
//! A lightweight JPEG decoder optimized for embedded systems.
//! 
//! Based on TJpgDec R0.03 (C)ChaN, 2021
//! 
//! ## Key Features
//! 
//! - **Memory pool allocation** - Predictable memory usage
//! - **Small decoder struct** - Only ~120 bytes
//! - **no_std compatible** - Works in embedded environments
//! - **Three optimization levels** - Balance speed vs memory (fast-decode-0/1/2)
//! - **No heap allocation** - All memory from user-provided pool
//! 
//! ## Example Usage
//! 
//! ```rust,no_run
//! use tjpgdec_rs::{JpegDecoder, MemoryPool, RECOMMENDED_POOL_SIZE, Result};
//! 
//! fn decode_jpeg(jpeg_data: &[u8]) -> Result<()> {
//!     // Allocate memory pool
//!     let mut pool_buffer = vec![0u8; RECOMMENDED_POOL_SIZE];
//!     let mut pool = MemoryPool::new(&mut pool_buffer);
//!     
//!     // Create decoder
//!     let mut decoder = JpegDecoder::new();
//!     
//!     // Prepare decoder
//!     decoder.prepare(jpeg_data, &mut pool)?;
//!     
//!     // Get image info
//!     let width = decoder.width();
//!     let height = decoder.height();
//!     
//!     // Decode image...
//!     Ok(())
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

mod types;
mod tables;
mod huffman;
mod idct;
mod decoder;
mod pool;

pub use types::{Result, Error, OutputFormat, Rectangle};
pub use decoder::{JpegDecoder, OutputCallback, calculate_pool_size};
pub use huffman::{HuffmanTable, BitStream};
pub use pool::{MemoryPool, RECOMMENDED_POOL_SIZE, MINIMUM_POOL_SIZE};

/// Size of stream input buffer
pub const BUFFER_SIZE: usize = 512;

/// Minimum workspace size required
/// 
/// Depends on optimization level:
/// - Level 0: 3100 bytes (basic optimization)
/// - Level 1: 3500 bytes (32-bit barrel shifter)
/// - Level 2: 9644 bytes (+ Huffman LUT)
#[cfg(feature = "fast-decode-2")]
pub const MIN_WORKSPACE_SIZE: usize = 9644;

#[cfg(all(feature = "fast-decode-1", not(feature = "fast-decode-2")))]
pub const MIN_WORKSPACE_SIZE: usize = 3500;

#[cfg(all(feature = "fast-decode-0", not(feature = "fast-decode-1"), not(feature = "fast-decode-2")))]
pub const MIN_WORKSPACE_SIZE: usize = 3100;

#[cfg(not(any(feature = "fast-decode-0", feature = "fast-decode-1", feature = "fast-decode-2")))]
pub const MIN_WORKSPACE_SIZE: usize = 3500;

/// Query the current optimization level
/// 
/// # Returns
/// 
/// - `0`: Basic optimization
/// - `1`: 32-bit optimization (default)
/// - `2`: Full optimization with LUT
pub use huffman::fastdecode_level;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        // Basic sanity test
        assert_eq!(BUFFER_SIZE, 512);
    }
}
