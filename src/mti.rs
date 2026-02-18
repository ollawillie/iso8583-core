//! Message Type Indicator (MTI) definitions and parsing
//!
//! The MTI is a 4-digit numeric field that indicates the message's purpose:
//! - Position 1: Version (0-9)
//! - Position 2: Message Class (Authorization, Financial, etc.)
//! - Position 3: Message Function (Request, Response, Advice, etc.)
//! - Position 4: Message Origin (Acquirer, Issuer, etc.)

use crate::error::{ISO8583Error, Result};
use std::fmt;

/// ISO 8583 Message Type Indicator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageType {
    pub version: u8,
    pub class: MessageClass,
    pub function: MessageFunction,
    pub origin: MessageOrigin,
}

/// Message Class (2nd digit of MTI)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageClass {
    /// Reserved for ISO use (0xxx)
    Reserved = 0,
    /// Authorization (01xx, 02xx)
    Authorization = 1,
    /// Financial transactions (02xx)
    Financial = 2,
    /// File actions (03xx)
    FileActions = 3,
    /// Reversal/Chargeback (04xx)
    Reversal = 4,
    /// Reconciliation (05xx)
    Reconciliation = 5,
    /// Administrative (06xx)
    Administrative = 6,
    /// Fee collection (07xx)
    FeeCollection = 7,
    /// Network management (08xx)
    NetworkManagement = 8,
    /// Reserved for ISO use (09xx)
    ReservedISO = 9,
}

/// Message Function (3rd digit of MTI)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageFunction {
    /// Request (xx0x)
    Request = 0,
    /// Request response (xx1x)
    Response = 1,
    /// Advice (xx2x)
    Advice = 2,
    /// Advice response (xx3x)
    AdviceResponse = 3,
    /// Notification (xx4x)
    Notification = 4,
    /// Notification acknowledgement (xx5x)
    NotificationAck = 5,
    /// Instruction (xx6x)
    Instruction = 6,
    /// Instruction acknowledgement (xx7x)
    InstructionAck = 7,
    /// Reserved (xx8x)
    Reserved8 = 8,
    /// Reserved (xx9x)
    Reserved9 = 9,
}

/// Message Origin (4th digit of MTI)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageOrigin {
    /// Acquirer (xxx0)
    Acquirer = 0,
    /// Acquirer repeat (xxx1)
    AcquirerRepeat = 1,
    /// Issuer (xxx2)
    Issuer = 2,
    /// Issuer repeat (xxx3)
    IssuerRepeat = 3,
    /// Other (xxx4)
    Other = 4,
    /// Other repeat (xxx5)
    OtherRepeat = 5,
    /// Reserved (xxx6)
    Reserved6 = 6,
    /// Reserved (xxx7)
    Reserved7 = 7,
    /// Reserved (xxx8)
    Reserved8 = 8,
    /// Reserved (xxx9)
    Reserved9 = 9,
}

impl MessageType {
    /// Common message types as constants
    
    /// Authorization request (0100)
    pub const AUTHORIZATION_REQUEST: Self = Self {
        version: 0,
        class: MessageClass::Authorization,
        function: MessageFunction::Request,
        origin: MessageOrigin::Acquirer,
    };

    /// Authorization response (0110)
    pub const AUTHORIZATION_RESPONSE: Self = Self {
        version: 0,
        class: MessageClass::Authorization,
        function: MessageFunction::Response,
        origin: MessageOrigin::Acquirer,
    };

    /// Authorization advice (0120)
    pub const AUTHORIZATION_ADVICE: Self = Self {
        version: 0,
        class: MessageClass::Authorization,
        function: MessageFunction::Advice,
        origin: MessageOrigin::Acquirer,
    };

    /// Authorization advice response (0130)
    pub const AUTHORIZATION_ADVICE_RESPONSE: Self = Self {
        version: 0,
        class: MessageClass::Authorization,
        function: MessageFunction::AdviceResponse,
        origin: MessageOrigin::Acquirer,
    };

    /// Financial request (0200)
    pub const FINANCIAL_REQUEST: Self = Self {
        version: 0,
        class: MessageClass::Financial,
        function: MessageFunction::Request,
        origin: MessageOrigin::Acquirer,
    };

    /// Financial response (0210)
    pub const FINANCIAL_RESPONSE: Self = Self {
        version: 0,
        class: MessageClass::Financial,
        function: MessageFunction::Response,
        origin: MessageOrigin::Acquirer,
    };

