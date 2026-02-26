# ISO 8583 Library - Complete User Guide

## Table of Contents
1. [Installation](#installation)
2. [Quick Start](#quick-start)
3. [Core Concepts](#core-concepts)
4. [Building Messages](#building-messages)
5. [Parsing Messages](#parsing-messages)
6. [Field Reference](#field-reference)
7. [Response Codes](#response-codes)
8. [Processing Codes](#processing-codes)
9. [Utilities](#utilities)
10. [Best Practices](#best-practices)
11. [Examples](#examples)

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
iso8583-core = "0.1.0"
```

Or use from local path:

```toml
[dependencies]
iso8583-core = { path = "../iso8583-core" }
```

---

## Quick Start

### Parse a Message

```rust
use rust_iso8583::*;

// Receive bytes from network
let bytes = receive_from_network();

// Parse the message
let message = ISO8583Message::from_bytes(&bytes)?;

// Access fields
let pan = message.get_field(Field::PrimaryAccountNumber)?;
let amount = message.get_field(Field::TransactionAmount)?;
let response_code = message.get_field(Field::ResponseCode)?;

println!("Card: {}", mask_pan(pan.as_string().unwrap()));
println!("Amount: {}", format_amount(amount.as_string().unwrap(), "$"));
println!("Status: {}", response_code);
```

### Build a Message

```rust
use rust_iso8583::*;

let message = ISO8583Message::builder()
    .mti(MessageType::AUTHORIZATION_REQUEST)
    .field(Field::PrimaryAccountNumber, "4111111111111111")
    .field(Field::ProcessingCode, "000000")
    .field(Field::TransactionAmount, "000000010000") // $100.00
    .field(Field::TransmissionDateTime, generate_transmission_datetime())
    .field(Field::SystemTraceAuditNumber, generate_stan())
    .field(Field::LocalTransactionTime, generate_local_time())
    .field(Field::LocalTransactionDate, generate_local_date())
    .field(Field::CardAcceptorTerminalIdentification, "TERM0001")
    .field(Field::CurrencyCodeTransaction, "840") // USD
    .build()?;

// Send to network
let bytes = message.to_bytes();
send_to_network(&bytes)?;
```

---

## Core Concepts

### Message Type Indicator (MTI)

The MTI identifies what the message is for:

```rust
// Authorization (check if funds available)
MessageType::AUTHORIZATION_REQUEST;  // 0100
MessageType::AUTHORIZATION_RESPONSE; // 0110

// Financial (actual money movement)
MessageType::FINANCIAL_REQUEST;   // 0200
MessageType::FINANCIAL_RESPONSE;  // 0210

// Reversal (undo a transaction)
MessageType::REVERSAL_REQUEST;    // 0400
MessageType::REVERSAL_RESPONSE;   // 0410

// Network management
MessageType::NETWORK_MANAGEMENT_REQUEST;  // 0800
MessageType::NETWORK_MANAGEMENT_RESPONSE; // 0810
```

### Bitmaps

Bitmaps indicate which fields are present:

```rust
let mut bitmap = Bitmap::new();

// Set fields as present
bitmap.set(2)?;  // PAN
bitmap.set(3)?;  // Processing Code
bitmap.set(4)?;  // Amount

// Check if field is set
if bitmap.is_set(2) {
    println!("PAN is present");
}

// Get all set fields
let fields = bitmap.get_set_fields();
println!("Fields: {:?}", fields); // [2, 3, 4]
```

### Fields

All 128 ISO 8583 fields are defined:

```rust
// Access field metadata
let field = Field::PrimaryAccountNumber;
let def = field.definition();

println!("Field {}: {}", def.number, def.name);
println!("Type: {:?}", def.field_type);
println!("Description: {}", def.description);
```

---

## Building Messages

### Authorization Request (ATM Withdrawal)

```rust
let message = ISO8583Message::builder()
    .mti(MessageType::AUTHORIZATION_REQUEST)
    .field(Field::PrimaryAccountNumber, "4111111111111111")
    .field(Field::ProcessingCode, "010000") // Withdrawal
    .field(Field::TransactionAmount, "000000020000") // $200.00
    .field(Field::TransmissionDateTime, "0115150000")
    .field(Field::SystemTraceAuditNumber, "123456")
    .field(Field::LocalTransactionTime, "150000")
    .field(Field::LocalTransactionDate, "0115")
    .field(Field::PointOfServiceEntryMode, "021") // Chip + PIN
    .field(Field::CardAcceptorTerminalIdentification, "ATM00001")
    .field(Field::CurrencyCodeTransaction, "840")
    .build()?;
```

### Authorization Response

```rust
let response = ISO8583Message::builder()
    .mti(MessageType::AUTHORIZATION_RESPONSE)
    .field(Field::PrimaryAccountNumber, "4111111111111111")
    .field(Field::ProcessingCode, "010000")
    .field(Field::TransactionAmount, "000000020000")
    .field(Field::TransmissionDateTime, "0115150001")
    .field(Field::SystemTraceAuditNumber, "123456")
    .field(Field::LocalTransactionTime, "150001")
    .field(Field::LocalTransactionDate, "0115")
    .field(Field::AuthorizationIdentificationResponse, "AUTH01")
    .field(Field::ResponseCode, "00") // Approved
    .build()?;
```

### Balance Inquiry

```rust
let inquiry = ISO8583Message::builder()
    .mti(MessageType::AUTHORIZATION_REQUEST)
    .field(Field::PrimaryAccountNumber, "4111111111111111")
    .field(Field::ProcessingCode, "310000") // Balance inquiry
    .field(Field::TransactionAmount, "000000000000") // Zero for inquiry
    .field(Field::SystemTraceAuditNumber, generate_stan())
    .field(Field::LocalTransactionTime, generate_local_time())
    .field(Field::LocalTransactionDate, generate_local_date())
    .build()?;
```

---

## Parsing Messages

### Basic Parsing

```rust
// Parse from bytes
let message = ISO8583Message::from_bytes(&bytes)?;

// Check MTI
println!("Message type: {}", message.mti);

// Access fields
if let Some(pan) = message.get_field(Field::PrimaryAccountNumber) {
    println!("PAN: {}", mask_pan(pan.as_string().unwrap()));
}

// Check if field exists
if message.has_field(Field::ResponseCode) {
    println!("This is a response message");
}
```

### Error Handling

```rust
match ISO8583Message::from_bytes(&bytes) {
    Ok(message) => {
        // Process message
        process_transaction(&message)?;
    }
    Err(ISO8583Error::InvalidMTI(mti)) => {
        eprintln!("Invalid MTI: {}", mti);
    }
    Err(ISO8583Error::MessageTooShort { expected, actual }) => {
        eprintln!("Message too short: expected {}, got {}", expected, actual);
    }
    Err(e) => {
        eprintln!("Parse error: {}", e);
    }
}
```

---

## Field Reference

### Most Common Fields

| Field | Name | Format | Example |
|-------|------|--------|---------|
| 2 | Primary Account Number | LLVAR n..19 | `"4111111111111111"` |
| 3 | Processing Code | n 6 | `"000000"` |
| 4 | Transaction Amount | n 12 | `"000000010000"` |
| 7 | Transmission Date/Time | n 10 | `"0115120000"` |
| 11 | STAN | n 6 | `"123456"` |
| 12 | Local Time | n 6 | `"120000"` |
| 13 | Local Date | n 4 | `"0115"` |
| 14 | Expiration Date | n 4 | `"2512"` |
| 22 | POS Entry Mode | n 3 | `"051"` |
| 37 | Retrieval Reference Number | an 12 | `"000115123456"` |
| 38 | Authorization ID Response | an 6 | `"AUTH01"` |
| 39 | Response Code | an 2 | `"00"` |
| 41 | Terminal ID | ans 8 | `"TERM0001"` |
| 42 | Merchant ID | ans 15 | `"MERCHANT000001"` |
| 49 | Currency Code | an 3 | `"840"` |

---

## Response Codes

### Using Response Codes

```rust
use rust_iso8583::ResponseCode;

// Parse response code
let code = ResponseCode::from_str("00").unwrap();

// Check status
if code.is_approved() {
    println!("Transaction approved!");
} else if code.is_declined() {
    println!("Transaction declined: {}", code.description());
}

// Get category
match code.category() {
    ResponseCategory::Approved => dispense_cash(),
    ResponseCategory::InsufficientFunds => show_error("Insufficient funds"),
    ResponseCategory::PINError => request_pin_again(),
    ResponseCategory::SystemError => retry_transaction(),
    _ => show_generic_error(),
}
```

### Common Response Codes

| Code | Description | Action |
|------|-------------|--------|
| 00 | Approved | Complete transaction |
| 01 | Refer to issuer | Call bank |
| 05 | Do not honor | Decline |
| 14 | Invalid card number | Decline |
| 51 | Insufficient funds | Decline |
| 54 | Expired card | Decline |
| 55 | Incorrect PIN | Request PIN again |
| 91 | Issuer unavailable | Retry later |

---

## Processing Codes

### Using Processing Codes

```rust
use rust_iso8583::ProcessingCode;

// Common codes
let purchase = ProcessingCode::PURCHASE; // "000000"
let withdrawal = ProcessingCode::WITHDRAWAL_CHECKING; // "010000"
let balance = ProcessingCode::BALANCE_INQUIRY_CHECKING; // "310000"

// Parse processing code
let code = ProcessingCode::from_str("010000").unwrap();

// Get description
println!("{}", code.description()); // "Cash Withdrawal"

// Check type
if code.is_cash() {
    prepare_cash_dispensing();
} else if code.is_inquiry() {
    query_balance();
}
```

### Processing Code Format

```
TTFFTT
││││└└── To Account Type (00, 10, 20, ...)
││└└──── From Account Type (00, 10, 20, ...)
└└────── Transaction Type (00, 01, 21, 31, ...)

Examples:
000000 - Purchase from default account
010000 - Withdrawal from default (checking)
011000 - Withdrawal from savings (10)
210000 - Deposit to default (checking)
310000 - Balance inquiry checking
311000 - Balance inquiry savings
```

---

## Utilities

### PAN Masking

```rust
use rust_iso8583::utils::mask_pan;

let masked = mask_pan("4111111111111111");
println!("{}", masked); // "411111****1111"
```

### Amount Formatting

```rust
use rust_iso8583::utils::{format_amount, parse_amount};

// Format for display
let display = format_amount("000000010000", "$");
println!("{}", display); // "$100.00"

// Parse from decimal
let field_value = parse_amount(100.50);
println!("{}", field_value); // "000000010050"
```

### Date/Time Generation

```rust
use rust_iso8583::utils::*;

// Generate current date/time
let dt = generate_transmission_datetime(); // "0115120530"
let time = generate_local_time();          // "120530"
let date = generate_local_date();          // "0115"

// Generate transaction IDs
let stan = generate_stan();                // "123456"
let rrn = generate_rrn();                  // "240115123456"
let auth_id = generate_auth_id();          // "A1B2C3"
```

### Currency Utilities

```rust
use rust_iso8583::utils::{currency_symbol, currency_name};

let symbol = currency_symbol("840");  // "$"
let symbol = currency_symbol("566");  // "₦"
let name = currency_name("840");      // "US Dollar"
```

---

## Best Practices

### 1. Always Validate

```rust
// Validate PAN
if !Validator::validate_pan(&pan) {
    return Err("Invalid card number");
}

// Validate required fields
Validator::validate_required_fields(&message)?;
```

### 2. Mask Sensitive Data

```rust
// In logs
log::info!("Transaction for card: {}", mask_pan(&pan));

// Never log full PAN or PIN
// log::info!("PAN: {}", pan); //  DON'T DO THIS
```

### 3. Use Constants

```rust
// Good
.field(Field::ProcessingCode, ProcessingCode::PURCHASE.to_string())

// Avoid magic strings
// .field(Field::ProcessingCode, "000000") //  Less clear
```

### 4. Handle Errors

```rust
// Good - specific error handling
match send_transaction(&message) {
    Ok(response) => process_response(response),
    Err(ISO8583Error::InvalidMTI(_)) => log_protocol_error(),
    Err(ISO8583Error::MessageTooShort { .. }) => log_network_error(),
    Err(e) => log_unknown_error(e),
}
```

### 5. Use Builder Pattern

```rust
// Good - clear and type-safe
let message = ISO8583Message::builder()
    .mti(MessageType::AUTHORIZATION_REQUEST)
    .field(Field::PrimaryAccountNumber, pan)
    .field(Field::TransactionAmount, amount)
    .build()?;

// Avoid manual construction
```

---

## Examples

### Complete ATM Withdrawal Flow

```rust
// 1. Authorization Request
let auth_req = ISO8583Message::builder()
    .mti(MessageType::AUTHORIZATION_REQUEST)
    .field(Field::PrimaryAccountNumber, "4111111111111111")
    .field(Field::ProcessingCode, "010000")
    .field(Field::TransactionAmount, "000000020000")
    .field(Field::SystemTraceAuditNumber, generate_stan())
    .field(Field::LocalTransactionTime, generate_local_time())
    .field(Field::LocalTransactionDate, generate_local_date())
    .build()?;

send_to_bank(&auth_req.to_bytes())?;

// 2. Receive Authorization Response
let auth_resp_bytes = receive_from_bank()?;
let auth_resp = ISO8583Message::from_bytes(&auth_resp_bytes)?;

let response_code = ResponseCode::from_str(
    auth_resp.get_field(Field::ResponseCode)?.as_string().unwrap()
).unwrap();

if !response_code.is_approved() {
    return Err("Authorization declined");
}

// 3. Financial Request
let fin_req = ISO8583Message::builder()
    .mti(MessageType::FINANCIAL_REQUEST)
    .field(Field::PrimaryAccountNumber, "4111111111111111")
    .field(Field::ProcessingCode, "010000")
    .field(Field::TransactionAmount, "000000020000")
    .field(Field::SystemTraceAuditNumber, generate_stan())
    .field(Field::LocalTransactionTime, generate_local_time())
    .field(Field::LocalTransactionDate, generate_local_date())
    .build()?;

send_to_bank(&fin_req.to_bytes())?;

// 4. Receive Financial Response
let fin_resp_bytes = receive_from_bank()?;
let fin_resp = ISO8583Message::from_bytes(&fin_resp_bytes)?;

// 5. Dispense cash if approved
let final_code = ResponseCode::from_str(
    fin_resp.get_field(Field::ResponseCode)?.as_string().unwrap()
).unwrap();

if final_code.is_approved() {
    dispense_cash(200.00);
    print_receipt(&fin_resp);
}
```

### POS Purchase with Reversal

```rust
// Make purchase
let purchase = ISO8583Message::builder()
    .mti(MessageType::AUTHORIZATION_REQUEST)
    .field(Field::PrimaryAccountNumber, "5500000000000004")
    .field(Field::ProcessingCode, "000000")
    .field(Field::TransactionAmount, "000000007550")
    .field(Field::SystemTraceAuditNumber, "123456")
    .build()?;

send_to_acquirer(&purchase.to_bytes())?;

// If communication error, send reversal
let reversal = ISO8583Message::builder()
    .mti(MessageType::REVERSAL_REQUEST)
    .field(Field::PrimaryAccountNumber, "5500000000000004")
    .field(Field::ProcessingCode, "000000")
    .field(Field::TransactionAmount, "000000007550")
    .field(Field::SystemTraceAuditNumber, "123456") // Same STAN
    .field(Field::RetrievalReferenceNumber, "000115123456")
    .build()?;

send_to_acquirer(&reversal.to_bytes())?;
```

---

## Troubleshooting

### Parse Errors

```rust
// Check message length
if bytes.len() < 12 {
    eprintln!("Message too short: {} bytes", bytes.len());
}

// Verify MTI
let mti_str = String::from_utf8_lossy(&bytes[..4]);
println!("MTI: {}", mti_str);

// Check bitmap
let bitmap_bytes = &bytes[4..12];
println!("Bitmap: {}", hex::encode(bitmap_bytes));
```

### Field Issues

```rust
// Check if field is present
if !message.has_field(Field::ResponseCode) {
    eprintln!("Missing response code!");
}

// List all fields
let fields = message.get_field_numbers();
println!("Present fields: {:?}", fields);
```

---

## Performance Tips

1. **Reuse Builders**: Create builder once, reuse for multiple messages
2. **Batch Processing**: Process multiple messages before I/O
3. **Pre-allocate**: Use `Vec::with_capacity` for known sizes
4. **Avoid String Allocation**: Use string slices where possible

---

