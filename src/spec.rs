//! ISO 8583 Field Specifications - Zero-Allocation Static Tables
//!
//! This module provides compile-time field definitions with zero runtime overhead.
//! All field metadata is stored in static const tables.

#![cfg_attr(not(feature = "std"), no_std)]

/// Data type for field values
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// Numeric digits only (0-9)
    Numeric = 0,
    /// Alphabetic characters only (A-Z, a-z)
    Alpha = 1,
    /// Alphanumeric (0-9, A-Z, a-z)
    Alphanumeric = 2,
    /// Alphanumeric with special characters
    AlphanumericSpecial = 3,
    /// Binary data
    Binary = 4,
    /// Track 2 magnetic stripe format
    Track2 = 5,
    /// Track 3 magnetic stripe format
    Track3 = 6,
}

/// Length encoding type for field
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthType {
    /// Fixed length (no length indicator)
    Fixed = 0,
    /// Variable length with 2-digit length indicator (LLVAR)
    Llvar = 1,
    /// Variable length with 3-digit length indicator (LLLVAR)
    Lllvar = 2,
}

/// Field definition - small, copyable, stored in static memory
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldDefinition {
    /// Data type of the field
    pub data_type: DataType,
    /// Length encoding type
    pub length_type: LengthType,
    /// Maximum length in bytes
    pub max_len: u16,
}

impl FieldDefinition {
    /// Create a new field definition
    #[inline]
    pub const fn new(data_type: DataType, length_type: LengthType, max_len: u16) -> Self {
        Self {
            data_type,
            length_type,
            max_len,
        }
    }

    /// Create a fixed-length field
    #[inline]
    pub const fn fixed(data_type: DataType, len: u16) -> Self {
        Self::new(data_type, LengthType::Fixed, len)
    }

    /// Create an LLVAR field
    #[inline]
    pub const fn llvar(data_type: DataType, max_len: u16) -> Self {
        Self::new(data_type, LengthType::Llvar, max_len)
    }

    /// Create an LLLVAR field
    #[inline]
    pub const fn lllvar(data_type: DataType, max_len: u16) -> Self {
        Self::new(data_type, LengthType::Lllvar, max_len)
    }
}

/// Macro to generate ISO 8583 field specification table
macro_rules! iso_table {
    ($($field:expr => $def:expr),* $(,)?) => {{
        let mut table: [Option<FieldDefinition>; 129] = [None; 129];
        $(
            table[$field] = Some($def);
        )*
        table
    }};
}

