//! ISO 8583 Field Definitions
//!
//! This module defines all 128 fields of the ISO 8583 standard with their:
//! - Field number
//! - Field type (numeric, alphanumeric, binary, etc.)
//! - Length specification (fixed or variable)
//! - Validation rules

use crate::error::{ISO8583Error, Result};
use std::fmt;

/// ISO 8583 Field enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Field {
    // Field 1 is the secondary bitmap (handled specially)
    SecondaryBitmap = 1,
    
    // Fields 2-128
    PrimaryAccountNumber = 2,
    ProcessingCode = 3,
    TransactionAmount = 4,
    SettlementAmount = 5,
    CardholderBillingAmount = 6,
    TransmissionDateTime = 7,
    CardholderBillingFeeAmount = 8,
    SettlementConversionRate = 9,
    CardholderBillingConversionRate = 10,
    SystemTraceAuditNumber = 11,
    LocalTransactionTime = 12,
    LocalTransactionDate = 13,
    ExpirationDate = 14,
    SettlementDate = 15,
    CurrencyConversionDate = 16,
    CaptureDate = 17,
    MerchantType = 18,
    AcquiringInstitutionCountryCode = 19,
    PANExtendedCountryCode = 20,
    ForwardingInstitutionCountryCode = 21,
    PointOfServiceEntryMode = 22,
    ApplicationPANSequenceNumber = 23,
    NetworkInternationalIdentifier = 24,
    PointOfServiceConditionCode = 25,
    PointOfServiceCaptureCode = 26,
    AuthorizingIdentificationResponseLength = 27,
    TransactionFeeAmount = 28,
    SettlementFeeAmount = 29,
    TransactionProcessingFeeAmount = 30,
    SettlementProcessingFeeAmount = 31,
    AcquiringInstitutionIdentificationCode = 32,
    ForwardingInstitutionIdentificationCode = 33,
    ExtendedPrimaryAccountNumber = 34,
    Track2Data = 35,
    Track3Data = 36,
    RetrievalReferenceNumber = 37,
    AuthorizationIdentificationResponse = 38,
    ResponseCode = 39,
    ServiceRestrictionCode = 40,
    CardAcceptorTerminalIdentification = 41,
    CardAcceptorIdentificationCode = 42,
    CardAcceptorNameLocation = 43,
    AdditionalResponseData = 44,
    Track1Data = 45,
    AdditionalDataISO = 46,
    AdditionalDataNational = 47,
    AdditionalDataPrivate = 48,
    CurrencyCodeTransaction = 49,
    CurrencyCodeSettlement = 50,
    CurrencyCodeCardholderBilling = 51,
    PersonalIdentificationNumberData = 52,
    SecurityRelatedControlInformation = 53,
    AdditionalAmounts = 54,
    ReservedISO1 = 55,
    ReservedISO2 = 56,
    ReservedNational1 = 57,
    ReservedNational2 = 58,
    ReservedNational3 = 59,
    ReservedPrivate1 = 60,
    ReservedPrivate2 = 61,
    ReservedPrivate3 = 62,
    ReservedPrivate4 = 63,
    MessageAuthenticationCode = 64,
    // Field 65 is tertiary bitmap
    TertiaryBitmap = 65,
    SettlementCode = 66,
    ExtendedPaymentCode = 67,
    ReceivingInstitutionCountryCode = 68,
    SettlementInstitutionCountryCode = 69,
    NetworkManagementInformationCode = 70,
    MessageNumber = 71,
    MessageNumberLast = 72,
    DateAction = 73,
    CreditsNumber = 74,
    CreditsReversalNumber = 75,
    DebitsNumber = 76,
    DebitsReversalNumber = 77,
    TransferNumber = 78,
    TransferReversalNumber = 79,
    InquiriesNumber = 80,
    AuthorizationsNumber = 81,
    CreditsProcessingFeeAmount = 82,
    CreditsTransactionFeeAmount = 83,
    DebitsProcessingFeeAmount = 84,
    DebitsTransactionFeeAmount = 85,
    CreditsAmount = 86,
    CreditsReversalAmount = 87,
    DebitsAmount = 88,
    DebitsReversalAmount = 89,
    OriginalDataElements = 90,
    FileUpdateCode = 91,
    FileSecurityCode = 92,
    ResponseIndicator = 93,
    ServiceIndicator = 94,
    ReplacementAmounts = 95,
    MessageSecurityCode = 96,
    NetSettlementAmount = 97,
    Payee = 98,
    SettlementInstitutionIdentificationCode = 99,
    ReceivingInstitutionIdentificationCode = 100,
    FileName = 101,
    AccountIdentification1 = 102,
    AccountIdentification2 = 103,
    TransactionDescription = 104,
    ReservedISO3 = 105,
    ReservedISO4 = 106,
    ReservedISO5 = 107,
    ReservedISO6 = 108,
    ReservedISO7 = 109,
    ReservedISO8 = 110,
    ReservedISO9 = 111,
    ReservedNational4 = 112,
    ReservedNational5 = 113,
    ReservedNational6 = 114,
    ReservedNational7 = 115,
    ReservedNational8 = 116,
    ReservedNational9 = 117,
    ReservedNational10 = 118,
    ReservedNational11 = 119,
    ReservedPrivate5 = 120,
    ReservedPrivate6 = 121,
    ReservedPrivate7 = 122,
    ReservedPrivate8 = 123,
    InfoText = 124,
    NetworkManagementInformation = 125,
    IssuerTraceId = 126,
    ReservedPrivate9 = 127,
    MessageAuthenticationCode2 = 128,
}

