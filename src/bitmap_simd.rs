//! SIMD-Optimized Bitmap Operations
//!
//! High-performance bitmap parsing with SIMD acceleration where available.
//! Falls back to scalar operations on unsupported platforms.

#![cfg_attr(not(feature = "std"), no_std)]

/// Bitmap for tracking present fields (supports up to 192 fields)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bitmap {
    /// Primary bitmap (fields 1-64)
    primary: [u8; 8],
    /// Secondary bitmap (fields 65-128), if field 1 is set
    secondary: Option<[u8; 8]>,
    /// Tertiary bitmap (fields 129-192), if field 65 is set
    tertiary: Option<[u8; 8]>,
}

impl Bitmap {
    /// Create a new empty bitmap
    #[inline]
    pub const fn new() -> Self {
        Self {
            primary: [0u8; 8],
            secondary: None,
            tertiary: None,
        }
    }

    /// Check if field is set using SIMD where available
    #[inline]
    pub fn is_set(&self, field: u8) -> bool {
        if field == 0 || field > 192 {
            return false;
        }

        match field {
            1..=64 => self.is_set_in_bitmap(&self.primary, field),
            65..=128 => {
                if let Some(ref secondary) = self.secondary {
                    self.is_set_in_bitmap(secondary, field - 64)
                } else {
                    false
                }
            }
            129..=192 => {
                if let Some(ref tertiary) = self.tertiary {
                    self.is_set_in_bitmap(tertiary, field - 128)
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Set a field in the bitmap
    #[inline]
    pub fn set(&mut self, field: u8) -> Result<(), &'static str> {
        if field == 0 || field > 192 {
            return Err("Field number out of range (1-192)");
        }

        match field {
            1 => {
                // Setting field 1 means secondary bitmap will be present
                self.set_in_bitmap(&mut self.primary, 1);
                if self.secondary.is_none() {
                    self.secondary = Some([0u8; 8]);
                }
            }
            2..=64 => {
                self.set_in_bitmap(&mut self.primary, field);
            }
            65 => {
                // Setting field 65 means tertiary bitmap will be present
                if self.secondary.is_none() {
                    self.secondary = Some([0u8; 8]);
                    self.set_in_bitmap(&mut self.primary, 1); // Enable secondary
                }
                if let Some(ref mut secondary) = self.secondary {
                    self.set_in_bitmap(secondary, 1);
                    if self.tertiary.is_none() {
                        self.tertiary = Some([0u8; 8]);
                    }
                }
            }
            66..=128 => {
                if self.secondary.is_none() {
                    self.secondary = Some([0u8; 8]);
                    self.set_in_bitmap(&mut self.primary, 1); // Enable secondary
                }
                if let Some(ref mut secondary) = self.secondary {
                    self.set_in_bitmap(secondary, field - 64);
                }
            }
            129..=192 => {
                // Ensure secondary and tertiary exist
                if self.secondary.is_none() {
                    self.secondary = Some([0u8; 8]);
                    self.set_in_bitmap(&mut self.primary, 1);
                }
                if let Some(ref mut secondary) = self.secondary {
                    if self.tertiary.is_none() {
                        self.tertiary = Some([0u8; 8]);
                        self.set_in_bitmap(secondary, 1); // Enable tertiary
                    }
                }
                if let Some(ref mut tertiary) = self.tertiary {
                    self.set_in_bitmap(tertiary, field - 128);
                }
            }
            _ => return Err("Field number out of range"),
        }

        Ok(())
    }

    /// Clear a field in the bitmap
    #[inline]
    pub fn clear(&mut self, field: u8) -> Result<(), &'static str> {
        if field == 0 || field > 192 {
            return Err("Field number out of range (1-192)");
        }

        match field {
            1..=64 => {
                self.clear_in_bitmap(&mut self.primary, field);
            }
            65..=128 => {
                if let Some(ref mut secondary) = self.secondary {
                    self.clear_in_bitmap(secondary, field - 64);
                }
            }
            129..=192 => {
                if let Some(ref mut tertiary) = self.tertiary {
                    self.clear_in_bitmap(tertiary, field - 128);
                }
            }
            _ => return Err("Field number out of range"),
        }

        Ok(())
    }

    /// Check if bitmap is empty (SIMD optimized)
    #[inline]
    pub fn is_empty(&self) -> bool {
        !self.has_any_set(&self.primary)
            && !self.secondary.as_ref().map_or(false, |s| self.has_any_set(s))
            && !self.tertiary.as_ref().map_or(false, |t| self.has_any_set(t))
    }

    /// Get all set field numbers
    pub fn get_set_fields(&self) -> alloc::vec::Vec<u8> {
        let mut fields = alloc::vec::Vec::with_capacity(32);

        // Primary bitmap (fields 1-64)
        for field in 1..=64 {
            if self.is_set_in_bitmap(&self.primary, field) {
                fields.push(field);
            }
        }

        // Secondary bitmap (fields 65-128)
        if let Some(ref secondary) = self.secondary {
            for field in 1..=64 {
                if self.is_set_in_bitmap(secondary, field) {
                    fields.push(field + 64);
                }
            }
        }

        // Tertiary bitmap (fields 129-192)
        if let Some(ref tertiary) = self.tertiary {
            for field in 1..=64 {
                if self.is_set_in_bitmap(tertiary, field) {
                    fields.push(field + 128);
                }
            }
        }

        fields
    }

    /// Convert to bytes for transmission
    pub fn to_bytes(&self) -> alloc::vec::Vec<u8> {
        let mut bytes = alloc::vec::Vec::with_capacity(24);
        bytes.extend_from_slice(&self.primary);

        if let Some(ref secondary) = self.secondary {
            bytes.extend_from_slice(secondary);
        }

        if let Some(ref tertiary) = self.tertiary {
            bytes.extend_from_slice(tertiary);
        }

        bytes
    }

    /// Parse from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < 8 {
            return Err("Bitmap must be at least 8 bytes");
        }

        let mut primary = [0u8; 8];
        primary.copy_from_slice(&bytes[0..8]);

        let mut bitmap = Self {
            primary,
            secondary: None,
            tertiary: None,
        };

        // Check for secondary bitmap (field 1)
        if bitmap.is_set(1) && bytes.len() >= 16 {
            let mut secondary = [0u8; 8];
            secondary.copy_from_slice(&bytes[8..16]);
            bitmap.secondary = Some(secondary);

            // Check for tertiary bitmap (field 65)
            if bitmap.is_set(65) && bytes.len() >= 24 {
                let mut tertiary = [0u8; 8];
                tertiary.copy_from_slice(&bytes[16..24]);
                bitmap.tertiary = Some(tertiary);
            }
        }

        Ok(bitmap)
    }

