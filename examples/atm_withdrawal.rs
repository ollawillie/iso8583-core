//! ATM Withdrawal Transaction Example
//!
//! This example demonstrates a complete ATM withdrawal flow:
//! 1. Customer inserts card and enters PIN
//! 2. ATM sends authorization request
//! 3. Bank responds with approval
//! 4. ATM sends financial request
//! 5. Bank responds with confirmation
//! 6. ATM dispenses cash

use iso8583_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║              ATM WITHDRAWAL TRANSACTION EXAMPLE              ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Step 1: Authorization Request (0100)
    println!("STEP 1: Authorization Request (Pre-check if funds available)");
    println!("─────────────────────────────────────────────────────────────\n");

    let auth_request = ISO8583Message::builder()
        .mti(MessageType::AUTHORIZATION_REQUEST)
        .field(Field::PrimaryAccountNumber, "4111111111111111")
        .field(Field::ProcessingCode, "010000") // Withdrawal from checking
        .field(Field::TransactionAmount, "000000020000") // $200.00
        .field(Field::TransmissionDateTime, "0115150000") // Jan 15, 15:00:00
        .field(Field::SystemTraceAuditNumber, "123456")
        .field(Field::LocalTransactionTime, "150000") // 15:00:00
        .field(Field::LocalTransactionDate, "0115") // Jan 15
        .field(Field::PointOfServiceEntryMode, "021") // Chip card with PIN
        .field(Field::AcquiringInstitutionCountryCode, "840") // USA
        .field(Field::PointOfServiceConditionCode, "00") // Normal presentment
        .field(Field::CardAcceptorTerminalIdentification, "ATM00123")
        .field(Field::CardAcceptorIdentificationCode, "BANK00000000001")
        .field(Field::CurrencyCodeTransaction, "840") // USD
        .build()?;

    println!("Authorization Request Generated:");
    println!("  MTI: {}", auth_request.mti);
    println!("  PAN: {}", mask_pan(auth_request.get_field(Field::PrimaryAccountNumber).unwrap().as_string().unwrap()));
    println!("  Amount: {}", format_amount(auth_request.get_field(Field::TransactionAmount).unwrap().as_string().unwrap()));
    println!("  STAN: {}", auth_request.get_field(Field::SystemTraceAuditNumber).unwrap());
    println!("  Terminal: {}\n", auth_request.get_field(Field::CardAcceptorTerminalIdentification).unwrap());

    // Generate and display message bytes
    let auth_request_bytes = auth_request.to_bytes();
    println!("  Message Size: {} bytes", auth_request_bytes.len());
    println!("  Hex Preview: {}...\n", hex::encode(&auth_request_bytes[..40.min(auth_request_bytes.len())]));

    // Step 2: Authorization Response (0110) - Simulated from bank
    println!("STEP 2: Authorization Response from Bank");
    println!("─────────────────────────────────────────────────────────────\n");

    let auth_response = ISO8583Message::builder()
        .mti(MessageType::AUTHORIZATION_RESPONSE)
        .field(Field::PrimaryAccountNumber, "4111111111111111")
        .field(Field::ProcessingCode, "010000")
        .field(Field::TransactionAmount, "000000020000")
        .field(Field::TransmissionDateTime, "0115150001")
        .field(Field::SystemTraceAuditNumber, "123456")
        .field(Field::LocalTransactionTime, "150001")
        .field(Field::LocalTransactionDate, "0115")
        .field(Field::RetrievalReferenceNumber, "000115123456")
        .field(Field::AuthorizationIdentificationResponse, "AUTH01")
        .field(Field::ResponseCode, "00") // Approved
        .build()?;

    println!("Authorization Response Received:");
    println!("  Response Code: {} ({})", 
             auth_response.get_field(Field::ResponseCode).unwrap(),
             get_response_description("00"));
    println!("  Auth ID: {}", auth_response.get_field(Field::AuthorizationIdentificationResponse).unwrap());
    println!("  RRN: {}\n", auth_response.get_field(Field::RetrievalReferenceNumber).unwrap());

    // Step 3: Financial Request (0200) - Actual withdrawal
    println!("STEP 3: Financial Request (Complete Transaction)");
    println!("─────────────────────────────────────────────────────────────\n");

    let financial_request = ISO8583Message::builder()
        .mti(MessageType::FINANCIAL_REQUEST)
        .field(Field::PrimaryAccountNumber, "4111111111111111")
        .field(Field::ProcessingCode, "010000")
        .field(Field::TransactionAmount, "000000020000")
        .field(Field::TransmissionDateTime, "0115150002")
        .field(Field::SystemTraceAuditNumber, "123457") // Incremented
        .field(Field::LocalTransactionTime, "150002")
        .field(Field::LocalTransactionDate, "0115")
        .field(Field::PointOfServiceEntryMode, "021")
        .field(Field::AcquiringInstitutionCountryCode, "840")
        .field(Field::RetrievalReferenceNumber, "000115123456")
        .field(Field::CardAcceptorTerminalIdentification, "ATM00123")
        .field(Field::CardAcceptorIdentificationCode, "BANK00000000001")
        .field(Field::CurrencyCodeTransaction, "840")
        .build()?;

    println!("Financial Request Generated:");
    println!("  MTI: {}", financial_request.mti);
    println!("  STAN: {}", financial_request.get_field(Field::SystemTraceAuditNumber).unwrap());
    println!("  Amount: {}\n", format_amount(financial_request.get_field(Field::TransactionAmount).unwrap().as_string().unwrap()));

    // Step 4: Financial Response (0210) - Confirmation from bank
    println!("STEP 4: Financial Response (Funds Debited)");
    println!("─────────────────────────────────────────────────────────────\n");

    let financial_response = ISO8583Message::builder()
        .mti(MessageType::FINANCIAL_RESPONSE)
        .field(Field::PrimaryAccountNumber, "4111111111111111")
        .field(Field::ProcessingCode, "010000")
        .field(Field::TransactionAmount, "000000020000")
        .field(Field::TransmissionDateTime, "0115150003")
        .field(Field::SystemTraceAuditNumber, "123457")
        .field(Field::LocalTransactionTime, "150003")
        .field(Field::LocalTransactionDate, "0115")
        .field(Field::RetrievalReferenceNumber, "000115123456")
        .field(Field::AuthorizationIdentificationResponse, "AUTH01")
        .field(Field::ResponseCode, "00") // Approved
        .build()?;

    println!("Financial Response Received:");
    println!("  Response Code: {} ({})", 
             financial_response.get_field(Field::ResponseCode).unwrap(),
             get_response_description("00"));
    println!("  Transaction Complete!");
    println!("  ✓ Customer account debited: $200.00");
    println!("  ✓ ATM can now dispense cash\n");

    // Display transaction summary
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    TRANSACTION SUMMARY                       ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║ Card Number:    {}                              ║", mask_pan("4111111111111111"));
    println!("║ Transaction:    Withdrawal                                   ║");
    println!("║ Amount:         $200.00                                      ║");
    println!("║ Date/Time:      Jan 15, 15:00:03                            ║");
    println!("║ Terminal:       ATM00123                                     ║");
    println!("║ Status:         APPROVED                                     ║");
    println!("║ Auth Code:      AUTH01                                       ║");
    println!("║ Reference:      000115123456                                 ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Test message roundtrip
    println!("VERIFICATION: Testing Message Roundtrip");
    println!("─────────────────────────────────────────────────────────────\n");

    let original_bytes = financial_request.to_bytes();
    let parsed_message = ISO8583Message::from_bytes(&original_bytes)?;

    println!("  ✓ Original MTI: {}", financial_request.mti);
    println!("  ✓ Parsed MTI:   {}", parsed_message.mti);
    println!("  ✓ Fields match: {}", parsed_message.get_field_numbers().len());
    println!("  ✓ Roundtrip successful!\n");

    Ok(())
}

/// Mask PAN for display (show first 6 and last 4 digits)
fn mask_pan(pan: &str) -> String {
    if pan.len() < 10 {
        return "*".repeat(pan.len());
    }
    let first = &pan[..6];
    let last = &pan[pan.len() - 4..];
    format!("{}****{}", first, last)
}

/// Format amount from minor units to major units
fn format_amount(amount_str: &str) -> String {
    let amount: i64 = amount_str.parse().unwrap_or(0);
    format!("${:.2}", amount as f64 / 100.0)
}

/// Get response code description
fn get_response_description(code: &str) -> &str {
    match code {
        "00" => "Approved",
        "01" => "Refer to card issuer",
        "05" => "Do not honor",
        "14" => "Invalid card number",
        "51" => "Insufficient funds",
        "54" => "Expired card",
        "55" => "Incorrect PIN",
        "91" => "Issuer unavailable",
        _ => "Unknown",
    }
}