/// ISO 8583:1987 Specification Table
///
/// This is a compile-time const array with zero runtime overhead.
/// Field lookup is O(1) with no heap allocation.
pub const ISO8583_1987_TABLE: [Option<FieldDefinition>; 129] = iso_table! {
    // Field 1: Secondary Bitmap (binary, fixed 8 bytes)
    1 => FieldDefinition::fixed(DataType::Binary, 8),

    // Field 2: Primary Account Number (numeric, LLVAR, max 19)
    2 => FieldDefinition::llvar(DataType::Numeric, 19),

    // Field 3: Processing Code (numeric, fixed 6)
    3 => FieldDefinition::fixed(DataType::Numeric, 6),

    // Field 4: Transaction Amount (numeric, fixed 12)
    4 => FieldDefinition::fixed(DataType::Numeric, 12),

    // Field 5: Settlement Amount (numeric, fixed 12)
    5 => FieldDefinition::fixed(DataType::Numeric, 12),

    // Field 6: Cardholder Billing Amount (numeric, fixed 12)
    6 => FieldDefinition::fixed(DataType::Numeric, 12),

    // Field 7: Transmission Date & Time (numeric, fixed 10 - MMDDhhmmss)
    7 => FieldDefinition::fixed(DataType::Numeric, 10),

    // Field 8: Cardholder Billing Fee Amount (numeric, fixed 8)
    8 => FieldDefinition::fixed(DataType::Numeric, 8),

    // Field 9: Settlement Conversion Rate (numeric, fixed 8)
    9 => FieldDefinition::fixed(DataType::Numeric, 8),

    // Field 10: Cardholder Billing Conversion Rate (numeric, fixed 8)
    10 => FieldDefinition::fixed(DataType::Numeric, 8),

    // Field 11: System Trace Audit Number (numeric, fixed 6)
    11 => FieldDefinition::fixed(DataType::Numeric, 6),

    // Field 12: Local Transaction Time (numeric, fixed 6 - hhmmss)
    12 => FieldDefinition::fixed(DataType::Numeric, 6),

    // Field 13: Local Transaction Date (numeric, fixed 4 - MMDD)
    13 => FieldDefinition::fixed(DataType::Numeric, 4),

    // Field 14: Expiration Date (numeric, fixed 4 - YYMM)
    14 => FieldDefinition::fixed(DataType::Numeric, 4),

    // Field 15: Settlement Date (numeric, fixed 4 - MMDD)
    15 => FieldDefinition::fixed(DataType::Numeric, 4),

    // Field 16: Currency Conversion Date (numeric, fixed 4)
    16 => FieldDefinition::fixed(DataType::Numeric, 4),

    // Field 17: Capture Date (numeric, fixed 4)
    17 => FieldDefinition::fixed(DataType::Numeric, 4),

    // Field 18: Merchant Type (numeric, fixed 4)
    18 => FieldDefinition::fixed(DataType::Numeric, 4),

    // Field 19: Acquiring Institution Country Code (numeric, fixed 3)
    19 => FieldDefinition::fixed(DataType::Numeric, 3),

    // Field 20: PAN Extended Country Code (numeric, fixed 3)
    20 => FieldDefinition::fixed(DataType::Numeric, 3),

    // Field 21: Forwarding Institution Country Code (numeric, fixed 3)
    21 => FieldDefinition::fixed(DataType::Numeric, 3),

    // Field 22: Point of Service Entry Mode (numeric, fixed 3)
    22 => FieldDefinition::fixed(DataType::Numeric, 3),

    // Field 23: Card Sequence Number (numeric, fixed 3)
    23 => FieldDefinition::fixed(DataType::Numeric, 3),

    // Field 24: Function Code (numeric, fixed 3)
    24 => FieldDefinition::fixed(DataType::Numeric, 3),

    // Field 25: Point of Service Condition Code (numeric, fixed 2)
    25 => FieldDefinition::fixed(DataType::Numeric, 2),

    // Field 26: Point of Service Capture Code (numeric, fixed 2)
    26 => FieldDefinition::fixed(DataType::Numeric, 2),

    // Field 27: Authorization Identification Response Length (numeric, fixed 1)
    27 => FieldDefinition::fixed(DataType::Numeric, 1),

    // Field 28: Transaction Fee Amount (numeric, fixed 9)
    28 => FieldDefinition::fixed(DataType::Numeric, 9),

    // Field 29: Settlement Fee Amount (numeric, fixed 9)
    29 => FieldDefinition::fixed(DataType::Numeric, 9),

    // Field 30: Transaction Processing Fee Amount (numeric, fixed 9)
    30 => FieldDefinition::fixed(DataType::Numeric, 9),

    // Field 31: Settlement Processing Fee Amount (numeric, fixed 9)
    31 => FieldDefinition::fixed(DataType::Numeric, 9),

    // Field 32: Acquiring Institution ID Code (LLVAR, max 11)
    32 => FieldDefinition::llvar(DataType::Numeric, 11),

    // Field 33: Forwarding Institution ID Code (LLVAR, max 11)
    33 => FieldDefinition::llvar(DataType::Numeric, 11),

    // Field 34: Extended PAN (LLVAR, max 28)
    34 => FieldDefinition::llvar(DataType::Alphanumeric, 28),

    // Field 35: Track 2 Data (LLVAR, max 37)
    35 => FieldDefinition::llvar(DataType::Track2, 37),

    // Field 36: Track 3 Data (LLLVAR, max 104)
    36 => FieldDefinition::lllvar(DataType::Track3, 104),

    // Field 37: Retrieval Reference Number (alphanumeric, fixed 12)
    37 => FieldDefinition::fixed(DataType::Alphanumeric, 12),

    // Field 38: Authorization ID Response (alphanumeric, fixed 6)
    38 => FieldDefinition::fixed(DataType::Alphanumeric, 6),

    // Field 39: Response Code (alphanumeric, fixed 2)
    39 => FieldDefinition::fixed(DataType::Alphanumeric, 2),

    // Field 40: Service Restriction Code (alphanumeric, fixed 3)
    40 => FieldDefinition::fixed(DataType::Alphanumeric, 3),

    // Field 41: Card Acceptor Terminal ID (alphanumeric special, fixed 8)
    41 => FieldDefinition::fixed(DataType::AlphanumericSpecial, 8),

    // Field 42: Card Acceptor ID Code (alphanumeric special, fixed 15)
    42 => FieldDefinition::fixed(DataType::AlphanumericSpecial, 15),

    // Field 43: Card Acceptor Name/Location (alphanumeric special, fixed 40)
    43 => FieldDefinition::fixed(DataType::AlphanumericSpecial, 40),

    // Field 44: Additional Response Data (LLVAR, max 25)
    44 => FieldDefinition::llvar(DataType::AlphanumericSpecial, 25),

    // Field 45: Track 1 Data (LLVAR, max 76)
    45 => FieldDefinition::llvar(DataType::AlphanumericSpecial, 76),

    // Field 46: Additional Data - ISO (LLLVAR, max 999)
    46 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),

    // Field 47: Additional Data - National (LLLVAR, max 999)
    47 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),

    // Field 48: Additional Data - Private (LLLVAR, max 999)
    48 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),

    // Field 49: Currency Code, Transaction (alphanumeric, fixed 3)
    49 => FieldDefinition::fixed(DataType::Alphanumeric, 3),

    // Field 50: Currency Code, Settlement (alphanumeric, fixed 3)
    50 => FieldDefinition::fixed(DataType::Alphanumeric, 3),

    // Field 51: Currency Code, Cardholder Billing (alphanumeric, fixed 3)
    51 => FieldDefinition::fixed(DataType::Alphanumeric, 3),

    // Field 52: PIN Data (binary, fixed 8)
    52 => FieldDefinition::fixed(DataType::Binary, 8),

    // Field 53: Security Related Control Information (numeric, fixed 16)
    53 => FieldDefinition::fixed(DataType::Numeric, 16),

    // Field 54: Additional Amounts (LLLVAR, max 120)
    54 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 120),

    // Field 55: ICC Data - EMV (LLLVAR, max 999)
    55 => FieldDefinition::lllvar(DataType::Binary, 999),

    // Fields 56-63: Reserved for ISO use
    56 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    57 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    58 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    59 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    60 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    61 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    62 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    63 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),

    // Field 64: Message Authentication Code (binary, fixed 8)
    64 => FieldDefinition::fixed(DataType::Binary, 8),

    // Field 65: Tertiary Bitmap (binary, fixed 8)
    65 => FieldDefinition::fixed(DataType::Binary, 8),

    // Fields 66-128: Additional fields
    66 => FieldDefinition::fixed(DataType::Numeric, 1),
    67 => FieldDefinition::fixed(DataType::Numeric, 2),
    68 => FieldDefinition::fixed(DataType::Numeric, 3),
    69 => FieldDefinition::fixed(DataType::Numeric, 3),
    70 => FieldDefinition::fixed(DataType::Numeric, 3),
    71 => FieldDefinition::fixed(DataType::Numeric, 4),
    72 => FieldDefinition::fixed(DataType::Numeric, 4),
    73 => FieldDefinition::fixed(DataType::Numeric, 6),
    74 => FieldDefinition::fixed(DataType::Numeric, 10),
    75 => FieldDefinition::fixed(DataType::Numeric, 10),
    76 => FieldDefinition::fixed(DataType::Numeric, 10),
    77 => FieldDefinition::fixed(DataType::Numeric, 10),
    78 => FieldDefinition::fixed(DataType::Numeric, 10),
    79 => FieldDefinition::fixed(DataType::Numeric, 10),
    80 => FieldDefinition::fixed(DataType::Numeric, 10),
    81 => FieldDefinition::fixed(DataType::Numeric, 10),
    82 => FieldDefinition::fixed(DataType::Numeric, 12),
    83 => FieldDefinition::fixed(DataType::Numeric, 12),
    84 => FieldDefinition::fixed(DataType::Numeric, 12),
    85 => FieldDefinition::fixed(DataType::Numeric, 12),
    86 => FieldDefinition::fixed(DataType::Numeric, 16),
    87 => FieldDefinition::fixed(DataType::Numeric, 16),
    88 => FieldDefinition::fixed(DataType::Numeric, 16),
    89 => FieldDefinition::fixed(DataType::Numeric, 16),
    90 => FieldDefinition::fixed(DataType::Numeric, 42),
    91 => FieldDefinition::fixed(DataType::Alphanumeric, 1),
    92 => FieldDefinition::fixed(DataType::Alphanumeric, 2),
    93 => FieldDefinition::fixed(DataType::Alphanumeric, 5),
    94 => FieldDefinition::fixed(DataType::Alphanumeric, 7),
    95 => FieldDefinition::fixed(DataType::Alphanumeric, 42),
    96 => FieldDefinition::fixed(DataType::Binary, 8),
    97 => FieldDefinition::fixed(DataType::Numeric, 16),
    98 => FieldDefinition::fixed(DataType::AlphanumericSpecial, 25),
    99 => FieldDefinition::llvar(DataType::Numeric, 11),
    100 => FieldDefinition::llvar(DataType::Numeric, 11),
    101 => FieldDefinition::llvar(DataType::AlphanumericSpecial, 17),
    102 => FieldDefinition::llvar(DataType::AlphanumericSpecial, 28),
    103 => FieldDefinition::llvar(DataType::AlphanumericSpecial, 28),
    104 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 100),
    105 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    106 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    107 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    108 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    109 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    110 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    111 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    112 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    113 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    114 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    115 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    116 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    117 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    118 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    119 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    120 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    121 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    122 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    123 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    124 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 255),
    125 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 50),
    126 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 6),
    127 => FieldDefinition::lllvar(DataType::AlphanumericSpecial, 999),
    128 => FieldDefinition::fixed(DataType::Binary, 8),
};