    /// Financial advice (0220)
    pub const FINANCIAL_ADVICE: Self = Self {
        version: 0,
        class: MessageClass::Financial,
        function: MessageFunction::Advice,
        origin: MessageOrigin::Acquirer,
    };

    /// Financial advice response (0230)
    pub const FINANCIAL_ADVICE_RESPONSE: Self = Self {
        version: 0,
        class: MessageClass::Financial,
        function: MessageFunction::AdviceResponse,
        origin: MessageOrigin::Acquirer,
    };

    /// Reversal request (0400)
    pub const REVERSAL_REQUEST: Self = Self {
        version: 0,
        class: MessageClass::Reversal,
        function: MessageFunction::Request,
        origin: MessageOrigin::Acquirer,
    };

    /// Reversal response (0410)
    pub const REVERSAL_RESPONSE: Self = Self {
        version: 0,
        class: MessageClass::Reversal,
        function: MessageFunction::Response,
        origin: MessageOrigin::Acquirer,
    };

    /// Reversal advice (0420)
    pub const REVERSAL_ADVICE: Self = Self {
        version: 0,
        class: MessageClass::Reversal,
        function: MessageFunction::Advice,
        origin: MessageOrigin::Acquirer,
    };

    /// Reversal advice response (0430)
    pub const REVERSAL_ADVICE_RESPONSE: Self = Self {
        version: 0,
        class: MessageClass::Reversal,
        function: MessageFunction::AdviceResponse,
        origin: MessageOrigin::Acquirer,
    };

    /// Network management request (0800)
    pub const NETWORK_MANAGEMENT_REQUEST: Self = Self {
        version: 0,
        class: MessageClass::NetworkManagement,
        function: MessageFunction::Request,
        origin: MessageOrigin::Acquirer,
    };

    /// Network management response (0810)
    pub const NETWORK_MANAGEMENT_RESPONSE: Self = Self {
        version: 0,
        class: MessageClass::NetworkManagement,
        function: MessageFunction::Response,
        origin: MessageOrigin::Acquirer,
    };

    /// Network management advice (0820)
    pub const NETWORK_MANAGEMENT_ADVICE: Self = Self {
        version: 0,
        class: MessageClass::NetworkManagement,
        function: MessageFunction::Advice,
        origin: MessageOrigin::Acquirer,
    };

    /// Create a new MTI from components
    pub fn new(
        version: u8,
        class: MessageClass,
        function: MessageFunction,
        origin: MessageOrigin,
    ) -> Self {
        Self {
            version,
            class,
            function,
            origin,
        }
    }

    /// Parse MTI from 4-digit string
    pub fn from_str(s: &str) -> Result<Self> {
        if s.len() != 4 {
            return Err(ISO8583Error::InvalidMTI(format!(
                "MTI must be 4 digits, got {}",
                s.len()
            )));
        }

        let digits: Vec<u8> = s
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map(|d| d as u8)
                    .ok_or_else(|| ISO8583Error::InvalidMTI(format!("Invalid digit: {}", c)))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            version: digits[0],
            class: MessageClass::from_digit(digits[1])?,
            function: MessageFunction::from_digit(digits[2])?,
            origin: MessageOrigin::from_digit(digits[3])?,
        })
    }

    /// Parse MTI from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 4 {
            return Err(ISO8583Error::InvalidMTI(format!(
                "MTI must be at least 4 bytes, got {}",
                bytes.len()
            )));
        }

        let s = std::str::from_utf8(&bytes[..4])
            .map_err(|e| ISO8583Error::InvalidMTI(format!("Invalid UTF-8: {}", e)))?;

        Self::from_str(s)
    }

    /// Convert to 4-digit string
    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}{}",
            self.version,
            self.class.to_digit(),
            self.function.to_digit(),
            self.origin.to_digit()
        )
    }

    /// Convert to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    /// Check if this is a request message
    pub fn is_request(&self) -> bool {
        matches!(self.function, MessageFunction::Request)
    }

    /// Check if this is a response message
    pub fn is_response(&self) -> bool {
        matches!(self.function, MessageFunction::Response)
    }

    /// Check if this is an advice message
    pub fn is_advice(&self) -> bool {
        matches!(
            self.function,
            MessageFunction::Advice | MessageFunction::AdviceResponse
        )
    }

    /// Get the corresponding response MTI for a request
    pub fn to_response(&self) -> Result<Self> {
        if !self.is_request() {
            return Err(ISO8583Error::InvalidMTI(
                "Can only convert request to response".to_string(),
            ));
        }

        Ok(Self {
            version: self.version,
            class: self.class,
            function: MessageFunction::Response,
            origin: self.origin,
        })
    }
}