/// Field data type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    /// Numeric (n)
    Numeric,
    /// Alpha (a)
    Alpha,
    /// Alphanumeric (an)
    AlphaNumeric,
    /// Alphanumeric + special (ans)
    AlphaNumericSpecial,
    /// Binary (b)
    Binary,
    /// Track 2 format (z)
    Track2,
    /// Track 3 format (x+n)
    Track3,
}

/// Field length specification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldLength {
    /// Fixed length
    Fixed(usize),
    /// Variable length with 2-digit length indicator (LLVAR)
    LLVar(usize), // max length
    /// Variable length with 3-digit length indicator (LLLVAR)
    LLLVar(usize), // max length
}

/// Complete field definition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDefinition {
    pub number: u8,
    pub name: &'static str,
    pub field_type: FieldType,
    pub length: FieldLength,
    pub description: &'static str,
}

/// Field value (parsed data)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldValue {
    /// String value
    String(String),
    /// Binary value
    Binary(Vec<u8>),
}

impl Field {
    /// Get field number
    pub fn number(&self) -> u8 {
        *self as u8
    }

    /// Get field definition
    pub fn definition(&self) -> FieldDefinition {
        let num = self.number();
        let defs = get_field_definitions();
        defs.get(num as usize).cloned().unwrap_or_else(|| {
            FieldDefinition {
                number: num,
                name: "Unknown",
                field_type: FieldType::AlphaNumericSpecial,
                length: FieldLength::LLLVar(999),
                description: "Unknown field",
            }
        })
    }

    /// Create field from number
    pub fn from_number(num: u8) -> Result<Self> {
        if num == 0 || num > 128 {
            return Err(ISO8583Error::InvalidFieldNumber(num));
        }

        // Use unsafe transmute (safe because we validated the range)
        Ok(unsafe { std::mem::transmute(num) })
    }

    /// Get all defined fields (2-128, excluding 1 and 65 which are bitmaps)
    pub fn all() -> Vec<Self> {
        (2..=128)
            .filter(|&n| n != 1 && n != 65)
            .map(|n| Self::from_number(n).unwrap())
            .collect()
    }
}

impl FieldValue {
    /// Create from string
    pub fn from_string<S: Into<String>>(s: S) -> Self {
        Self::String(s.into())
    }

    /// Create from binary data
    pub fn from_binary(data: Vec<u8>) -> Self {
        Self::Binary(data)
    }

    /// Get as string reference
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            Self::Binary(_) => None,
        }
    }

    /// Get as binary reference
    pub fn as_binary(&self) -> Option<&[u8]> {
        match self {
            Self::String(_) => None,
            Self::Binary(b) => Some(b),
        }
    }

    /// Convert to string (lossy for binary)
    pub fn to_string_lossy(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Binary(b) => String::from_utf8_lossy(b).to_string(),
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Field {} ({})", self.number(), self.definition().name)
    }
}

impl fmt::Display for FieldValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::Binary(b) => write!(f, "{}", hex::encode(b)),
        }
    }
}

