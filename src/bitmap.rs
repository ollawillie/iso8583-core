//! Bitmap handling for ISO 8583 messages
//!
//! The bitmap indicates which fields are present in the message.
//! - Primary bitmap: Fields 1-64
//! - Secondary bitmap: Fields 65-128 (if field 1 is set in primary)
//! - Tertiary bitmap: Fields 129-192 (if field 65 is set in secondary) [rare]

use crate::error::{ISO8583Error, Result};
use bitvec::prelude::*;

/// ISO 8583 Bitmap
///
/// Represents which fields are present in a message using a bit vector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bitmap {
    bits: BitVec<u8, Msb0>,
}

impl Bitmap {
    /// Create a new empty bitmap
    pub fn new() -> Self {
        Self {
            bits: BitVec::repeat(false, 192), // Support up to 192 fields
        }
    }

    /// Create bitmap from hex string
    pub fn from_hex(hex: &str) -> Result<Self> {
        let bytes = hex::decode(hex)
            .map_err(|e| ISO8583Error::InvalidBitmap(format!("Invalid hex: {}", e)))?;

        Self::from_bytes(&bytes)
    }

    /// Create bitmap from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.is_empty() {
            return Err(ISO8583Error::InvalidBitmap(
                "Bitmap cannot be empty".to_string(),
            ));
        }

        let mut bitmap = Self::new();
        let bits_to_copy = (bytes.len() * 8).min(192);

        for i in 0..bits_to_copy {
            let byte_idx = i / 8;
            let bit_idx = i % 8;
            let bit_value = (bytes[byte_idx] >> (7 - bit_idx)) & 1 == 1;
            bitmap.bits.set(i, bit_value);
        }

        Ok(bitmap)
    }

    /// Set a field as present
    ///
    /// Field numbers are 1-indexed (1-192)
    pub fn set(&mut self, field: u8) -> Result<()> {
        if field == 0 || field > 192 {
            return Err(ISO8583Error::InvalidFieldNumber(field));
        }

        // Convert to 0-indexed
        let idx = (field - 1) as usize;
        self.bits.set(idx, true);

        // If setting a field > 64, also set field 1 (secondary bitmap indicator)
        if field > 64 && field <= 128 {
            self.bits.set(0, true);
        }

        // If setting a field > 128, also set field 65 (tertiary bitmap indicator)
        if field > 128 {
            self.bits.set(0, true);
            self.bits.set(64, true);
        }

        Ok(())
    }

    /// Clear a field (mark as not present)
    pub fn clear(&mut self, field: u8) -> Result<()> {
        if field == 0 || field > 192 {
            return Err(ISO8583Error::InvalidFieldNumber(field));
        }

        let idx = (field - 1) as usize;
        self.bits.set(idx, false);

        Ok(())
    }

    /// Check if a field is present
    pub fn is_set(&self, field: u8) -> bool {
        if field == 0 || field > 192 {
            return false;
        }

        let idx = (field - 1) as usize;
        *self.bits.get(idx).unwrap_or(&false)
    }

    /// Get all set field numbers
    pub fn get_set_fields(&self) -> Vec<u8> {
        self.bits
            .iter()
            .enumerate()
            .filter(|(_, &bit)| bit)
            .map(|(idx, _)| (idx + 1) as u8)
            .filter(|&field| {
                // Exclude bitmap indicator fields unless they represent actual data
                field != 1 && field != 65
            })
            .collect()
    }

    /// Check if secondary bitmap is present (field 1 is set)
    pub fn has_secondary_bitmap(&self) -> bool {
        self.is_set(1)
    }

    /// Check if tertiary bitmap is present (field 65 is set)
    pub fn has_tertiary_bitmap(&self) -> bool {
        self.is_set(65)
    }

    /// Get the number of bitmaps required
    pub fn bitmap_count(&self) -> usize {
        if self.has_tertiary_bitmap() {
            3
        } else if self.has_secondary_bitmap() {
            2
        } else {
            1
        }
    }

    /// Convert to bytes (primary bitmap only)
    pub fn to_primary_bytes(&self) -> Vec<u8> {
        self.bits_to_bytes(&self.bits[0..64])
    }

    /// Convert to bytes (primary + secondary if present)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.to_primary_bytes();

        if self.has_secondary_bitmap() {
            bytes.extend_from_slice(&self.bits_to_bytes(&self.bits[64..128]));
        }

        if self.has_tertiary_bitmap() {
            bytes.extend_from_slice(&self.bits_to_bytes(&self.bits[128..192]));
        }

        bytes
    }

    /// Convert to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }

    /// Helper function to convert bit slice to bytes
    fn bits_to_bytes(&self, bits: &BitSlice<u8, Msb0>) -> Vec<u8> {
        let mut bytes = vec![0u8; bits.len() / 8];

        for (i, bit) in bits.iter().enumerate() {
            if *bit {
                let byte_idx = i / 8;
                let bit_idx = i % 8;
                bytes[byte_idx] |= 1 << (7 - bit_idx);
            }
        }

        bytes
    }

    /// Get bitmap size in bytes
    pub fn size_in_bytes(&self) -> usize {
        self.to_bytes().len()
    }
}

