//! Integration tests for ISO 8583 message parsing and generation

use iso8583_core::field::FieldValue;
use iso8583_core::*;

#[test]
fn test_complete_authorization_request_roundtrip() {
    // Build a complete authorization request message
    let original_message = ISO8583Message::builder()
        .mti(MessageType::AUTHORIZATION_REQUEST)
        .field(Field::PrimaryAccountNumber, "4111111111111111")
        .field(Field::ProcessingCode, "000000")
        .field(Field::TransactionAmount, "000000010000") // $100.00
        .field(Field::TransmissionDateTime, "0115120000")
        .field(Field::SystemTraceAuditNumber, "123456")
        .field(Field::LocalTransactionTime, "120000")
        .field(Field::LocalTransactionDate, "0115")
        .field(Field::ExpirationDate, "2512")
        .field(Field::MerchantType, "5999")
        .field(Field::PointOfServiceEntryMode, "051")
        .field(Field::AcquiringInstitutionCountryCode, "840")
        .field(Field::RetrievalReferenceNumber, "000000123456")
        .field(Field::CardAcceptorTerminalIdentification, "TERM0001")
        .field(Field::CardAcceptorIdentificationCode, "MERCHANT0000001")
        .field(Field::CurrencyCodeTransaction, "840")
        .build()
        .unwrap();

    // Generate bytes
    let bytes = original_message.to_bytes();
    println!("Generated message: {} bytes", bytes.len());
    println!("Hex: {}", hex::encode(&bytes));

    // Parse bytes back
    let parsed_message = ISO8583Message::from_bytes(&bytes).unwrap();

    // Verify MTI
    assert_eq!(parsed_message.mti, MessageType::AUTHORIZATION_REQUEST);
    assert_eq!(parsed_message.mti.to_string(), "0100");

    // Verify all fields
    assert_eq!(
        parsed_message
            .get_field(Field::PrimaryAccountNumber)
            .unwrap()
            .as_string(),
        Some("4111111111111111")
    );
    assert_eq!(
        parsed_message
            .get_field(Field::ProcessingCode)
            .unwrap()
            .as_string(),
        Some("000000")
    );
    assert_eq!(
        parsed_message
            .get_field(Field::TransactionAmount)
            .unwrap()
            .as_string(),
        Some("000000010000")
    );
    assert_eq!(
        parsed_message
            .get_field(Field::SystemTraceAuditNumber)
            .unwrap()
            .as_string(),
        Some("123456")
    );
}

#[test]
fn test_authorization_response() {
    let response = ISO8583Message::builder()
        .mti(MessageType::AUTHORIZATION_RESPONSE)
        .field(Field::PrimaryAccountNumber, "4111111111111111")
        .field(Field::ProcessingCode, "000000")
        .field(Field::TransactionAmount, "000000010000")
        .field(Field::TransmissionDateTime, "0115120000")
        .field(Field::SystemTraceAuditNumber, "123456")
        .field(Field::LocalTransactionTime, "120000")
        .field(Field::LocalTransactionDate, "0115")
        .field(Field::RetrievalReferenceNumber, "000000123456")
        .field(Field::AuthorizationIdentificationResponse, "ABC123")
        .field(Field::ResponseCode, "00") // Approved
        .build()
        .unwrap();

    assert!(response.mti.is_response());
    assert_eq!(
        response.get_field(Field::ResponseCode).unwrap().as_string(),
        Some("00")
    );
}

#[test]
fn test_financial_request() {
    let message = ISO8583Message::builder()
        .mti(MessageType::FINANCIAL_REQUEST)
        .field(Field::PrimaryAccountNumber, "5500000000000004")
        .field(Field::ProcessingCode, "010000") // Withdrawal
        .field(Field::TransactionAmount, "000000020000") // $200.00
        .field(Field::TransmissionDateTime, "0115150000")
        .field(Field::SystemTraceAuditNumber, "654321")
        .field(Field::LocalTransactionTime, "150000")
        .field(Field::LocalTransactionDate, "0115")
        .field(Field::CardAcceptorTerminalIdentification, "ATM00001")
        .field(Field::CurrencyCodeTransaction, "840")
        .build()
        .unwrap();

    assert_eq!(message.mti, MessageType::FINANCIAL_REQUEST);
    assert_eq!(message.mti.to_string(), "0200");

    // Test roundtrip
    let bytes = message.to_bytes();
    let parsed = ISO8583Message::from_bytes(&bytes).unwrap();
    assert_eq!(parsed.mti, message.mti);
}