    // ===== Internal Helper Methods =====

    /// Check if specific field is set in 8-byte bitmap
    #[inline]
    fn is_set_in_bitmap(&self, bitmap: &[u8; 8], field: u8) -> bool {
        if field == 0 || field > 64 {
            return false;
        }

        let byte_index = ((field - 1) / 8) as usize;
        let bit_index = 7 - ((field - 1) % 8);

        bitmap[byte_index] & (1 << bit_index) != 0
    }

    /// Set specific field in 8-byte bitmap
    #[inline]
    fn set_in_bitmap(&self, bitmap: &mut [u8; 8], field: u8) {
        if field == 0 || field > 64 {
            return;
        }

        let byte_index = ((field - 1) / 8) as usize;
        let bit_index = 7 - ((field - 1) % 8);

        bitmap[byte_index] |= 1 << bit_index;
    }

    /// Clear specific field in 8-byte bitmap
    #[inline]
    fn clear_in_bitmap(&self, bitmap: &mut [u8; 8], field: u8) {
        if field == 0 || field > 64 {
            return;
        }

        let byte_index = ((field - 1) / 8) as usize;
        let bit_index = 7 - ((field - 1) % 8);

        bitmap[byte_index] &= !(1 << bit_index);
    }

    /// SIMD-optimized check for any set bits (x86_64)
    #[cfg(all(feature = "simd", target_arch = "x86_64", target_feature = "sse2"))]
    #[inline]
    fn has_any_set(&self, bitmap: &[u8; 8]) -> bool {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            use core::arch::x86_64::*;
            let ptr = bitmap.as_ptr() as *const __m128i;
            let value = _mm_loadl_epi64(ptr);
            _mm_testz_si128(value, value) == 0
        }
    }

    /// SIMD-optimized check for any set bits (aarch64/ARM NEON)
    #[cfg(all(feature = "simd", target_arch = "aarch64", target_feature = "neon"))]
    #[inline]
    fn has_any_set(&self, bitmap: &[u8; 8]) -> bool {
        #[cfg(target_arch = "aarch64")]
        unsafe {
            use core::arch::aarch64::*;
            let value = vld1_u8(bitmap.as_ptr());
            let zero = vdup_n_u8(0);
            let cmp = vceq_u8(value, zero);
            vminv_u8(cmp) == 0
        }
    }

    /// Fallback scalar implementation
    #[cfg(not(all(feature = "simd", any(
        all(target_arch = "x86_64", target_feature = "sse2"),
        all(target_arch = "aarch64", target_feature = "neon")
    ))))]
    #[inline]
    fn has_any_set(&self, bitmap: &[u8; 8]) -> bool {
        bitmap.iter().any(|&b| b != 0)
    }
}

impl Default for Bitmap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    use super::*;

    #[test]
    fn test_new_bitmap() {
        let bitmap = Bitmap::new();
        assert!(bitmap.is_empty());
    }

    #[test]
    fn test_set_and_check() {
        let mut bitmap = Bitmap::new();
        assert!(bitmap.set(2).is_ok());
        assert!(bitmap.is_set(2));
        assert!(!bitmap.is_set(3));
    }

    #[test]
    fn test_clear() {
        let mut bitmap = Bitmap::new();
        bitmap.set(2).unwrap();
        assert!(bitmap.is_set(2));
        bitmap.clear(2).unwrap();
        assert!(!bitmap.is_set(2));
    }

    #[test]
    fn test_secondary_bitmap() {
        let mut bitmap = Bitmap::new();
        bitmap.set(70).unwrap();
        assert!(bitmap.is_set(1)); // Secondary indicator should be set
        assert!(bitmap.is_set(70));
    }

    #[test]
    fn test_roundtrip() {
        let mut bitmap = Bitmap::new();
        bitmap.set(2).unwrap();
        bitmap.set(3).unwrap();
        bitmap.set(4).unwrap();

        let bytes = bitmap.to_bytes();
        let restored = Bitmap::from_bytes(&bytes).unwrap();

        assert_eq!(bitmap, restored);
    }

    #[test]
    fn test_get_set_fields() {
        let mut bitmap = Bitmap::new();
        bitmap.set(2).unwrap();
        bitmap.set(4).unwrap();
        bitmap.set(11).unwrap();

        let fields = bitmap.get_set_fields();
        // Field 1 is also set (secondary indicator)
        assert!(fields.contains(&2));
        assert!(fields.contains(&4));
        assert!(fields.contains(&11));
    }

    #[test]
    fn test_bounds() {
        let mut bitmap = Bitmap::new();
        assert!(bitmap.set(0).is_err());
        assert!(bitmap.set(193).is_err());
    }
}
