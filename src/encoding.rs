//! Encoding and decoding utilities for ISO 8583 messages
//!
//! Supports multiple encoding formats:
//! - ASCII: Standard text encoding
//! - BCD (Binary Coded Decimal): Compact numeric encoding
//! - EBCDIC: IBM mainframe encoding (less common)

use crate::error::{ISO8583Error, Result};

/// Encoding format for ISO 8583 messages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    /// ASCII encoding
    ASCII,
    /// Binary Coded Decimal
    BCD,
    /// EBCDIC (IBM mainframe)
    EBCDIC,
}

/// Encode numeric string to BCD
///
/// Each pair of digits is encoded into one byte.
/// If the string has odd length, a leading zero is added.
///
/// Example: "1234" -> [0x12, 0x34]
/// Example: "123" -> [0x01, 0x23]
pub fn encode_bcd(s: &str) -> Result<Vec<u8>> {
    if !s.chars().all(|c| c.is_ascii_digit()) {
        return Err(ISO8583Error::EncodingError(format!(
            "BCD encoding requires numeric input, got: {}",
            s
        )));
    }

    let mut padded = s.to_string();
    if padded.len() % 2 != 0 {
        padded.insert(0, '0');
    }

    let mut result = Vec::with_capacity(padded.len() / 2);

    for chunk in padded.as_bytes().chunks(2) {
        let high = (chunk[0] - b'0') << 4;
        let low = chunk[1] - b'0';
        result.push(high | low);
    }

    Ok(result)
}

/// Decode BCD to numeric string
pub fn decode_bcd(bytes: &[u8], length: usize) -> Result<String> {
    let mut result = String::with_capacity(length);

    for &byte in bytes {
        let high = (byte >> 4) & 0x0F;
        let low = byte & 0x0F;

        if high > 9 || low > 9 {
            return Err(ISO8583Error::EncodingError(format!(
                "Invalid BCD byte: 0x{:02X}",
                byte
            )));
        }

        result.push((b'0' + high) as char);
        result.push((b'0' + low) as char);

        if result.len() >= length {
            break;
        }
    }

    // Remove leading zeros if needed
    result.truncate(length);

    Ok(result)
}

