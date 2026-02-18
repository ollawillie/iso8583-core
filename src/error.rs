//! Error types for ISO 8583 message processing

use thiserror::Error;

/// Result type for ISO 8583 operations
pub type Result<T> = std::result::Result<T, ISO8583Error>;

/// Errors that can occur during ISO 8583 message processing
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ISO8583Error {
    /// Invalid message type indicator
    #[error("Invalid MTI: {0}")]
    InvalidMTI(String),

    /// Invalid field number
    #[error("Invalid field number: {0}")]
    InvalidFieldNumber(u8),

    /// Field not present in message
    #[error("Field {0} not present in message")]
    FieldNotPresent(u8),

    /// Invalid field value
    #[error("Invalid value for field {field}: {reason}")]
    InvalidFieldValue { field: u8, reason: String },

    /// Field length mismatch
    #[error("Field {field} length mismatch: expected {expected}, got {actual}")]
    FieldLengthMismatch {
        field: u8,
        expected: usize,
        actual: usize,
    },

    /// Invalid bitmap
    #[error("Invalid bitmap: {0}")]
    InvalidBitmap(String),

    /// Invalid encoding
    #[error("Invalid encoding: {0}")]
    InvalidEncoding(String),

    /// Message too short
    #[error("Message too short: expected at least {expected} bytes, got {actual}")]
    MessageTooShort { expected: usize, actual: usize },

    /// Invalid PAN (Primary Account Number)
    #[error("Invalid PAN: {0}")]
    InvalidPAN(String),

    /// Luhn check failed
    #[error("Luhn check failed for PAN")]
    LuhnCheckFailed,

    /// Invalid amount
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),

    /// Invalid date/time
    #[error("Invalid date/time in field {field}: {reason}")]
    InvalidDateTime { field: u8, reason: String },

    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingRequiredField(u8),

    /// Parse error
    #[error("Parse error: {0}")]
    ParseError(String),

    /// Encoding error
    #[error("Encoding error: {0}")]
    EncodingError(String),

    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Builder error
    #[error("Builder error: {0}")]
    BuilderError(String),

    /// Invalid message class
    #[error("Invalid message class: {0}")]
    InvalidMessageClass(String),

    /// Invalid message function
    #[error("Invalid message function: {0}")]
    InvalidMessageFunction(String),

    /// Invalid message origin
    #[error("Invalid message origin: {0}")]
    InvalidMessageOrigin(String),

    /// Custom error
    #[error("Custom error: {0}")]
    Custom(String),
}

impl ISO8583Error {
    /// Create a custom error
    pub fn custom<S: Into<String>>(msg: S) -> Self {
        ISO8583Error::Custom(msg.into())
    }

    /// Create an invalid field value error
    pub fn invalid_field_value<S: Into<String>>(field: u8, reason: S) -> Self {
        ISO8583Error::InvalidFieldValue {
            field,
            reason: reason.into(),
        }
    }

    /// Create a field length mismatch error
    pub fn field_length_mismatch(field: u8, expected: usize, actual: usize) -> Self {
        ISO8583Error::FieldLengthMismatch {
            field,
            expected,
            actual,
        }
    }

    /// Create a message too short error
    pub fn message_too_short(expected: usize, actual: usize) -> Self {
        ISO8583Error::MessageTooShort { expected, actual }
    }

    /// Create an invalid date/time error
    pub fn invalid_datetime<S: Into<String>>(field: u8, reason: S) -> Self {
        ISO8583Error::InvalidDateTime {
            field,
            reason: reason.into(),
        }
    }
}

// Conversion from &'static str to ISO8583Error
impl From<&'static str> for ISO8583Error {
    fn from(s: &'static str) -> Self {
        ISO8583Error::Custom(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ISO8583Error::InvalidMTI("9999".to_string());
        assert_eq!(err.to_string(), "Invalid MTI: 9999");

        let err = ISO8583Error::FieldNotPresent(2);
        assert_eq!(err.to_string(), "Field 2 not present in message");

        let err = ISO8583Error::invalid_field_value(4, "Amount cannot be negative");
        assert_eq!(
            err.to_string(),
            "Invalid value for field 4: Amount cannot be negative"
        );
    }

    #[test]
    fn test_error_equality() {
        let err1 = ISO8583Error::InvalidMTI("0100".to_string());
        let err2 = ISO8583Error::InvalidMTI("0100".to_string());
        let err3 = ISO8583Error::InvalidMTI("0200".to_string());

        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }
}