#[test]
fn test_reversal_request() {
    let reversal = ISO8583Message::builder()
        .mti(MessageType::REVERSAL_REQUEST)
        .field(Field::PrimaryAccountNumber, "4111111111111111")
        .field(Field::ProcessingCode, "000000")
        .field(Field::TransactionAmount, "000000010000")
        .field(Field::TransmissionDateTime, "0115120100")
        .field(Field::SystemTraceAuditNumber, "123456")
        .field(Field::LocalTransactionTime, "120100")
        .field(Field::LocalTransactionDate, "0115")
        .field(Field::RetrievalReferenceNumber, "000000123456")
        .build()
        .unwrap();

    assert_eq!(reversal.mti, MessageType::REVERSAL_REQUEST);
    assert_eq!(reversal.mti.to_string(), "0400");
}

#[test]
fn test_network_management() {
    let message = ISO8583Message::builder()
        .mti(MessageType::NETWORK_MANAGEMENT_REQUEST)
        .field(Field::ProcessingCode, "000000")
        .field(Field::TransmissionDateTime, "0115000000")
        .field(Field::SystemTraceAuditNumber, "000001")
        .field(Field::LocalTransactionTime, "000000")
        .field(Field::LocalTransactionDate, "0115")
        .field(Field::NetworkManagementInformationCode, "001") // Sign-on
        .build()
        .unwrap();

    assert_eq!(message.mti.to_string(), "0800");
}

#[test]
fn test_variable_length_fields() {
    let message = ISO8583Message::builder()
        .mti(MessageType::AUTHORIZATION_REQUEST)
        .field(Field::PrimaryAccountNumber, "4111111111111111") // LLVAR
        .field(Field::ProcessingCode, "000000")
        .field(Field::TransactionAmount, "000000010000")
        .field(Field::SystemTraceAuditNumber, "123456")
        .field(Field::LocalTransactionTime, "120000")
        .field(Field::LocalTransactionDate, "0115")
        .field(Field::AcquiringInstitutionIdentificationCode, "12345") // LLVAR - variable
        .field(Field::Track2Data, "4111111111111111=2512101") // LLVAR
        .build()
        .unwrap();

    let bytes = message.to_bytes();
    let parsed = ISO8583Message::from_bytes(&bytes).unwrap();

    // Verify variable length fields were parsed correctly
    assert_eq!(
        parsed
            .get_field(Field::PrimaryAccountNumber)
            .unwrap()
            .as_string(),
        Some("4111111111111111")
    );
    assert_eq!(
        parsed
            .get_field(Field::AcquiringInstitutionIdentificationCode)
            .unwrap()
            .as_string(),
        Some("12345")
    );
}

#[test]
fn test_field_presence() {
    let message = ISO8583Message::builder()
        .mti(MessageType::AUTHORIZATION_REQUEST)
        .field(Field::PrimaryAccountNumber, "4111111111111111")
        .field(Field::ProcessingCode, "000000")
        .field(Field::TransactionAmount, "000000010000")
        .field(Field::SystemTraceAuditNumber, "123456")
        .field(Field::LocalTransactionTime, "120000")
        .field(Field::LocalTransactionDate, "0115")
        .build()
        .unwrap();

    assert!(message.has_field(Field::PrimaryAccountNumber));
    assert!(message.has_field(Field::ProcessingCode));
    assert!(message.has_field(Field::TransactionAmount));
    assert!(!message.has_field(Field::ResponseCode)); // Not in request
}

#[test]
fn test_bitmap_generation() {
    let message = ISO8583Message::builder()
        .mti(MessageType::AUTHORIZATION_REQUEST)
        .field(Field::PrimaryAccountNumber, "4111111111111111") // Field 2
        .field(Field::ProcessingCode, "000000") // Field 3
        .field(Field::TransactionAmount, "000000010000") // Field 4
        .field(Field::SystemTraceAuditNumber, "123456") // Field 11
        .field(Field::LocalTransactionTime, "120000") // Field 12
        .field(Field::LocalTransactionDate, "0115") // Field 13
        .build()
        .unwrap();

    let bitmap = message.bitmap();
    assert!(bitmap.is_set(2));
    assert!(bitmap.is_set(3));
    assert!(bitmap.is_set(4));
    assert!(bitmap.is_set(11));
    assert!(bitmap.is_set(12));
    assert!(bitmap.is_set(13));
    assert!(!bitmap.is_set(5)); // Not set
}

#[test]
fn test_mti_conversion() {
    let request = MessageType::AUTHORIZATION_REQUEST;
    let response = request.to_response().unwrap();

    assert_eq!(request.to_string(), "0100");
    assert_eq!(response.to_string(), "0110");

    assert!(request.is_request());
    assert!(response.is_response());
}