/// Encode string to ASCII bytes
pub fn encode_ascii(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

/// Decode ASCII bytes to string
pub fn decode_ascii(bytes: &[u8]) -> Result<String> {
    std::str::from_utf8(bytes)
        .map(|s| s.to_string())
        .map_err(|e| ISO8583Error::EncodingError(format!("Invalid ASCII: {}", e)))
}

/// EBCDIC to ASCII conversion table (simplified)
const EBCDIC_TO_ASCII: &[u8; 256] = &[
    0x00, 0x01, 0x02, 0x03, 0x9C, 0x09, 0x86, 0x7F, // 0x00-0x07
    0x97, 0x8D, 0x8E, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, // 0x08-0x0F
    0x10, 0x11, 0x12, 0x13, 0x9D, 0x85, 0x08, 0x87, // 0x10-0x17
    0x18, 0x19, 0x92, 0x8F, 0x1C, 0x1D, 0x1E, 0x1F, // 0x18-0x1F
    0x80, 0x81, 0x82, 0x83, 0x84, 0x0A, 0x17, 0x1B, // 0x20-0x27
    0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x05, 0x06, 0x07, // 0x28-0x2F
    0x90, 0x91, 0x16, 0x93, 0x94, 0x95, 0x96, 0x04, // 0x30-0x37
    0x98, 0x99, 0x9A, 0x9B, 0x14, 0x15, 0x9E, 0x1A, // 0x38-0x3F
    0x20, 0xA0, 0xE2, 0xE4, 0xE0, 0xE1, 0xE3, 0xE5, // 0x40-0x47 (space, special chars)
    0xE7, 0xF1, 0xA2, 0x2E, 0x3C, 0x28, 0x2B, 0x7C, // 0x48-0x4F
    0x26, 0xE9, 0xEA, 0xEB, 0xE8, 0xED, 0xEE, 0xEF, // 0x50-0x57
    0xEC, 0xDF, 0x21, 0x24, 0x2A, 0x29, 0x3B, 0xAC, // 0x58-0x5F
    0x2D, 0x2F, 0xC2, 0xC4, 0xC0, 0xC1, 0xC3, 0xC5, // 0x60-0x67
    0xC7, 0xD1, 0xA6, 0x2C, 0x25, 0x5F, 0x3E, 0x3F, // 0x68-0x6F
    0xF8, 0xC9, 0xCA, 0xCB, 0xC8, 0xCD, 0xCE, 0xCF, // 0x70-0x77
    0xCC, 0x60, 0x3A, 0x23, 0x40, 0x27, 0x3D, 0x22, // 0x78-0x7F
    0xD8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, // 0x80-0x87 (a-g)
    0x68, 0x69, 0xAB, 0xBB, 0xF0, 0xFD, 0xFE, 0xB1, // 0x88-0x8F
    0xB0, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F, 0x70, // 0x90-0x97 (j-p)
    0x71, 0x72, 0xAA, 0xBA, 0xE6, 0xB8, 0xC6, 0xA4, // 0x98-0x9F
    0xB5, 0x7E, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, // 0xA0-0xA7 (s-x)
    0x79, 0x7A, 0xA1, 0xBF, 0xD0, 0xDD, 0xDE, 0xAE, // 0xA8-0xAF
    0x5E, 0xA3, 0xA5, 0xB7, 0xA9, 0xA7, 0xB6, 0xBC, // 0xB0-0xB7
    0xBD, 0xBE, 0x5B, 0x5D, 0xAF, 0xA8, 0xB4, 0xD7, // 0xB8-0xBF
    0x7B, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, // 0xC0-0xC7 (A-G)
    0x48, 0x49, 0xAD, 0xF4, 0xF6, 0xF2, 0xF3, 0xF5, // 0xC8-0xCF
    0x7D, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x50, // 0xD0-0xD7 (J-P)
    0x51, 0x52, 0xB9, 0xFB, 0xFC, 0xF9, 0xFA, 0xFF, // 0xD8-0xDF
    0x5C, 0xF7, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, // 0xE0-0xE7 (S-X)
    0x59, 0x5A, 0xB2, 0xD4, 0xD6, 0xD2, 0xD3, 0xD5, // 0xE8-0xEF
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, // 0xF0-0xF7 (0-7)
    0x38, 0x39, 0xB3, 0xDB, 0xDC, 0xD9, 0xDA, 0x9F, // 0xF8-0xFF (8-9)
];

/// Encode string to EBCDIC bytes
pub fn encode_ebcdic(s: &str) -> Result<Vec<u8>> {
    let mut result = Vec::with_capacity(s.len());

    for byte in s.as_bytes() {
        // Find ASCII byte in conversion table
        let ebcdic = EBCDIC_TO_ASCII
            .iter()
            .position(|&b| b == *byte)
            .ok_or_else(|| {
                ISO8583Error::EncodingError(format!("Cannot encode byte to EBCDIC: 0x{:02X}", byte))
            })?;

        result.push(ebcdic as u8);
    }

    Ok(result)
}

/// Decode EBCDIC bytes to string
pub fn decode_ebcdic(bytes: &[u8]) -> Result<String> {
    let ascii_bytes: Vec<u8> = bytes.iter().map(|&b| EBCDIC_TO_ASCII[b as usize]).collect();

    decode_ascii(&ascii_bytes)
}

/// Encode length indicator (for LLVAR and LLLVAR fields)
pub fn encode_length(length: usize, digits: usize, encoding: Encoding) -> Result<Vec<u8>> {
    let length_str = format!("{:0width$}", length, width = digits);

    match encoding {
        Encoding::ASCII => Ok(encode_ascii(&length_str)),
        Encoding::BCD => encode_bcd(&length_str),
        Encoding::EBCDIC => encode_ebcdic(&length_str),
    }
}

/// Decode length indicator
pub fn decode_length(bytes: &[u8], digits: usize, encoding: Encoding) -> Result<usize> {
    let length_str = match encoding {
        Encoding::ASCII => decode_ascii(bytes)?,
        Encoding::BCD => decode_bcd(bytes, digits)?,
        Encoding::EBCDIC => decode_ebcdic(bytes)?,
    };

    length_str
        .parse()
        .map_err(|e| ISO8583Error::EncodingError(format!("Invalid length value: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bcd_encoding() {
        let encoded = encode_bcd("1234").unwrap();
        assert_eq!(encoded, vec![0x12, 0x34]);

        let encoded = encode_bcd("123").unwrap();
        assert_eq!(encoded, vec![0x01, 0x23]);
    }

    #[test]
    fn test_bcd_decoding() {
        let decoded = decode_bcd(&[0x12, 0x34], 4).unwrap();
        assert_eq!(decoded, "1234");

        let decoded = decode_bcd(&[0x01, 0x23], 3).unwrap();
        assert_eq!(decoded, "012");
    }

    #[test]
    fn test_ascii_encoding() {
        let encoded = encode_ascii("Hello");
        assert_eq!(encoded, b"Hello");
    }

    #[test]
    fn test_ascii_decoding() {
        let decoded = decode_ascii(b"Hello").unwrap();
        assert_eq!(decoded, "Hello");
    }

    #[test]
    fn test_length_encoding_ascii() {
        let encoded = encode_length(12, 2, Encoding::ASCII).unwrap();
        assert_eq!(encoded, b"12");

        let encoded = encode_length(123, 3, Encoding::ASCII).unwrap();
        assert_eq!(encoded, b"123");
    }

    #[test]
    fn test_length_decoding_ascii() {
        let decoded = decode_length(b"12", 2, Encoding::ASCII).unwrap();
        assert_eq!(decoded, 12);

        let decoded = decode_length(b"123", 3, Encoding::ASCII).unwrap();
        assert_eq!(decoded, 123);
    }

    #[test]
    fn test_length_encoding_bcd() {
        let encoded = encode_length(12, 2, Encoding::BCD).unwrap();
        assert_eq!(encoded, vec![0x12]);
    }

    #[test]
    fn test_invalid_bcd_input() {
        assert!(encode_bcd("12A4").is_err());
        assert!(encode_bcd("ABCD").is_err());
    }

    #[test]
    fn test_ebcdic_numbers() {
        // Test numbers 0-9
        let encoded = encode_ebcdic("0123456789").unwrap();
        let decoded = decode_ebcdic(&encoded).unwrap();
        assert_eq!(decoded, "0123456789");
    }

    #[test]
    fn test_ebcdic_letters() {
        // Test letters
        let encoded = encode_ebcdic("ABCDEFGHIJKLMNOPQRSTUVWXYZ").unwrap();
        let decoded = decode_ebcdic(&encoded).unwrap();
        assert_eq!(decoded, "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
}
