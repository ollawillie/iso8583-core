//! ISO 8583 Message structure and operations
//!
//! This module provides the main message type and operations for
//! parsing and generating ISO 8583 messages.

use crate::bitmap::Bitmap;
use crate::error::{ISO8583Error, Result};
use crate::field::{Field, FieldDefinition, FieldLength, FieldType, FieldValue};
use crate::mti::MessageType;
use std::collections::HashMap;

/// ISO 8583 Message
#[derive(Debug, Clone, PartialEq)]
pub struct ISO8583Message {
    /// Message Type Indicator
    pub mti: MessageType,
    /// Field values (keyed by field number)
    fields: HashMap<u8, FieldValue>,
    /// Bitmap indicating present fields
    bitmap: Bitmap,
}

impl ISO8583Message {
    /// Create a new message with given MTI
    pub fn new(mti: MessageType) -> Self {
        Self {
            mti,
            fields: HashMap::new(),
            bitmap: Bitmap::new(),
        }
    }

    /// Parse message from bytes (ASCII encoding)
    ///
    /// # Format
    /// ```text
    /// [MTI (4 bytes)][Bitmap (8/16/24 bytes)][Fields...]
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 12 {
            // Minimum: 4 (MTI) + 8 (bitmap)
            return Err(ISO8583Error::message_too_short(12, bytes.len()));
        }

        let mut offset = 0;

        // 1. Parse MTI (first 4 bytes)
        let mti = MessageType::from_bytes(&bytes[offset..offset + 4])?;
        offset += 4;

        // 2. Parse primary bitmap (8 bytes = 16 hex chars)
        let bitmap_hex = hex::encode(&bytes[offset..offset + 8]);
        let mut bitmap = Bitmap::from_hex(&bitmap_hex)?;
        offset += 8;

        // 3. Check for secondary bitmap (if field 1 is set)
        if bitmap.is_set(1) {
            if bytes.len() < offset + 8 {
                return Err(ISO8583Error::message_too_short(offset + 8, bytes.len()));
            }
            let secondary_hex = hex::encode(&bytes[offset..offset + 8]);
            let secondary_bitmap = Bitmap::from_hex(&secondary_hex)?;

            // Merge secondary bitmap into main bitmap
            for field_num in 65..=128 {
                if secondary_bitmap.is_set(field_num) {
                    bitmap.set(field_num)?;
                }
            }
            offset += 8;
        }

        // 4. Parse fields based on bitmap
        let mut fields = HashMap::new();
        let (field_array, field_count) = bitmap.get_set_fields();

        for item in field_array.iter().take(field_count) {
            let field_num = *item;
            if field_num == 1 || field_num == 65 {
                continue; // Skip bitmap indicators
            }

            let field = Field::from_number(field_num)?;
            let def = field.definition();

            // Parse field based on its length specification
            let (value, bytes_consumed) = Self::parse_field(&bytes[offset..], &def)?;
            fields.insert(field_num, value);
            offset += bytes_consumed;
        }