#[test]
fn test_pan_masking() {
    fn mask_pan(pan: &str) -> String {
        if pan.len() < 10 {
            return "*".repeat(pan.len());
        }
        let first = &pan[..6];
        let last = &pan[pan.len() - 4..];
        format!("{}****{}", first, last)
    }

    assert_eq!(mask_pan("4111111111111111"), "411111****1111");
    assert_eq!(mask_pan("5500000000000004"), "550000****0004");
}

#[test]
fn test_amount_formatting() {
    fn format_amount(amount_str: &str) -> String {
        let amount: i64 = amount_str.parse().unwrap_or(0);
        format!("${:.2}", amount as f64 / 100.0)
    }

    assert_eq!(format_amount("000000010000"), "$100.00");
    assert_eq!(format_amount("000000020050"), "$200.50");
    assert_eq!(format_amount("000000000001"), "$0.01");
}

#[test]
fn test_message_modification() {
    let mut message = ISO8583Message::builder()
        .mti(MessageType::AUTHORIZATION_REQUEST)
        .field(Field::PrimaryAccountNumber, "4111111111111111")
        .field(Field::ProcessingCode, "000000")
        .field(Field::TransactionAmount, "000000010000")
        .field(Field::SystemTraceAuditNumber, "123456")
        .field(Field::LocalTransactionTime, "120000")
        .field(Field::LocalTransactionDate, "0115")
        .build()
        .unwrap();

    // Add a new field
    message
        .set_field(Field::MerchantType, FieldValue::from_string("5999"))
        .unwrap();
    assert!(message.has_field(Field::MerchantType));

    // Remove a field
    message.remove_field(Field::MerchantType).unwrap();
    assert!(!message.has_field(Field::MerchantType));
}

#[test]
fn test_invalid_pan_length() {
    let result = validation::Validator::validate_pan("123"); // Too short
    assert!(!result);

    let result = validation::Validator::validate_pan("12345678901234567890"); // Too long
    assert!(!result);
}

#[test]
fn test_luhn_validation() {
    // Valid test cards
    assert!(validation::Validator::validate_pan("4111111111111111")); // Visa
    assert!(validation::Validator::validate_pan("5500000000000004")); // Mastercard
    assert!(validation::Validator::validate_pan("340000000000009")); // Amex

    // Invalid
    assert!(!validation::Validator::validate_pan("4111111111111112"));
    assert!(!validation::Validator::validate_pan("1234567890123456"));
}

#[test]
fn test_multiple_messages() {
    // Simulate processing multiple messages
    let messages = vec![
        ("0100", "4111111111111111", "000000010000"),
        ("0200", "5500000000000004", "000000020000"),
        ("0400", "4111111111111111", "000000010000"),
    ];

    for (mti_str, pan, amount) in messages {
        let mti: MessageType = mti_str.parse().unwrap();
        let message = ISO8583Message::builder()
            .mti(mti)
            .field(Field::PrimaryAccountNumber, pan)
            .field(Field::ProcessingCode, "000000")
            .field(Field::TransactionAmount, amount)
            .field(Field::SystemTraceAuditNumber, "123456")
            .field(Field::LocalTransactionTime, "120000")
            .field(Field::LocalTransactionDate, "0115")
            .build()
            .unwrap();

        let bytes = message.to_bytes();
        let parsed = ISO8583Message::from_bytes(&bytes).unwrap();

        assert_eq!(parsed.mti.to_string(), mti_str);
        assert_eq!(
            parsed
                .get_field(Field::PrimaryAccountNumber)
                .unwrap()
                .as_string(),
            Some(pan)
        );
    }
}

#[test]
fn test_error_handling() {
    // Test parsing invalid message (too short)
    let result = ISO8583Message::from_bytes(&[0u8; 4]);
    assert!(result.is_err());

    // Test parsing with invalid MTI
    let result = ISO8583Message::from_bytes(b"XXXX12345678");
    assert!(result.is_err());
}

#[test]
fn test_field_numbers() {
    let message = ISO8583Message::builder()
        .mti(MessageType::AUTHORIZATION_REQUEST)
        .field(Field::PrimaryAccountNumber, "4111111111111111") // 2
        .field(Field::ProcessingCode, "000000") // 3
        .field(Field::TransactionAmount, "000000010000") // 4
        .field(Field::SystemTraceAuditNumber, "123456") // 11
        .field(Field::LocalTransactionTime, "120000") // 12
        .field(Field::LocalTransactionDate, "0115") // 13
        .build()
        .unwrap();

    let field_numbers = message.get_field_numbers();
    assert_eq!(field_numbers, vec![2, 3, 4, 11, 12, 13]);
}
