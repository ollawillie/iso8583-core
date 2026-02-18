//! Validation utilities for ISO 8583 messages and fields

use crate::error::{ISO8583Error, Result};
use crate::field::{Field, FieldValue};
use crate::message::ISO8583Message;

/// Validator for ISO 8583 messages and fields
pub struct Validator;

impl Validator {
    /// Validate Primary Account Number (PAN) using Luhn algorithm
    ///
    /// The Luhn algorithm (mod-10 algorithm) is used to validate credit card numbers.
    ///
    /// # Algorithm
    /// 1. Starting from the rightmost digit (check digit), double every second digit
    /// 2. If doubling results in a two-digit number, add the digits
    /// 3. Sum all digits
    /// 4. If sum is divisible by 10, the number is valid
    ///
    /// # Example
    /// ```
    /// use rust_iso8583::validation::Validator;
    ///
    /// assert!(Validator::validate_pan("4111111111111111"));  // Valid test card
    /// assert!(!Validator::validate_pan("4111111111111112")); // Invalid
    /// ```
    pub fn validate_pan(pan: &str) -> bool {
        // Remove any spaces or dashes
        let pan: String = pan.chars().filter(|c| c.is_ascii_digit()).collect();

        // PAN should be 13-19 digits
        if pan.len() < 13 || pan.len() > 19 {
            return false;
        }

        Self::luhn_check(&pan)
    }

    /// Luhn algorithm check
    fn luhn_check(number: &str) -> bool {
        let mut sum = 0;
        let mut double = false;

        // Process digits from right to left
        for ch in number.chars().rev() {
            if let Some(digit) = ch.to_digit(10) {
                let mut digit = digit;

                if double {
                    digit *= 2;
                    if digit > 9 {
                        digit -= 9; // Same as adding the digits
                    }
                }

                sum += digit;
                double = !double;
            } else {
                return false; // Non-digit character
            }
        }

        sum % 10 == 0
    }

    /// Validate field format based on field type
    pub fn validate_field_format(field: Field, value: &FieldValue) -> Result<()> {
        let def = field.definition();

        match value {
            FieldValue::String(s) => {
                // Check field type constraints
                match def.field_type {
                    crate::field::FieldType::Numeric => {
                        if !s.chars().all(|c| c.is_ascii_digit()) {
                            return Err(ISO8583Error::invalid_field_value(
                                field.number(),
                                "Field must be numeric",
                            ));
                        }
                    }
                    crate::field::FieldType::Alpha => {
                        if !s.chars().all(|c| c.is_ascii_alphabetic() || c == ' ') {
                            return Err(ISO8583Error::invalid_field_value(
                                field.number(),
                                "Field must be alphabetic",
                            ));
                        }
                    }
                    _ => {} // Other types allow more characters
                }

                // Check length
                match def.length {
                    crate::field::FieldLength::Fixed(len) => {
                        if s.len() != len {
                            return Err(ISO8583Error::field_length_mismatch(
                                field.number(),
                                len,
                                s.len(),
                            ));
                        }
                    }
                    crate::field::FieldLength::LLVar(max_len)
                    | crate::field::FieldLength::LLLVar(max_len) => {
                        if s.len() > max_len {
                            return Err(ISO8583Error::invalid_field_value(
                                field.number(),
                                format!("Field exceeds maximum length of {}", max_len),
                            ));
                        }
                    }
                }
            }
            FieldValue::Binary(_) => {
                // Binary fields have their own validation rules
            }
        }

        Ok(())
    }

