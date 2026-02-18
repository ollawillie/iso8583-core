//! Balance Inquiry Transaction Example
//!
//! This example demonstrates an ATM balance inquiry (no funds transfer)

use iso8583_core::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║            BALANCE INQUIRY TRANSACTION EXAMPLE               ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Balance Inquiry Request
    let request = ISO8583Message::builder()
        .mti(MessageType::AUTHORIZATION_REQUEST)
        .field(Field::PrimaryAccountNumber, "4111111111111111")
        .field(Field::ProcessingCode, "310000") // Balance inquiry, checking account
        .field(Field::TransactionAmount, "000000000000") // No amount for inquiry
        .field(Field::TransmissionDateTime, "0115093000")
        .field(Field::SystemTraceAuditNumber, "345678")
        .field(Field::LocalTransactionTime, "093000")
        .field(Field::LocalTransactionDate, "0115")
        .field(Field::PointOfServiceEntryMode, "021") // Chip with PIN
        .field(Field::AcquiringInstitutionCountryCode, "840")
        .field(Field::CardAcceptorTerminalIdentification, "ATM00456")
        .field(Field::CurrencyCodeTransaction, "840")
        .build()?;

    println!("Balance Inquiry Request:");
    println!("  Card:      {}", mask_pan("4111111111111111"));
    println!("  Terminal:  {}", request.get_field(Field::CardAcceptorTerminalIdentification).unwrap());
    println!("  Type:      Checking Account Balance");
    println!("  Time:      09:30:00\n");

    // Response with balance
    let response = ISO8583Message::builder()
        .mti(MessageType::AUTHORIZATION_RESPONSE)
        .field(Field::PrimaryAccountNumber, "4111111111111111")
        .field(Field::ProcessingCode, "310000")
        .field(Field::TransactionAmount, "000000000000")
        .field(Field::TransmissionDateTime, "0115093001")
        .field(Field::SystemTraceAuditNumber, "345678")
        .field(Field::LocalTransactionTime, "093001")
        .field(Field::LocalTransactionDate, "0115")
        .field(Field::ResponseCode, "00")
        .field(Field::AdditionalAmounts, "0084001C000000123456") // Available balance format
        .build()?;

    println!("Balance Inquiry Response:");
    println!("  Status:    ✓ APPROVED");
    println!("  Response:  00 - Approved\n");

    // Display balance (parsed from field 54)
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    BALANCE INQUIRY RESULT                    ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║                                                              ║");
    println!("║  ACCOUNT TYPE:    Checking                                   ║");
    println!("║  CARD NUMBER:     411111****1111                             ║");
    println!("║                                                              ║");
    println!("║  AVAILABLE BALANCE:        $1,234.56                         ║");
    println!("║  CURRENT BALANCE:          $1,234.56                         ║");
    println!("║                                                              ║");
    println!("║  DATE: 01/15/2024          TIME: 09:30:01                    ║");
    println!("║  TERMINAL: ATM00456                                          ║");
    println!("║                                                              ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Processing Code explanation
    println!("═══ PROCESSING CODE BREAKDOWN ═══\n");
    println!("Processing Code: 310000");
    println!("  Position 1-2 (31): Balance Inquiry");
    println!("  Position 3-4 (00): From Checking Account");
    println!("  Position 5-6 (00): Default/Checking Account\n");

    println!("Other common processing codes:");
    println!("  010000 - Withdrawal from Checking");
    println!("  210000 - Deposit to Checking");
    println!("  000000 - Purchase");
    println!("  200000 - Refund\n");

    Ok(())
}

fn mask_pan(pan: &str) -> String {
    if pan.len() < 10 {
        return "*".repeat(pan.len());
    }
    let first = &pan[..6];
    let last = &pan[pan.len() - 4..];
    format!("{}****{}", first, last)
}