impl Default for Bitmap {
    fn default() -> Self {
        Self::new()
    }
}

/// Display bitmap as hex string
impl std::fmt::Display for Bitmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitmap_new() {
        let bitmap = Bitmap::new();
        assert_eq!(bitmap.get_set_fields().len(), 0);
        assert!(!bitmap.has_secondary_bitmap());
    }

    #[test]
    fn test_bitmap_set_and_is_set() {
        let mut bitmap = Bitmap::new();

        bitmap.set(2).unwrap();
        bitmap.set(3).unwrap();
        bitmap.set(4).unwrap();

        assert!(bitmap.is_set(2));
        assert!(bitmap.is_set(3));
        assert!(bitmap.is_set(4));
        assert!(!bitmap.is_set(5));
    }

    #[test]
    fn test_bitmap_clear() {
        let mut bitmap = Bitmap::new();

        bitmap.set(2).unwrap();
        assert!(bitmap.is_set(2));

        bitmap.clear(2).unwrap();
        assert!(!bitmap.is_set(2));
    }

    #[test]
    fn test_get_set_fields() {
        let mut bitmap = Bitmap::new();

        bitmap.set(2).unwrap();
        bitmap.set(3).unwrap();
        bitmap.set(11).unwrap();
        bitmap.set(41).unwrap();

        let fields = bitmap.get_set_fields();
        assert_eq!(fields, vec![2, 3, 11, 41]);
    }

    #[test]
    fn test_secondary_bitmap() {
        let mut bitmap = Bitmap::new();

        // Setting a field > 64 should automatically set field 1
        bitmap.set(70).unwrap();

        assert!(bitmap.is_set(1)); // Secondary bitmap indicator
        assert!(bitmap.is_set(70));
        assert!(bitmap.has_secondary_bitmap());
        assert_eq!(bitmap.bitmap_count(), 2);
    }

    #[test]
    fn test_tertiary_bitmap() {
        let mut bitmap = Bitmap::new();

        // Setting a field > 128 should set fields 1 and 65
        bitmap.set(150).unwrap();

        assert!(bitmap.is_set(1)); // Secondary bitmap indicator
        assert!(bitmap.is_set(65)); // Tertiary bitmap indicator
        assert!(bitmap.is_set(150));
        assert!(bitmap.has_tertiary_bitmap());
        assert_eq!(bitmap.bitmap_count(), 3);
    }

    #[test]
    fn test_to_bytes_and_from_bytes() {
        let mut bitmap = Bitmap::new();
        bitmap.set(2).unwrap();
        bitmap.set(3).unwrap();
        bitmap.set(4).unwrap();

        let bytes = bitmap.to_bytes();
        let restored = Bitmap::from_bytes(&bytes).unwrap();

        assert_eq!(bitmap, restored);
    }

    #[test]
    fn test_to_hex_and_from_hex() {
        let mut bitmap = Bitmap::new();
        bitmap.set(2).unwrap();
        bitmap.set(3).unwrap();
        bitmap.set(4).unwrap();

        let hex = bitmap.to_hex();
        let restored = Bitmap::from_hex(&hex).unwrap();

        assert_eq!(bitmap, restored);
    }

    #[test]
    fn test_common_bitmap_pattern() {
        // Common pattern for authorization request
        let mut bitmap = Bitmap::new();
        bitmap.set(2).unwrap(); // PAN
        bitmap.set(3).unwrap(); // Processing code
        bitmap.set(4).unwrap(); // Amount
        bitmap.set(7).unwrap(); // Transmission date/time
        bitmap.set(11).unwrap(); // STAN
        bitmap.set(12).unwrap(); // Local time
        bitmap.set(13).unwrap(); // Local date
        bitmap.set(22).unwrap(); // POS entry mode
        bitmap.set(41).unwrap(); // Terminal ID
        bitmap.set(42).unwrap(); // Merchant ID
        bitmap.set(49).unwrap(); // Currency code

        let fields = bitmap.get_set_fields();
        assert_eq!(fields, vec![2, 3, 4, 7, 11, 12, 13, 22, 41, 42, 49]);
    }

    #[test]
    fn test_invalid_field_numbers() {
        let mut bitmap = Bitmap::new();

        assert!(bitmap.set(0).is_err());
        assert!(bitmap.set(193).is_err());
        assert!(!bitmap.is_set(0));
        assert!(!bitmap.is_set(193));
    }

    #[test]
    fn test_bitmap_display() {
        let mut bitmap = Bitmap::new();
        bitmap.set(2).unwrap();

        let display = format!("{}", bitmap);
        assert!(display.len() > 0);
        assert!(display.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