    /// Validate specific field values
    pub fn validate_field_value(field: Field, value: &FieldValue) -> Result<()> {
        match field {
            Field::PrimaryAccountNumber => {
                if let Some(pan) = value.as_string() {
                    if !Self::validate_pan(pan) {
                        return Err(ISO8583Error::LuhnCheckFailed);
                    }
                }
            }
            Field::ResponseCode => {
                if let Some(code) = value.as_string() {
                    if code.len() != 2 {
                        return Err(ISO8583Error::invalid_field_value(
                            39,
                            "Response code must be 2 characters",
                        ));
                    }
                }
            }
            Field::TransactionAmount | Field::SettlementAmount => {
                if let Some(amount) = value.as_string() {
                    if !amount.chars().all(|c| c.is_ascii_digit()) {
                        return Err(ISO8583Error::invalid_field_value(
                            field.number(),
                            "Amount must be numeric",
                        ));
                    }
                    // Amount must not be zero (in most cases)
                    if amount.chars().all(|c| c == '0') {
                        return Err(ISO8583Error::invalid_field_value(
                            field.number(),
                            "Amount cannot be zero",
                        ));
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Validate required fields for a message type
    pub fn validate_required_fields(msg: &ISO8583Message) -> Result<()> {
        // Common required fields for most transactions
        let common_required = vec![
            Field::ProcessingCode,
            Field::SystemTraceAuditNumber,
            Field::LocalTransactionTime,
            Field::LocalTransactionDate,
        ];

        for field in common_required {
            if msg.get_field(field).is_none() {
                return Err(ISO8583Error::MissingRequiredField(field.number()));
            }
        }

        // Message type specific requirements
        if msg.mti.is_request() {
            // Requests typically need PAN and amount
            if msg.mti.class == crate::mti::MessageClass::Financial
                || msg.mti.class == crate::mti::MessageClass::Authorization
            {
                if msg.get_field(Field::PrimaryAccountNumber).is_none() {
                    return Err(ISO8583Error::MissingRequiredField(2));
                }
                if msg.get_field(Field::TransactionAmount).is_none() {
                    return Err(ISO8583Error::MissingRequiredField(4));
                }
            }
        }

        if msg.mti.is_response() {
            // Responses need a response code
            if msg.get_field(Field::ResponseCode).is_none() {
                return Err(ISO8583Error::MissingRequiredField(39));
            }
        }

        Ok(())
    }

    /// Validate date format (MMDD)
    pub fn validate_date_mmdd(date: &str) -> bool {
        if date.len() != 4 {
            return false;
        }

        if let Ok(month) = date[0..2].parse::<u32>() {
            if let Ok(day) = date[2..4].parse::<u32>() {
                return month >= 1 && month <= 12 && day >= 1 && day <= 31;
            }
        }

        false
    }

    /// Validate time format (hhmmss)
    pub fn validate_time_hhmmss(time: &str) -> bool {
        if time.len() != 6 {
            return false;
        }

        if let Ok(hour) = time[0..2].parse::<u32>() {
            if let Ok(minute) = time[2..4].parse::<u32>() {
                if let Ok(second) = time[4..6].parse::<u32>() {
                    return hour < 24 && minute < 60 && second < 60;
                }
            }
        }

        false
    }

    /// Validate currency code (ISO 4217)
    pub fn validate_currency_code(code: &str) -> bool {
        code.len() == 3 && code.chars().all(|c| c.is_ascii_digit())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_luhn_valid() {
        // Valid test card numbers
        assert!(Validator::validate_pan("4111111111111111")); // Visa
        assert!(Validator::validate_pan("5500000000000004")); // Mastercard
        assert!(Validator::validate_pan("340000000000009")); // Amex
    }

    #[test]
    fn test_luhn_invalid() {
        assert!(!Validator::validate_pan("4111111111111112")); // Wrong check digit
        assert!(!Validator::validate_pan("1234567890123456")); // Invalid
        assert!(!Validator::validate_pan("0000000000000000")); // All zeros
    }

    #[test]
    fn test_luhn_with_spaces() {
        assert!(Validator::validate_pan("4111 1111 1111 1111")); // With spaces
    }

    #[test]
    fn test_pan_length() {
        assert!(!Validator::validate_pan("123")); // Too short
        assert!(!Validator::validate_pan("12345678901234567890")); // Too long
    }

    #[test]
    fn test_validate_date_mmdd() {
        assert!(Validator::validate_date_mmdd("0101")); // Jan 1
        assert!(Validator::validate_date_mmdd("1231")); // Dec 31
        assert!(!Validator::validate_date_mmdd("1301")); // Invalid month
        assert!(!Validator::validate_date_mmdd("0132")); // Invalid day
        assert!(!Validator::validate_date_mmdd("123")); // Wrong length
    }

    #[test]
    fn test_validate_time_hhmmss() {
        assert!(Validator::validate_time_hhmmss("000000")); // Midnight
        assert!(Validator::validate_time_hhmmss("235959")); // 23:59:59
        assert!(Validator::validate_time_hhmmss("120000")); // Noon
        assert!(!Validator::validate_time_hhmmss("240000")); // Invalid hour
        assert!(!Validator::validate_time_hhmmss("126000")); // Invalid minute
        assert!(!Validator::validate_time_hhmmss("120060")); // Invalid second
    }

    #[test]
    fn test_validate_currency_code() {
        assert!(Validator::validate_currency_code("840")); // USD
        assert!(Validator::validate_currency_code("566")); // NGN
        assert!(Validator::validate_currency_code("978")); // EUR
        assert!(!Validator::validate_currency_code("USD")); // Not numeric
        assert!(!Validator::validate_currency_code("84")); // Too short
    }
}
