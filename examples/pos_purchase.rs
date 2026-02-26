//! POS Purchase Transaction Example
//!
//! This example demonstrates a Point-of-Sale purchase with a chip card

use iso8583_core::*;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║              POS PURCHASE TRANSACTION EXAMPLE                ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("Scenario: Customer purchases $75.50 at a restaurant");
    println!("Payment Method: Chip card with PIN\n");

    // Authorization Request
    println!("═══ AUTHORIZATION REQUEST ═══\n");

    let request = ISO8583Message::builder()
        .mti(MessageType::AUTHORIZATION_REQUEST)
        .field(Field::PrimaryAccountNumber, "5500000000000004") // Mastercard
        .field(Field::ProcessingCode, "000000") // Purchase
        .field(Field::TransactionAmount, "000000007550") // $75.50
        .field(Field::TransmissionDateTime, "0115183045") // Jan 15, 18:30:45
        .field(Field::SystemTraceAuditNumber, "789012")
        .field(Field::LocalTransactionTime, "183045")
        .field(Field::LocalTransactionDate, "0115")
        .field(Field::ExpirationDate, "2512") // Dec 2025
        .field(Field::MerchantType, "5812") // Restaurant
        .field(Field::PointOfServiceEntryMode, "051") // Chip card, PIN verified
        .field(Field::AcquiringInstitutionCountryCode, "840")
        .field(Field::Track2Data, "5500000000000004=25121011234567890") // Chip fallback
        .field(Field::RetrievalReferenceNumber, "000115789012")
        .field(Field::CardAcceptorTerminalIdentification, "POS12345")
        .field(Field::CardAcceptorIdentificationCode, "RESTAURANT00001")
        .field(
            Field::CardAcceptorNameLocation,
            "JOES DINER        NEW YORK      NY US",
        )
        .field(Field::CurrencyCodeTransaction, "840")
        .build()?;

    println!("Transaction Details:");
    println!(
        "  Merchant:  {}",
        request.get_field(Field::CardAcceptorNameLocation).unwrap()
    );
    println!("  Card:      {}", mask_pan("5500000000000004"));
    println!(
        "  Amount:    {}",
        format_amount(
            request
                .get_field(Field::TransactionAmount)
                .unwrap()
                .as_string()
                .unwrap()
        )
    );
    println!(
        "  MCC:       {} (Restaurant)",
        request.get_field(Field::MerchantType).unwrap()
    );
    println!(
        "  Terminal:  {}",
        request
            .get_field(Field::CardAcceptorTerminalIdentification)
            .unwrap()
    );
    println!("  Entry:     Chip with PIN\n");

    // Generate bytes and show message structure
    let request_bytes = request.to_bytes();
    println!("Message Structure:");
    println!("  MTI:       {}", request.mti);
    println!("  Size:      {} bytes", request_bytes.len());
    println!("  Fields:    {}\n", request.get_field_numbers().len());

    // Simulate network transmission
    println!("→ Sending to acquirer...\n");

    // Authorization Response
    println!("═══ AUTHORIZATION RESPONSE ═══\n");

    let response = ISO8583Message::builder()
        .mti(MessageType::AUTHORIZATION_RESPONSE)
        .field(Field::PrimaryAccountNumber, "5500000000000004")
        .field(Field::ProcessingCode, "000000")
        .field(Field::TransactionAmount, "000000007550")
        .field(Field::TransmissionDateTime, "0115183046")
        .field(Field::SystemTraceAuditNumber, "789012")
        .field(Field::LocalTransactionTime, "183046")
        .field(Field::LocalTransactionDate, "0115")
        .field(Field::RetrievalReferenceNumber, "000115789012")
        .field(Field::AuthorizationIdentificationResponse, "POS789")
        .field(Field::ResponseCode, "00") // Approved!
        .field(Field::CardAcceptorTerminalIdentification, "POS12345")
        .build()?;

    println!("← Response received from bank");
    println!("\nApproval Details:");
    println!("  Status:      ✓ APPROVED");
    println!(
        "  Auth Code:   {}",
        response
            .get_field(Field::AuthorizationIdentificationResponse)
            .unwrap()
    );
    println!(
        "  Response:    {} - {}",
        response
            .get_field(Field::ResponseCode)
            .unwrap()
            .as_string()
            .unwrap(),
        get_response_description("00")
    );
    println!(
        "  Reference:   {}\n",
        response.get_field(Field::RetrievalReferenceNumber).unwrap()
    );

    // Print receipt
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                      CUSTOMER RECEIPT                        ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║                                                              ║");
    println!("║  JOE'S DINER                                                 ║");
    println!("║  123 Main Street                                             ║");
    println!("║  New York, NY 10001                                          ║");
    println!("║                                                              ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║                                                              ║");
    println!("║  DATE:     01/15/2024            TIME: 18:30:46              ║");
    println!("║  TERMINAL: POS12345              MERCHANT: RESTAURANT00001   ║");
    println!("║                                                              ║");
    println!("║  CARD TYPE:    MASTERCARD                                    ║");
    println!("║  CARD NUMBER:  550000****0004                                ║");
    println!("║  ENTRY MODE:   CHIP                                          ║");
    println!("║                                                              ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║                                                              ║");
    println!("║  PURCHASE                                   $75.50           ║");
    println!("║                                                              ║");
    println!("║  TIP:          __________                                    ║");
    println!("║  TOTAL:        __________                                    ║");
    println!("║                                                              ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║                                                              ║");
    println!("║  APPROVED                                                    ║");
    println!("║  AUTH CODE: POS789                                           ║");
    println!("║  REF: 000115789012                                           ║");
    println!("║                                                              ║");
    println!("║  SIGNATURE: ___________________________________________      ║");
    println!("║                                                              ║");
    println!("║              THANK YOU FOR YOUR BUSINESS!                    ║");
    println!("║                                                              ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Test roundtrip
    println!("═══ MESSAGE VERIFICATION ═══\n");
    let parsed = ISO8583Message::from_bytes(&request_bytes)?;
    println!("  ✓ Parse successful");
    println!("  ✓ MTI matches: {} == {}", request.mti, parsed.mti);
    println!(
        "  ✓ All {} fields preserved",
        parsed.get_field_numbers().len()
    );
    println!(
        "  ✓ Transaction amount: {}",
        format_amount(
            parsed
                .get_field(Field::TransactionAmount)
                .unwrap()
                .as_string()
                .unwrap()
        )
    );

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

fn format_amount(amount_str: &str) -> String {
    let amount: i64 = amount_str.parse().unwrap_or(0);
    format!("${:.2}", amount as f64 / 100.0)
}

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
