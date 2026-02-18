//! # iso8583-core
//!
//! Production-ready ISO 8583 message parsing and generation library.
//!
//! ## Features
//!
//! - **Zero Allocation**: Static const tables, no runtime overhead
//! - **SIMD Optimized**: Accelerated bitmap operations on x86_64 and aarch64
//! - **no_std Compatible**: Works in embedded environments
//! - **Type Safe**: Compile-time field validation
//! - **High Performance**: Optimized for financial systems
//!
//! ## Quick Start
//!
//! ```rust
//! use iso8583_core::*;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Build a message
//! let message = ISO8583Message::builder()
//!     .mti(MessageType::AUTHORIZATION_REQUEST)
//!     .field(Field::PrimaryAccountNumber, "4111111111111111")
//!     .field(Field::TransactionAmount, "000000010000")
//!     .build()?;
//!
//! // Generate bytes
//! let bytes = message.to_bytes();
//!
//! // Parse bytes
//! let parsed = ISO8583Message::from_bytes(&bytes)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Architecture
//!
//! This library uses several advanced techniques for production performance:
//!
//! 1. **Static Specification Tables**: Field definitions stored in const arrays
//! 2. **Zero-Copy Parsing**: Borrows from input buffers where possible
//! 3. **SIMD Bitmap Operations**: Vectorized field presence checking
//! 4. **Compile-Time Validation**: Type system enforces correctness
//!
//! ## Feature Flags
//!
//! - `std` (default): Standard library support
//! - `alloc`: Heap allocation (Vec, String)
//! - `simd`: SIMD-accelerated bitmap operations
//! - `serde`: JSON serialization support

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

// Core modules
pub mod spec;
pub mod fields;

#[cfg(feature = "alloc")]
pub mod bitmap_simd;

#[cfg(feature = "alloc")]
pub use bitmap_simd as bitmap;

// Legacy modules (with std feature)
#[cfg(feature = "std")]
pub mod error;

#[cfg(feature = "std")]
pub mod mti;

#[cfg(feature = "std")]
pub mod encoding;

#[cfg(feature = "std")]
pub mod validation;

#[cfg(feature = "std")]
pub mod response_code;

#[cfg(feature = "std")]
pub mod processing_code;

#[cfg(feature = "std")]
pub mod utils;

#[cfg(feature = "std")]
pub mod message;

// Re-exports for convenience
pub use spec::{DataType, FieldDefinition, IsoSpec, Iso1987, LengthType};
pub use fields::IsoField;

#[cfg(feature = "alloc")]
pub use bitmap::Bitmap;

#[cfg(feature = "std")]
pub use error::{ISO8583Error, Result};

#[cfg(feature = "std")]
pub use mti::{MessageClass, MessageFunction, MessageOrigin, MessageType};

#[cfg(feature = "std")]
pub use message::{ISO8583Message, MessageBuilder};

#[cfg(feature = "std")]
pub use response_code::{ResponseCategory, ResponseCode};

#[cfg(feature = "std")]
pub use processing_code::{AccountType, ProcessingCode, TransactionType};

#[cfg(feature = "std")]
pub use validation::Validator;

// Legacy field enum (std only for compatibility)
#[cfg(feature = "std")]
pub use crate::message::Field;

// Re-export macros
pub use define_field;
pub use define_numeric_field;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spec_lookup() {
        // Verify static table works
        let field2 = Iso1987::get_field(2).unwrap();
        assert_eq!(field2.data_type, DataType::Numeric);
        assert_eq!(field2.length_type, LengthType::Llvar);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_bitmap() {
        let mut bitmap = Bitmap::new();
        bitmap.set(2).unwrap();
        assert!(bitmap.is_set(2));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_message_roundtrip() {
        let message = ISO8583Message::builder()
            .mti(MessageType::AUTHORIZATION_REQUEST)
            .field(Field::PrimaryAccountNumber, "4111111111111111")
            .field(Field::ProcessingCode, "000000")
            .field(Field::TransactionAmount, "000000010000")
            .build()
            .unwrap();

        let bytes = message.to_bytes();
        let parsed = ISO8583Message::from_bytes(&bytes).unwrap();

        assert_eq!(parsed.mti, MessageType::AUTHORIZATION_REQUEST);
    }
}
