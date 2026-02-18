# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Security Considerations

### What This Library Provides

- **Memory Safety**: Rust's ownership system prevents memory corruption vulnerabilities  
- **Type Safety**: Compile-time guarantees prevent type confusion  
- **PAN Masking**: Utilities for masking card numbers in logs  
- **Input Validation**: Field format and length validation  
- **Luhn Check**: Card number validation algorithm  

### What You Must Implement

- **Encryption**: This library handles message structure only. You MUST implement:
- TLS/SSL for network communication
- PIN encryption (typically using HSM)
- MAC (Message Authentication Code) generation
- Key management for cryptographic operations

- **Authentication**: Implement proper authentication for:
- Terminal authentication
- Card authentication (EMV)
- Network participants

- **Logging**: When logging:
- Always mask PANs using `utils::mask_pan()`
- Never log PINs or cryptographic keys
- Never log full Track 2 data
- Sanitize all sensitive fields

## Reporting a Vulnerability

**Please DO NOT report security vulnerabilities through public GitHub issues.**

Instead, please send an email to: kingidrogen@gmail.com

Include:
- Type of vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

We will respond within 48 hours and provide a timeline for fixes.

## Security Best Practices

### PCI DSS Compliance

This library can help with PCI DSS compliance but does not guarantee it. Requirements:

1. **Cardholder Data Protection**
   - Always mask PANs when displaying or logging
   - Encrypt sensitive data in storage and transit
   - Never store CVV2, PIN, or full track data after authorization

2. **Secure Development**
   - Keep dependencies up to date (`cargo audit`)
   - Use compiler warnings as errors (`RUSTFLAGS="-D warnings"`)
   - Run security audits regularly

3. **Access Control**
   - Implement proper authentication
   - Use principle of least privilege
   - Audit all access to cardholder data

### Example: Secure Logging

```rust
use iso8583_core::*;

// ❌ WRONG - Logs full PAN
log::info!("Transaction for card {}", pan);

// - CORRECT - Masks PAN
log::info!("Transaction for card {}", mask_pan(&pan));
```

### Example: Never Log Sensitive Fields

```rust
// ❌ NEVER DO THIS
log::debug!("PIN: {:?}", message.get_field(Field::PersonalIdentificationNumberData));
log::debug!("Track2: {}", message.get_field(Field::Track2Data));
log::debug!("CVV: {}", cvv);

// - CORRECT - Don't log sensitive data at all
log::debug!("PIN verification requested");
log::debug!("Track2 data present: {}", message.has_field(Field::Track2Data));
```

## Vulnerability Disclosure Timeline

1. **Day 0**: Vulnerability reported
2. **Day 2**: Acknowledgment sent
3. **Day 7**: Impact assessment complete
4. **Day 14**: Fix developed and tested
5. **Day 21**: Security patch released
6. **Day 30**: Public disclosure (if critical)

## Known Limitations

### Out of Scope

This library does NOT provide:
- Network communication
- Cryptographic operations (encryption, MAC, PIN verification)
- HSM integration
- Key management
- Certificate validation
- Secure element integration
- EMV chip card processing (format only, not cryptography)

### In Scope

This library DOES provide:
- Message parsing and generation
- Field validation
- PAN masking utilities
- Luhn check algorithm
- Safe memory handling (Rust guarantees)

## Cryptographic Recommendations

For production use, implement:

1. **TLS 1.3** for network communication
2. **AES-256** for data encryption
3. **HMAC-SHA256** for MAC generation
4. **RSA-2048 or ECC-256** for key exchange
5. **FIPS 140-2 Level 2+** HSM for key storage

## Compliance Checklist

- [ ] PAN masking implemented in all logs
- [ ] PIN encryption using HSM
- [ ] TLS/SSL for all network communication
- [ ] MAC validation for all messages
- [ ] Secure key storage (HSM or equivalent)
- [ ] Access controls and audit logging
- [ ] Regular security testing
- [ ] Vulnerability scanning
- [ ] Dependency auditing (`cargo audit`)
- [ ] Code review process
- [ ] Incident response plan
- [ ] PCI DSS certification (if applicable)