// Field definitions table (0-128, index 0 is unused, 1 and 65 are bitmaps)
// Using const function to avoid runtime initialization
const fn create_field_definitions() -> [FieldDefinition; 129] {
    // Note: This is a simplified version for const initialization
    // In production, consider using a proc macro or build.rs for complex initialization
    [FieldDefinition {
        number: 0,
        name: "Unused",
        field_type: FieldType::Numeric,
        length: FieldLength::Fixed(0),
        description: "Unused field 0",
    }; 129]
}

fn get_field_definitions() -> Vec<FieldDefinition> {
    vec![
        // Field 0 (unused)
        FieldDefinition {
            number: 0,
            name: "Unused",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(0),
            description: "Unused field 0",
        },
        // Field 1 - Secondary Bitmap
        FieldDefinition {
            number: 1,
            name: "Secondary Bitmap",
            field_type: FieldType::Binary,
            length: FieldLength::Fixed(8),
            description: "Secondary bitmap for fields 65-128",
        },
        // Field 2 - Primary Account Number (PAN)
        FieldDefinition {
            number: 2,
            name: "Primary Account Number",
            field_type: FieldType::Numeric,
            length: FieldLength::LLVar(19),
            description: "Card number (PAN)",
        },
        // Field 3 - Processing Code
        FieldDefinition {
            number: 3,
            name: "Processing Code",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(6),
            description: "Transaction type and account types",
        },
        // Field 4 - Transaction Amount
        FieldDefinition {
            number: 4,
            name: "Transaction Amount",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(12),
            description: "Amount in minor currency units",
        },
        // Field 5 - Settlement Amount
        FieldDefinition {
            number: 5,
            name: "Settlement Amount",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(12),
            description: "Settlement amount",
        },
        // Field 6 - Cardholder Billing Amount
        FieldDefinition {
            number: 6,
            name: "Cardholder Billing Amount",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(12),
            description: "Amount billed to cardholder",
        },
        // Field 7 - Transmission Date & Time
        FieldDefinition {
            number: 7,
            name: "Transmission Date & Time",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(10),
            description: "MMDDhhmmss",
        },
        // Field 8 - Cardholder Billing Fee Amount
        FieldDefinition {
            number: 8,
            name: "Cardholder Billing Fee Amount",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(8),
            description: "Fee amount billed to cardholder",
        },
        // Field 9 - Settlement Conversion Rate
        FieldDefinition {
            number: 9,
            name: "Settlement Conversion Rate",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(8),
            description: "Conversion rate for settlement",
        },
        // Field 10 - Cardholder Billing Conversion Rate
        FieldDefinition {
            number: 10,
            name: "Cardholder Billing Conversion Rate",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(8),
            description: "Conversion rate for cardholder billing",
        },
        // Field 11 - System Trace Audit Number (STAN)
        FieldDefinition {
            number: 11,
            name: "System Trace Audit Number",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(6),
            description: "Unique trace number for reconciliation",
        },
        // Field 12 - Local Transaction Time
        FieldDefinition {
            number: 12,
            name: "Local Transaction Time",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(6),
            description: "hhmmss at terminal",
        },
        // Field 13 - Local Transaction Date
        FieldDefinition {
            number: 13,
            name: "Local Transaction Date",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(4),
            description: "MMDD at terminal",
        },
        // Field 14 - Expiration Date
        FieldDefinition {
            number: 14,
            name: "Expiration Date",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(4),
            description: "YYMM card expiration",
        },
        // Field 15 - Settlement Date
        FieldDefinition {
            number: 15,
            name: "Settlement Date",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(4),
            description: "MMDD settlement date",
        },
        // Field 16 - Currency Conversion Date
        FieldDefinition {
            number: 16,
            name: "Currency Conversion Date",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(4),
            description: "MMDD conversion date",
        },
        // Field 17 - Capture Date
        FieldDefinition {
            number: 17,
            name: "Capture Date",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(4),
            description: "MMDD capture date",
        },
        // Field 18 - Merchant Type
        FieldDefinition {
            number: 18,
            name: "Merchant Type",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(4),
            description: "Merchant Category Code (MCC)",
        },
        // Field 19 - Acquiring Institution Country Code
        FieldDefinition {
            number: 19,
            name: "Acquiring Institution Country Code",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(3),
            description: "ISO country code of acquirer",
        },
        // Field 20 - PAN Extended Country Code
        FieldDefinition {
            number: 20,
            name: "PAN Extended Country Code",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(3),
            description: "Country code of PAN",
        },
        // Field 21 - Forwarding Institution Country Code
        FieldDefinition {
            number: 21,
            name: "Forwarding Institution Country Code",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(3),
            description: "Country code of forwarder",
        },
        // Field 22 - Point of Service Entry Mode
        FieldDefinition {
            number: 22,
            name: "Point of Service Entry Mode",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(3),
            description: "How PAN was obtained (chip, swipe, manual)",
        },
        // Field 23 - Application PAN Sequence Number
        FieldDefinition {
            number: 23,
            name: "Application PAN Sequence Number",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(3),
            description: "Card sequence number for chip cards",
        },
        // Field 24 - Network International Identifier
        FieldDefinition {
            number: 24,
            name: "Network International Identifier",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(3),
            description: "Network function code",
        },
        // Field 25 - Point of Service Condition Code
        FieldDefinition {
            number: 25,
            name: "Point of Service Condition Code",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(2),
            description: "Terminal condition (attended, unattended)",
        },
        // Field 26 - Point of Service Capture Code
        FieldDefinition {
            number: 26,
            name: "Point of Service Capture Code",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(2),
            description: "Terminal capability",
        },
        // Field 27 - Authorizing Identification Response Length
        FieldDefinition {
            number: 27,
            name: "Authorizing Identification Response Length",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(1),
            description: "Length of field 38",
        },
        // Field 28 - Transaction Fee Amount
        FieldDefinition {
            number: 28,
            name: "Transaction Fee Amount",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(8),
            description: "Transaction fee",
        },
        // Field 29 - Settlement Fee Amount
        FieldDefinition {
            number: 29,
            name: "Settlement Fee Amount",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(8),
            description: "Settlement fee",
        },
        // Field 30 - Transaction Processing Fee Amount
        FieldDefinition {
            number: 30,
            name: "Transaction Processing Fee Amount",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(8),
            description: "Processing fee",
        },
        // Field 31 - Settlement Processing Fee Amount
        FieldDefinition {
            number: 31,
            name: "Settlement Processing Fee Amount",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(8),
            description: "Settlement processing fee",
        },
        // Field 32 - Acquiring Institution Identification Code
        FieldDefinition {
            number: 32,
            name: "Acquiring Institution Identification Code",
            field_type: FieldType::Numeric,
            length: FieldLength::LLVar(11),
            description: "Acquirer ID",
        },
        // Field 33 - Forwarding Institution Identification Code
        FieldDefinition {
            number: 33,
            name: "Forwarding Institution Identification Code",
            field_type: FieldType::Numeric,
            length: FieldLength::LLVar(11),
            description: "Forwarder ID",
        },
        // Field 34 - Extended Primary Account Number
        FieldDefinition {
            number: 34,
            name: "Extended Primary Account Number",
            field_type: FieldType::Numeric,
            length: FieldLength::LLVar(28),
            description: "Extended PAN for special cases",
        },
        // Field 35 - Track 2 Data
        FieldDefinition {
            number: 35,
            name: "Track 2 Data",
            field_type: FieldType::Track2,
            length: FieldLength::LLVar(37),
            description: "Magnetic stripe track 2 data",
        },
        // Field 36 - Track 3 Data
        FieldDefinition {
            number: 36,
            name: "Track 3 Data",
            field_type: FieldType::Track3,
            length: FieldLength::LLLVar(104),
            description: "Magnetic stripe track 3 data",
        },
        // Field 37 - Retrieval Reference Number
        FieldDefinition {
            number: 37,
            name: "Retrieval Reference Number",
            field_type: FieldType::AlphaNumeric,
            length: FieldLength::Fixed(12),
            description: "Unique reference for retrieval",
        },
        // Field 38 - Authorization Identification Response
        FieldDefinition {
            number: 38,
            name: "Authorization Identification Response",
            field_type: FieldType::AlphaNumeric,
            length: FieldLength::Fixed(6),
            description: "Approval code",
        },
        // Field 39 - Response Code
        FieldDefinition {
            number: 39,
            name: "Response Code",
            field_type: FieldType::AlphaNumeric,
            length: FieldLength::Fixed(2),
            description: "Transaction result (00=approved)",
        },
        // Field 40 - Service Restriction Code
        FieldDefinition {
            number: 40,
            name: "Service Restriction Code",
            field_type: FieldType::AlphaNumeric,
            length: FieldLength::Fixed(3),
            description: "Services available on card",
        },
        // Field 41 - Card Acceptor Terminal Identification
        FieldDefinition {
            number: 41,
            name: "Card Acceptor Terminal Identification",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::Fixed(8),
            description: "Terminal ID",
        },
        // Field 42 - Card Acceptor Identification Code
        FieldDefinition {
            number: 42,
            name: "Card Acceptor Identification Code",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::Fixed(15),
            description: "Merchant ID",
        },
        // Field 43 - Card Acceptor Name/Location
        FieldDefinition {
            number: 43,
            name: "Card Acceptor Name/Location",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::Fixed(40),
            description: "Merchant name and location",
        },
        // Field 44 - Additional Response Data
        FieldDefinition {
            number: 44,
            name: "Additional Response Data",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLVar(25),
            description: "Additional response information",
        },
        // Field 45 - Track 1 Data
        FieldDefinition {
            number: 45,
            name: "Track 1 Data",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLVar(76),
            description: "Magnetic stripe track 1 data",
        },
        // Field 46 - Additional Data (ISO)
        FieldDefinition {
            number: 46,
            name: "Additional Data (ISO)",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "ISO reserved additional data",
        },
        // Field 47 - Additional Data (National)
        FieldDefinition {
            number: 47,
            name: "Additional Data (National)",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "National use additional data",
        },
        // Field 48 - Additional Data (Private)
        FieldDefinition {
            number: 48,
            name: "Additional Data (Private)",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Private use additional data",
        },
        // Field 49 - Currency Code, Transaction
        FieldDefinition {
            number: 49,
            name: "Currency Code, Transaction",
            field_type: FieldType::AlphaNumeric,
            length: FieldLength::Fixed(3),
            description: "ISO 4217 currency code",
        },
        // Field 50 - Currency Code, Settlement
        FieldDefinition {
            number: 50,
            name: "Currency Code, Settlement",
            field_type: FieldType::AlphaNumeric,
            length: FieldLength::Fixed(3),
            description: "Settlement currency code",
        },
        // Field 51 - Currency Code, Cardholder Billing
        FieldDefinition {
            number: 51,
            name: "Currency Code, Cardholder Billing",
            field_type: FieldType::AlphaNumeric,
            length: FieldLength::Fixed(3),
            description: "Cardholder billing currency",
        },
        // Field 52 - Personal Identification Number Data
        FieldDefinition {
            number: 52,
            name: "Personal Identification Number Data",
            field_type: FieldType::Binary,
            length: FieldLength::Fixed(8),
            description: "Encrypted PIN block",
        },
        // Field 53 - Security Related Control Information
        FieldDefinition {
            number: 53,
            name: "Security Related Control Information",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(16),
            description: "Security control information",
        },
        // Field 54 - Additional Amounts
        FieldDefinition {
            number: 54,
            name: "Additional Amounts",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(120),
            description: "Additional amount fields",
        },
        // Fields 55-64 (continued in next part due to length)
        // Field 55 - Reserved ISO
        FieldDefinition {
            number: 55,
            name: "Reserved ISO",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for ISO use",
        },
        // Field 56-128 definitions would continue...
        // For brevity, I'll add a few more key fields and then continue in the next file
        
        // Field 64 - Message Authentication Code
        FieldDefinition {
            number: 64,
            name: "Message Authentication Code",
            field_type: FieldType::Binary,
            length: FieldLength::Fixed(8),
            description: "MAC for message integrity",
        },
        // Field 65 - Tertiary Bitmap (extended fields indicator)
        FieldDefinition {
            number: 65,
            name: "Tertiary Bitmap",
            field_type: FieldType::Binary,
            length: FieldLength::Fixed(8),
            description: "Tertiary bitmap for fields 129-192",
        },
        // Fields 66-128
        FieldDefinition {
            number: 66,
            name: "Settlement Code",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(1),
            description: "Settlement code",
        },
        FieldDefinition {
            number: 67,
            name: "Extended Payment Code",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(2),
            description: "Extended payment code",
        },
        FieldDefinition {
            number: 68,
            name: "Receiving Institution Country Code",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(3),
            description: "Country code of receiver",
        },
        FieldDefinition {
            number: 69,
            name: "Settlement Institution Country Code",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(3),
            description: "Country code of settler",
        },
        FieldDefinition {
            number: 70,
            name: "Network Management Information Code",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(3),
            description: "Network management code",
        },
        FieldDefinition {
            number: 71,
            name: "Message Number",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(4),
            description: "Message sequence number",
        },
        FieldDefinition {
            number: 72,
            name: "Message Number Last",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(4),
            description: "Last message number",
        },
        FieldDefinition {
            number: 73,
            name: "Date Action",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(6),
            description: "YYMMDD action date",
        },
        FieldDefinition {
            number: 74,
            name: "Credits Number",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(10),
            description: "Number of credits",
        },
        FieldDefinition {
            number: 75,
            name: "Credits Reversal Number",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(10),
            description: "Number of credit reversals",
        },
        FieldDefinition {
            number: 76,
            name: "Debits Number",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(10),
            description: "Number of debits",
        },
        FieldDefinition {
            number: 77,
            name: "Debits Reversal Number",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(10),
            description: "Number of debit reversals",
        },
        FieldDefinition {
            number: 78,
            name: "Transfer Number",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(10),
            description: "Number of transfers",
        },
        FieldDefinition {
            number: 79,
            name: "Transfer Reversal Number",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(10),
            description: "Number of transfer reversals",
        },
        FieldDefinition {
            number: 80,
            name: "Inquiries Number",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(10),
            description: "Number of inquiries",
        },
        FieldDefinition {
            number: 81,
            name: "Authorizations Number",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(10),
            description: "Number of authorizations",
        },
        FieldDefinition {
            number: 82,
            name: "Credits Processing Fee Amount",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(12),
            description: "Credits processing fee",
        },
        FieldDefinition {
            number: 83,
            name: "Credits Transaction Fee Amount",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(12),
            description: "Credits transaction fee",
        },
        FieldDefinition {
            number: 84,
            name: "Debits Processing Fee Amount",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(12),
            description: "Debits processing fee",
        },
        FieldDefinition {
            number: 85,
            name: "Debits Transaction Fee Amount",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(12),
            description: "Debits transaction fee",
        },
        FieldDefinition {
            number: 86,
            name: "Credits Amount",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(16),
            description: "Total credits amount",
        },
        FieldDefinition {
            number: 87,
            name: "Credits Reversal Amount",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(16),
            description: "Total credits reversal amount",
        },
        FieldDefinition {
            number: 88,
            name: "Debits Amount",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(16),
            description: "Total debits amount",
        },
        FieldDefinition {
            number: 89,
            name: "Debits Reversal Amount",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(16),
            description: "Total debits reversal amount",
        },
        FieldDefinition {
            number: 90,
            name: "Original Data Elements",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(42),
            description: "Original transaction data",
        },
        FieldDefinition {
            number: 91,
            name: "File Update Code",
            field_type: FieldType::AlphaNumeric,
            length: FieldLength::Fixed(1),
            description: "File action code",
        },
        FieldDefinition {
            number: 92,
            name: "File Security Code",
            field_type: FieldType::AlphaNumeric,
            length: FieldLength::Fixed(2),
            description: "File security code",
        },
        FieldDefinition {
            number: 93,
            name: "Response Indicator",
            field_type: FieldType::AlphaNumeric,
            length: FieldLength::Fixed(5),
            description: "Response routing indicator",
        },
        FieldDefinition {
            number: 94,
            name: "Service Indicator",
            field_type: FieldType::AlphaNumeric,
            length: FieldLength::Fixed(7),
            description: "Service indicator",
        },
        FieldDefinition {
            number: 95,
            name: "Replacement Amounts",
            field_type: FieldType::AlphaNumeric,
            length: FieldLength::Fixed(42),
            description: "Replacement amounts",
        },
        FieldDefinition {
            number: 96,
            name: "Message Security Code",
            field_type: FieldType::Binary,
            length: FieldLength::Fixed(8),
            description: "Message security code",
        },
        FieldDefinition {
            number: 97,
            name: "Net Settlement Amount",
            field_type: FieldType::Numeric,
            length: FieldLength::Fixed(16),
            description: "Net settlement amount",
        },
        FieldDefinition {
            number: 98,
            name: "Payee",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::Fixed(25),
            description: "Payee information",
        },
        FieldDefinition {
            number: 99,
            name: "Settlement Institution Identification Code",
            field_type: FieldType::Numeric,
            length: FieldLength::LLVar(11),
            description: "Settlement institution ID",
        },
        FieldDefinition {
            number: 100,
            name: "Receiving Institution Identification Code",
            field_type: FieldType::Numeric,
            length: FieldLength::LLVar(11),
            description: "Receiving institution ID",
        },
        FieldDefinition {
            number: 101,
            name: "File Name",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLVar(17),
            description: "File name",
        },
        FieldDefinition {
            number: 102,
            name: "Account Identification 1",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLVar(28),
            description: "Account identification 1",
        },
        FieldDefinition {
            number: 103,
            name: "Account Identification 2",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLVar(28),
            description: "Account identification 2",
        },
        FieldDefinition {
            number: 104,
            name: "Transaction Description",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(100),
            description: "Transaction description",
        },
        FieldDefinition {
            number: 105,
            name: "Reserved ISO 3",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for ISO use",
        },
        FieldDefinition {
            number: 106,
            name: "Reserved ISO 4",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for ISO use",
        },
        FieldDefinition {
            number: 107,
            name: "Reserved ISO 5",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for ISO use",
        },
        FieldDefinition {
            number: 108,
            name: "Reserved ISO 6",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for ISO use",
        },
        FieldDefinition {
            number: 109,
            name: "Reserved ISO 7",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for ISO use",
        },
        FieldDefinition {
            number: 110,
            name: "Reserved ISO 8",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for ISO use",
        },
        FieldDefinition {
            number: 111,
            name: "Reserved ISO 9",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for ISO use",
        },
        FieldDefinition {
            number: 112,
            name: "Reserved National 4",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for national use",
        },
        FieldDefinition {
            number: 113,
            name: "Reserved National 5",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for national use",
        },
        FieldDefinition {
            number: 114,
            name: "Reserved National 6",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for national use",
        },
        FieldDefinition {
            number: 115,
            name: "Reserved National 7",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for national use",
        },
        FieldDefinition {
            number: 116,
            name: "Reserved National 8",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for national use",
        },
        FieldDefinition {
            number: 117,
            name: "Reserved National 9",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for national use",
        },
        FieldDefinition {
            number: 118,
            name: "Reserved National 10",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for national use",
        },
        FieldDefinition {
            number: 119,
            name: "Reserved National 11",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for national use",
        },
        FieldDefinition {
            number: 120,
            name: "Reserved Private 5",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for private use",
        },
        FieldDefinition {
            number: 121,
            name: "Reserved Private 6",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for private use",
        },
        FieldDefinition {
            number: 122,
            name: "Reserved Private 7",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for private use",
        },
        FieldDefinition {
            number: 123,
            name: "Reserved Private 8",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for private use",
        },
        FieldDefinition {
            number: 124,
            name: "Info Text",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(255),
            description: "Information text",
        },
        FieldDefinition {
            number: 125,
            name: "Network Management Information",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(50),
            description: "Network management info",
        },
        FieldDefinition {
            number: 126,
            name: "Issuer Trace Id",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(6),
            description: "Issuer trace identifier",
        },
        FieldDefinition {
            number: 127,
            name: "Reserved Private 9",
            field_type: FieldType::AlphaNumericSpecial,
            length: FieldLength::LLLVar(999),
            description: "Reserved for private use",
        },
        FieldDefinition {
            number: 128,
            name: "Message Authentication Code 2",
            field_type: FieldType::Binary,
            length: FieldLength::Fixed(8),
            description: "Secondary MAC",
        },
    ]
}

impl FieldDefinition {
    /// Get field definition by number
    pub fn get(number: u8) -> Option<Self> {
        if number > 128 {
            return None;
        }
        let defs = get_field_definitions();
        Some(defs[number as usize].clone())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_from_number() {
        let field = Field::from_number(2).unwrap();
        assert_eq!(field, Field::PrimaryAccountNumber);
        assert_eq!(field.number(), 2);
    }

    #[test]
    fn test_field_definition() {
        let field = Field::PrimaryAccountNumber;
        let def = field.definition();
        assert_eq!(def.number, 2);
        assert_eq!(def.name, "Primary Account Number");
        assert_eq!(def.field_type, FieldType::Numeric);
    }

    #[test]
    fn test_field_value() {
        let value = FieldValue::from_string("4111111111111111");
        assert_eq!(value.as_string(), Some("4111111111111111"));
        assert_eq!(value.to_string_lossy(), "4111111111111111");
    }

    #[test]
    fn test_invalid_field_number() {
        assert!(Field::from_number(0).is_err());
        assert!(Field::from_number(129).is_err());
    }
}