impl MessageClass {
    fn from_digit(digit: u8) -> Result<Self> {
        match digit {
            0 => Ok(Self::Reserved),
            1 => Ok(Self::Authorization),
            2 => Ok(Self::Financial),
            3 => Ok(Self::FileActions),
            4 => Ok(Self::Reversal),
            5 => Ok(Self::Reconciliation),
            6 => Ok(Self::Administrative),
            7 => Ok(Self::FeeCollection),
            8 => Ok(Self::NetworkManagement),
            9 => Ok(Self::ReservedISO),
            _ => Err(ISO8583Error::InvalidMessageClass(format!(
                "Invalid message class digit: {}",
                digit
            ))),
        }
    }

    fn to_digit(&self) -> u8 {
        *self as u8
    }
}

impl MessageFunction {
    fn from_digit(digit: u8) -> Result<Self> {
        match digit {
            0 => Ok(Self::Request),
            1 => Ok(Self::Response),
            2 => Ok(Self::Advice),
            3 => Ok(Self::AdviceResponse),
            4 => Ok(Self::Notification),
            5 => Ok(Self::NotificationAck),
            6 => Ok(Self::Instruction),
            7 => Ok(Self::InstructionAck),
            8 => Ok(Self::Reserved8),
            9 => Ok(Self::Reserved9),
            _ => Err(ISO8583Error::InvalidMessageFunction(format!(
                "Invalid message function digit: {}",
                digit
            ))),
        }
    }

    fn to_digit(&self) -> u8 {
        *self as u8
    }
}

impl MessageOrigin {
    fn from_digit(digit: u8) -> Result<Self> {
        match digit {
            0 => Ok(Self::Acquirer),
            1 => Ok(Self::AcquirerRepeat),
            2 => Ok(Self::Issuer),
            3 => Ok(Self::IssuerRepeat),
            4 => Ok(Self::Other),
            5 => Ok(Self::OtherRepeat),
            6 => Ok(Self::Reserved6),
            7 => Ok(Self::Reserved7),
            8 => Ok(Self::Reserved8),
            9 => Ok(Self::Reserved9),
            _ => Err(ISO8583Error::InvalidMessageOrigin(format!(
                "Invalid message origin digit: {}",
                digit
            ))),
        }
    }

    fn to_digit(&self) -> u8 {
        *self as u8
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mti_parsing() {
        let mti = MessageType::from_str("0100").unwrap();
        assert_eq!(mti.version, 0);
        assert_eq!(mti.class, MessageClass::Authorization);
        assert_eq!(mti.function, MessageFunction::Request);
        assert_eq!(mti.origin, MessageOrigin::Acquirer);
        assert_eq!(mti.to_string(), "0100");
    }

    #[test]
    fn test_mti_constants() {
        assert_eq!(MessageType::AUTHORIZATION_REQUEST.to_string(), "0100");
        assert_eq!(MessageType::AUTHORIZATION_RESPONSE.to_string(), "0110");
        assert_eq!(MessageType::FINANCIAL_REQUEST.to_string(), "0200");
        assert_eq!(MessageType::FINANCIAL_RESPONSE.to_string(), "0210");
        assert_eq!(MessageType::REVERSAL_REQUEST.to_string(), "0400");
        assert_eq!(MessageType::NETWORK_MANAGEMENT_REQUEST.to_string(), "0800");
    }

    #[test]
    fn test_mti_predicates() {
        let request = MessageType::AUTHORIZATION_REQUEST;
        assert!(request.is_request());
        assert!(!request.is_response());
        assert!(!request.is_advice());

        let response = MessageType::AUTHORIZATION_RESPONSE;
        assert!(!response.is_request());
        assert!(response.is_response());
        assert!(!response.is_advice());

        let advice = MessageType::AUTHORIZATION_ADVICE;
        assert!(!advice.is_request());
        assert!(!advice.is_response());
        assert!(advice.is_advice());
    }

    #[test]
    fn test_to_response() {
        let request = MessageType::AUTHORIZATION_REQUEST;
        let response = request.to_response().unwrap();
        assert_eq!(response, MessageType::AUTHORIZATION_RESPONSE);

        // Cannot convert response to response
        let err = response.to_response();
        assert!(err.is_err());
    }

    #[test]
    fn test_invalid_mti() {
        assert!(MessageType::from_str("123").is_err()); // Too short
        assert!(MessageType::from_str("12345").is_err()); // Too long
        assert!(MessageType::from_str("abcd").is_err()); // Invalid chars
    }
}