        Ok(Self {
            mti,
            fields,
            bitmap,
        })
    }

    /// Parse a single field from bytes
    fn parse_field(bytes: &[u8], def: &FieldDefinition) -> Result<(FieldValue, usize)> {
        // Ensure we have at least some bytes to parse
        if bytes.is_empty() {
            return Err(ISO8583Error::message_too_short(1, 0));
        }

        match def.length {
            FieldLength::Fixed(len) => {
                // Bounds check for fixed length
                if bytes.len() < len {
                    return Err(ISO8583Error::field_length_mismatch(
                        def.number,
                        len,
                        bytes.len(),
                    ));
                }

                let value = match def.field_type {
                    FieldType::Binary => FieldValue::from_binary(bytes[..len].to_vec()),
                    _ => {
                        let s = std::str::from_utf8(&bytes[..len]).map_err(|e| {
                            ISO8583Error::EncodingError(format!(
                                "Invalid UTF-8 in field {}: {}",
                                def.number, e
                            ))
                        })?;
                        FieldValue::from_string(s.to_string())
                    }
                };

                Ok((value, len))
            }
            FieldLength::LLVar(max_len) => {
                // 2-digit length indicator - bounds check
                if bytes.len() < 2 {
                    return Err(ISO8583Error::message_too_short(2, bytes.len()));
                }

                let length_str = std::str::from_utf8(&bytes[..2]).map_err(|e| {
                    ISO8583Error::EncodingError(format!(
                        "Invalid length indicator for field {}: {}",
                        def.number, e
                    ))
                })?;
                let length: usize = length_str.parse().map_err(|e| {
                    ISO8583Error::EncodingError(format!(
                        "Invalid length value for field {}: {}",
                        def.number, e
                    ))
                })?;

                if length > max_len {
                    return Err(ISO8583Error::invalid_field_value(
                        def.number,
                        format!(
                            "Length {} exceeds maximum {} for field {}",
                            length, max_len, def.number
                        ),
                    ));
                }

                // Bounds check for field data
                if bytes.len() < 2 + length {
                    return Err(ISO8583Error::message_too_short(2 + length, bytes.len()));
                }

                let value = match def.field_type {
                    FieldType::Binary => FieldValue::from_binary(bytes[2..2 + length].to_vec()),
                    _ => {
                        let s = std::str::from_utf8(&bytes[2..2 + length]).map_err(|e| {
                            ISO8583Error::EncodingError(format!(
                                "Invalid UTF-8 in field {}: {}",
                                def.number, e
                            ))
                        })?;
                        FieldValue::from_string(s.to_string())
                    }
                };

                Ok((value, 2 + length))
            }
            FieldLength::LLLVar(max_len) => {
                // 3-digit length indicator - bounds check
                if bytes.len() < 3 {
                    return Err(ISO8583Error::message_too_short(3, bytes.len()));
                }

                let length_str = std::str::from_utf8(&bytes[..3]).map_err(|e| {
                    ISO8583Error::EncodingError(format!(
                        "Invalid length indicator for field {}: {}",
                        def.number, e
                    ))
                })?;
                let length: usize = length_str.parse().map_err(|e| {
                    ISO8583Error::EncodingError(format!(
                        "Invalid length value for field {}: {}",
                        def.number, e
                    ))
                })?;

                if length > max_len {
                    return Err(ISO8583Error::invalid_field_value(
                        def.number,
                        format!(
                            "Length {} exceeds maximum {} for field {}",
                            length, max_len, def.number
                        ),
                    ));
                }

                // Bounds check for field data
                if bytes.len() < 3 + length {
                    return Err(ISO8583Error::message_too_short(3 + length, bytes.len()));
                }

                let value = match def.field_type {
                    FieldType::Binary => FieldValue::from_binary(bytes[3..3 + length].to_vec()),
                    _ => {
                        let s = std::str::from_utf8(&bytes[3..3 + length]).map_err(|e| {
                            ISO8583Error::EncodingError(format!(
                                "Invalid UTF-8 in field {}: {}",
                                def.number, e
                            ))
                        })?;
                        FieldValue::from_string(s.to_string())
                    }
                };

                Ok((value, 3 + length))
            }
        }
    }

    /// Generate message bytes (ASCII encoding)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // 1. Add MTI
        bytes.extend_from_slice(&self.mti.to_bytes());

        // 2. Add bitmap(s)
        let (bitmap_bytes, bitmap_len) = self.bitmap.to_bytes();
        bytes.extend_from_slice(&bitmap_bytes[..bitmap_len]);

        // 3. Add fields in numerical order
        let mut field_numbers: Vec<u8> = self.fields.keys().copied().collect();
        field_numbers.sort();

        for field_num in field_numbers {
            if field_num == 1 || field_num == 65 {
                continue; // Skip bitmap indicators
            }

            if let Some(value) = self.fields.get(&field_num) {
                let field = Field::from_number(field_num).unwrap();
                let field_bytes = Self::generate_field(&field, value);
                bytes.extend_from_slice(&field_bytes);
            }
        }

        bytes
    }

    /// Generate bytes for a single field
    fn generate_field(field: &Field, value: &FieldValue) -> Vec<u8> {
        let def = field.definition();
        let mut bytes = Vec::new();

        match def.length {
            FieldLength::Fixed(len) => {
                // Fixed length field
                match value {
                    FieldValue::String(s) => {
                        let mut field_str = s.clone();
                        // Pad or truncate to exact length
                        if field_str.len() < len {
                            // Pad with spaces or zeros depending on field type
                            match def.field_type {
                                FieldType::Numeric => {
                                    field_str = format!("{:0>width$}", field_str, width = len);
                                }
                                _ => {
                                    field_str = format!("{:<width$}", field_str, width = len);
                                }
                            }
                        } else if field_str.len() > len {
                            field_str.truncate(len);
                        }
                        bytes.extend_from_slice(field_str.as_bytes());
                    }
                    FieldValue::Binary(b) => {
                        let mut bin = b.clone();
                        bin.resize(len, 0); // Pad with zeros if needed
                        bytes.extend_from_slice(&bin);
                    }
                }
            }
            FieldLength::LLVar(_max_len) => {
                // Variable length with 2-digit length indicator
                match value {
                    FieldValue::String(s) => {
                        let length = format!("{:02}", s.len());
                        bytes.extend_from_slice(length.as_bytes());
                        bytes.extend_from_slice(s.as_bytes());
                    }
                    FieldValue::Binary(b) => {
                        let length = format!("{:02}", b.len());
                        bytes.extend_from_slice(length.as_bytes());
                        bytes.extend_from_slice(b);
                    }
                }
            }
            FieldLength::LLLVar(_max_len) => {
                // Variable length with 3-digit length indicator
                match value {
                    FieldValue::String(s) => {
                        let length = format!("{:03}", s.len());
                        bytes.extend_from_slice(length.as_bytes());
                        bytes.extend_from_slice(s.as_bytes());
                    }
                    FieldValue::Binary(b) => {
                        let length = format!("{:03}", b.len());
                        bytes.extend_from_slice(length.as_bytes());
                        bytes.extend_from_slice(b);
                    }
                }
            }
        }

        bytes
    }

    /// Get field value
    pub fn get_field(&self, field: Field) -> Option<&FieldValue> {
        self.fields.get(&field.number())
    }

    /// Set field value
    pub fn set_field(&mut self, field: Field, value: FieldValue) -> Result<()> {
        let field_num = field.number();

        // Update bitmap
        self.bitmap.set(field_num)?;

        // Store value
        self.fields.insert(field_num, value);

        Ok(())
    }

    /// Remove field
    pub fn remove_field(&mut self, field: Field) -> Result<()> {
        let field_num = field.number();

        // Update bitmap
        self.bitmap.clear(field_num)?;

        // Remove value
        self.fields.remove(&field_num);

        Ok(())
    }

    /// Check if field is present
    pub fn has_field(&self, field: Field) -> bool {
        self.fields.contains_key(&field.number())
    }

    /// Get all present field numbers
    pub fn get_field_numbers(&self) -> Vec<u8> {
        let mut numbers: Vec<u8> = self.fields.keys().copied().collect();
        numbers.sort();
        numbers
    }

    /// Get bitmap reference
    pub fn bitmap(&self) -> &Bitmap {
        &self.bitmap
    }

    /// Create a builder for constructing messages
    pub fn builder() -> MessageBuilder {
        MessageBuilder::new()
    }
}