/// Trait for ISO 8583 specification versions
pub trait IsoSpec {
    /// Static field definition table
    const TABLE: &'static [Option<FieldDefinition>];

    /// Get field definition by number (O(1) lookup)
    #[inline]
    fn get_field(number: u8) -> Option<&'static FieldDefinition> {
        if (number as usize) < Self::TABLE.len() {
            Self::TABLE[number as usize].as_ref()
        } else {
            None
        }
    }
}

/// ISO 8583:1987 Specification
pub struct Iso1987;

impl IsoSpec for Iso1987 {
    const TABLE: &'static [Option<FieldDefinition>] = &ISO8583_1987_TABLE;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_lookup() {
        // Field 2 - PAN
        let field2 = Iso1987::get_field(2).unwrap();
        assert_eq!(field2.data_type, DataType::Numeric);
        assert_eq!(field2.length_type, LengthType::Llvar);
        assert_eq!(field2.max_len, 19);

        // Field 4 - Amount
        let field4 = Iso1987::get_field(4).unwrap();
        assert_eq!(field4.data_type, DataType::Numeric);
        assert_eq!(field4.length_type, LengthType::Fixed);
        assert_eq!(field4.max_len, 12);
    }

    #[test]
    fn test_invalid_field() {
        assert!(Iso1987::get_field(0).is_none());
        assert!(Iso1987::get_field(200).is_none());
    }

    #[test]
    fn test_zero_overhead() {
        // Verify that FieldDefinition is small
        assert_eq!(core::mem::size_of::<FieldDefinition>(), 4);

        // Verify enums are single byte
        assert_eq!(core::mem::size_of::<DataType>(), 1);
        assert_eq!(core::mem::size_of::<LengthType>(), 1);
    }
}
