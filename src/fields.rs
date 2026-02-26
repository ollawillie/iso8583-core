//! Type-Safe Field Generation System
//!
//! Provides compile-time field type safety using Rust's macro system.

/// Trait for ISO 8583 fields with compile-time field numbers
pub trait IsoField {
    /// Field number (compile-time constant)
    const NUMBER: u8;
}

/// Macro to define a type-safe ISO 8583 field
///
/// # Example
/// ```
/// use iso8583_core::define_field;
///
/// define_field!(
///     /// Primary Account Number
///     Field2Pan,
///     2
/// );
/// ```
#[macro_export]
macro_rules! define_field {
    (
        $(#[$meta:meta])*
        $name:ident,
        $num:expr
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name<T>(pub T);

        impl<T> $crate::fields::IsoField for $name<T> {
            const NUMBER: u8 = $num;
        }

        impl<T> $name<T> {
            /// Get the field number
            #[inline]
            pub const fn number() -> u8 {
                $num
            }

            /// Create a new field value
            #[inline]
            pub fn new(value: T) -> Self {
                Self(value)
            }

            /// Get the inner value
            #[inline]
            pub fn value(&self) -> &T {
                &self.0
            }

            /// Consume and get the inner value
            #[inline]
            pub fn into_value(self) -> T {
                self.0
            }
        }
    };
}

/// Macro to define a fixed-length numeric field with compile-time size checking
///
/// # Example
/// ```
/// use iso8583_core::define_numeric_field;
///
/// define_numeric_field!(
///     /// Transaction Amount (12 digits)
///     Field4Amount,
///     4,
///     12
/// );
/// ```
#[macro_export]
macro_rules! define_numeric_field {
    (
        $(#[$meta:meta])*
        $name:ident,
        $num:expr,
        $len:expr
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name(pub [u8; $len]);

        impl $crate::fields::IsoField for $name {
            const NUMBER: u8 = $num;
        }

        impl std::str::FromStr for $name {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if s.len() != $len {
                    return Err("Invalid length");
                }
                if !s.bytes().all(|b| b.is_ascii_digit()) {
                    return Err("Must contain only digits");
                }
                let mut data = [0u8; $len];
                data.copy_from_slice(s.as_bytes());
                Ok(Self(data))
            }
        }

        impl $name {
            /// Create from byte array
            #[inline]
            pub const fn new(data: [u8; $len]) -> Self {
                Self(data)
            }

            /// Get as byte slice
            #[inline]
            pub fn as_bytes(&self) -> &[u8; $len] {
                &self.0
            }

            /// Convert to u64 (if fits)
            #[inline]
            pub fn to_u64(&self) -> Result<u64, &'static str> {
                core::str::from_utf8(&self.0)
                    .map_err(|_| "Invalid UTF-8")?
                    .parse::<u64>()
                    .map_err(|_| "Parse error")
            }
        }
    };
}

// Generate all standard ISO 8583 fields

define_field!(
    /// Primary Account Number (Field 2)
    Field2Pan,
    2
);

define_field!(
    /// Processing Code (Field 3)
    Field3ProcessingCode,
    3
);

define_numeric_field!(
    /// Transaction Amount (Field 4) - 12 digits
    Field4Amount,
    4,
    12
);

define_numeric_field!(
    /// Transmission Date & Time (Field 7) - MMDDhhmmss
    Field7TransmissionDateTime,
    7,
    10
);

define_numeric_field!(
    /// System Trace Audit Number (Field 11) - 6 digits
    Field11Stan,
    11,
    6
);

define_numeric_field!(
    /// Local Transaction Time (Field 12) - hhmmss
    Field12LocalTime,
    12,
    6
);

define_numeric_field!(
    /// Local Transaction Date (Field 13) - MMDD
    Field13LocalDate,
    13,
    4
);

define_numeric_field!(
    /// Expiration Date (Field 14) - YYMM
    Field14ExpirationDate,
    14,
    4
);

define_field!(
    /// Point of Service Entry Mode (Field 22)
    Field22PosEntryMode,
    22
);

define_field!(
    /// Acquiring Institution Country Code (Field 19)
    Field19AcquiringCountry,
    19
);

define_field!(
    /// Track 2 Data (Field 35)
    Field35Track2,
    35
);

define_field!(
    /// Retrieval Reference Number (Field 37)
    Field37Rrn,
    37
);

define_field!(
    /// Authorization ID Response (Field 38)
    Field38AuthId,
    38
);

define_field!(
    /// Response Code (Field 39)
    Field39ResponseCode,
    39
);

define_field!(
    /// Card Acceptor Terminal ID (Field 41)
    Field41TerminalId,
    41
);

define_field!(
    /// Card Acceptor ID Code (Field 42)
    Field42MerchantId,
    42
);

define_field!(
    /// Card Acceptor Name/Location (Field 43)
    Field43CardAcceptorName,
    43
);

define_field!(
    /// Currency Code, Transaction (Field 49)
    Field49CurrencyCode,
    49
);

define_field!(
    /// PIN Data (Field 52)
    Field52PinData,
    52
);

define_field!(
    /// Additional Amounts (Field 54)
    Field54AdditionalAmounts,
    54
);

define_field!(
    /// Message Authentication Code (Field 64)
    Field64Mac,
    64
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_number() {
        assert_eq!(Field2Pan::<&str>::NUMBER, 2);
        assert_eq!(Field4Amount::NUMBER, 4);
        assert_eq!(Field11Stan::NUMBER, 11);
    }

    #[test]
    fn test_numeric_field_creation() {
        let amount = "000000010000".parse::<Field4Amount>().unwrap();
        assert_eq!(amount.to_u64().unwrap(), 10000);
    }

    #[test]
    fn test_numeric_field_validation() {
        // Wrong length
        assert!("123".parse::<Field4Amount>().is_err());

        // Non-numeric
        assert!("00000001000A".parse::<Field4Amount>().is_err());
    }

    #[test]
    fn test_field_wrapper() {
        let pan = Field2Pan::new("4111111111111111");
        assert_eq!(pan.value(), &"4111111111111111");
        assert_eq!(pan.into_value(), "4111111111111111");
    }
}