/// Builder for ISO 8583 messages
#[derive(Debug)]
pub struct MessageBuilder {
    message: ISO8583Message,
}

impl MessageBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            message: ISO8583Message::new(MessageType::AUTHORIZATION_REQUEST),
        }
    }

    /// Set the MTI
    pub fn mti(mut self, mti: MessageType) -> Self {
        self.message.mti = mti;
        self
    }

    /// Add a field
    pub fn field<S: Into<String>>(mut self, field: Field, value: S) -> Self {
        let _ = self
            .message
            .set_field(field, FieldValue::from_string(value.into()));
        self
    }

    /// Add a binary field
    pub fn binary_field(mut self, field: Field, value: Vec<u8>) -> Self {
        let _ = self
            .message
            .set_field(field, FieldValue::from_binary(value));
        self
    }

    /// Build the message
    pub fn build(self) -> Result<ISO8583Message> {
        // Validate the message
        crate::validation::Validator::validate_required_fields(&self.message)?;

        Ok(self.message)
    }
}

impl Default for MessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let msg = ISO8583Message::new(MessageType::AUTHORIZATION_REQUEST);
        assert_eq!(msg.mti, MessageType::AUTHORIZATION_REQUEST);
        assert_eq!(msg.get_field_numbers().len(), 0);
    }

    #[test]
    fn test_set_and_get_field() {
        let mut msg = ISO8583Message::new(MessageType::AUTHORIZATION_REQUEST);

        msg.set_field(
            Field::PrimaryAccountNumber,
            FieldValue::from_string("4111111111111111"),
        )
        .unwrap();

        assert!(msg.has_field(Field::PrimaryAccountNumber));
        let value = msg.get_field(Field::PrimaryAccountNumber).unwrap();
        assert_eq!(value.as_string(), Some("4111111111111111"));
    }

    #[test]
    fn test_remove_field() {
        let mut msg = ISO8583Message::new(MessageType::AUTHORIZATION_REQUEST);

        msg.set_field(
            Field::PrimaryAccountNumber,
            FieldValue::from_string("4111111111111111"),
        )
        .unwrap();

        assert!(msg.has_field(Field::PrimaryAccountNumber));

        msg.remove_field(Field::PrimaryAccountNumber).unwrap();

        assert!(!msg.has_field(Field::PrimaryAccountNumber));
    }

    #[test]
    fn test_builder() {
        let msg = ISO8583Message::builder()
            .mti(MessageType::FINANCIAL_REQUEST)
            .field(Field::ProcessingCode, "000000")
            .field(Field::SystemTraceAuditNumber, "123456")
            .field(Field::LocalTransactionTime, "120000")
            .field(Field::LocalTransactionDate, "0101");

        // Note: build() will fail because required fields are missing
        // This is expected behavior
        assert!(msg.build().is_err());
    }
}
