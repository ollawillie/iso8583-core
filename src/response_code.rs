//! ISO 8583 Response Codes
//!
//! Standard response codes used in authorization and financial responses.
//! These indicate the outcome of a transaction request.

use std::fmt;

/// ISO 8583 Response Code
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResponseCode(pub u8, pub u8);

#[allow(missing_docs)]
impl ResponseCode {
    // Approval codes
    pub const APPROVED: Self = Self(0, 0);
    pub const APPROVED_WITH_ID: Self = Self(0, 1);
    pub const APPROVED_PARTIAL: Self = Self(0, 2);

    // Referral codes
    pub const REFER_TO_ISSUER: Self = Self(0, 1);
    pub const REFER_SPECIAL: Self = Self(0, 2);
    pub const INVALID_MERCHANT: Self = Self(0, 3);
    pub const PICK_UP_CARD: Self = Self(0, 4);

    // Decline codes
    pub const DO_NOT_HONOR: Self = Self(0, 5);
    pub const ERROR: Self = Self(0, 6);
    pub const PICK_UP_SPECIAL: Self = Self(0, 7);
    pub const HONOR_WITH_ID: Self = Self(0, 8);

    // Format/validity errors
    pub const INVALID_TRANSACTION: Self = Self(1, 2);
    pub const INVALID_AMOUNT: Self = Self(1, 3);
    pub const INVALID_CARD_NUMBER: Self = Self(1, 4);
    pub const NO_SUCH_ISSUER: Self = Self(1, 5);

    // Card/Account issues
    pub const CUSTOMER_CANCELLATION: Self = Self(1, 7);
    pub const DUPLICATE_TRANSACTION: Self = Self(1, 8);
    pub const RE_ENTER_TRANSACTION: Self = Self(1, 9);
    pub const INVALID_RESPONSE: Self = Self(2, 0);
    pub const NO_ACTION_TAKEN: Self = Self(2, 1);
    pub const SUSPECTED_MALFUNCTION: Self = Self(2, 2);
    pub const UNACCEPTABLE_TRANSACTION_FEE: Self = Self(2, 3);
    pub const FILE_UPDATE_NOT_SUPPORTED: Self = Self(2, 4);
    pub const UNABLE_TO_LOCATE_RECORD: Self = Self(2, 5);
    pub const DUPLICATE_RECORD: Self = Self(2, 6);
    pub const FILE_UPDATE_EDIT_ERROR: Self = Self(2, 7);
    pub const FILE_UPDATE_FILE_LOCKED: Self = Self(2, 8);
    pub const FILE_UPDATE_FAILED: Self = Self(2, 9);
    pub const FORMAT_ERROR: Self = Self(3, 0);

    // Security/Authorization issues
    pub const BANK_NOT_SUPPORTED: Self = Self(3, 1);
    pub const COMPLETED_PARTIALLY: Self = Self(3, 2);
    pub const EXPIRED_CARD_PICKUP: Self = Self(3, 3);
    pub const SUSPECTED_FRAUD: Self = Self(3, 4);
    pub const RESTRICTED_CARD: Self = Self(3, 6);
    pub const CONTACT_ACQUIRER_SECURITY: Self = Self(3, 7);
    pub const LOST_CARD: Self = Self(4, 1);
    pub const STOLEN_CARD: Self = Self(4, 3);

    // Insufficient funds/limits
    pub const INSUFFICIENT_FUNDS: Self = Self(5, 1);
    pub const NO_CHECKING_ACCOUNT: Self = Self(5, 2);
    pub const NO_SAVINGS_ACCOUNT: Self = Self(5, 3);
    pub const EXPIRED_CARD: Self = Self(5, 4);
    pub const INCORRECT_PIN: Self = Self(5, 5);
    pub const NO_CARD_RECORD: Self = Self(5, 6);
    pub const TRANSACTION_NOT_PERMITTED: Self = Self(5, 7);
    pub const TRANSACTION_NOT_PERMITTED_TERMINAL: Self = Self(5, 8);
    pub const SUSPECTED_FRAUD_DECLINE: Self = Self(5, 9);
    pub const CONTACT_ACQUIRER: Self = Self(6, 0);
    pub const EXCEEDS_WITHDRAWAL_LIMIT: Self = Self(6, 1);
    pub const RESTRICTED_CARD_DECLINE: Self = Self(6, 2);
    pub const SECURITY_VIOLATION: Self = Self(6, 3);
    pub const EXCEEDS_WITHDRAWAL_FREQUENCY: Self = Self(6, 5);

    // PIN issues
    pub const PIN_REQUIRED: Self = Self(7, 5);
    pub const PIN_VALIDATION_NOT_POSSIBLE: Self = Self(7, 6);
    pub const PIN_TRIES_EXCEEDED: Self = Self(7, 7);

    // System/Network issues
    pub const CRYPTOGRAPHIC_FAILURE: Self = Self(8, 0);
    pub const CRYPTOGRAPHIC_KEY_SYNC_ERROR: Self = Self(8, 1);
    pub const CVV_FAILURE: Self = Self(8, 2);
    pub const CANT_VERIFY_PIN: Self = Self(8, 3);
    pub const MESSAGE_FLOW_ERROR: Self = Self(8, 5);
    pub const CUTOVER_IN_PROGRESS: Self = Self(9, 0);
    pub const ISSUER_UNAVAILABLE: Self = Self(9, 1);
    pub const ROUTING_ERROR: Self = Self(9, 2);
    pub const DUPLICATE_TRANSMISSION: Self = Self(9, 4);
    pub const RECONCILE_ERROR: Self = Self(9, 5);
    pub const SYSTEM_MALFUNCTION: Self = Self(9, 6);

    // Special conditions
    pub const MAC_ERROR: Self = Self(9, 7);
    pub const FAILED_SECURITY_CHECK: Self = Self(9, 8);

    /// Create from two digits
    pub fn new(first: u8, second: u8) -> Self {
        Self(first, second)
    }

    /// Get human-readable description
    pub fn description(&self) -> &'static str {
        match (self.0, self.1) {
            (0, 0) => "Approved or completed successfully",
            (0, 1) => "Refer to card issuer",
            (0, 2) => "Refer to card issuer, special condition",
            (0, 3) => "Invalid merchant",
            (0, 4) => "Pick up card",
            (0, 5) => "Do not honor",
            (0, 6) => "Error",
            (0, 7) => "Pick up card, special condition",
            (1, 2) => "Invalid transaction",
            (1, 3) => "Invalid amount",
            (1, 4) => "Invalid card number",
            (1, 5) => "No such issuer",
            (3, 0) => "Format error",
            (4, 1) => "Lost card, pick up",
            (4, 3) => "Stolen card, pick up",
            (5, 1) => "Insufficient funds",
            (5, 4) => "Expired card",
            (5, 5) => "Incorrect PIN",
            (5, 7) => "Transaction not permitted to cardholder",
            (5, 8) => "Transaction not permitted to terminal",
            (6, 1) => "Exceeds withdrawal amount limit",
            (7, 5) => "PIN required",
            (7, 7) => "PIN tries exceeded",
            (9, 1) => "Issuer or switch inoperative",
            (9, 6) => "System malfunction",
            _ => "Unknown response code",
        }
    }

    /// Check if the response indicates approval
    pub fn is_approved(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }

    /// Check if response indicates a decline
    pub fn is_declined(&self) -> bool {
        !self.is_approved() && !self.is_referral() && !self.is_system_error()
    }

    /// Check if response indicates referral to issuer
    pub fn is_referral(&self) -> bool {
        matches!((self.0, self.1), (0, 1) | (0, 2))
    }

    /// Check if response indicates a system/network error
    pub fn is_system_error(&self) -> bool {
        matches!(self.0, 9 | 8) || matches!((self.0, self.1), (3, 0) | (0, 6))
    }

    /// Check if response indicates card should be retained
    pub fn should_retain_card(&self) -> bool {
        matches!((self.0, self.1), (0, 4) | (0, 7) | (4, 1) | (4, 3))
    }

    /// Get response category
    pub fn category(&self) -> ResponseCategory {
        match (self.0, self.1) {
            (0, 0..=2) => ResponseCategory::Approved,
            (0, 1..=4) | (0, 7) => ResponseCategory::Referral,
            (4, 1) | (4, 3) => ResponseCategory::CardRetention,
            (5, 1) | (6, 1) | (6, 5) => ResponseCategory::InsufficientFunds,
            (5, 4) => ResponseCategory::ExpiredCard,
            (5, 5) | (7, 5) | (7, 7) => ResponseCategory::PINError,
            (9, _) | (8, _) => ResponseCategory::SystemError,
            _ => ResponseCategory::Declined,
        }
    }
}

impl std::str::FromStr for ResponseCode {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(());
        }

        let first = s.chars().nth(0).ok_or(())?.to_digit(10).ok_or(())? as u8;
        let second = s.chars().nth(1).ok_or(())?.to_digit(10).ok_or(())? as u8;

        Ok(Self(first, second))
    }
}

/// Response code category
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseCategory {
    Approved,
    Declined,
    Referral,
    CardRetention,
    InsufficientFunds,
    ExpiredCard,
    PINError,
    SystemError,
}

impl fmt::Display for ResponseCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}", self.0 * 10 + self.1)
    }
}

impl fmt::Display for ResponseCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Approved => write!(f, "Approved"),
            Self::Declined => write!(f, "Declined"),
            Self::Referral => write!(f, "Referral"),
            Self::CardRetention => write!(f, "Card Retention"),
            Self::InsufficientFunds => write!(f, "Insufficient Funds"),
            Self::ExpiredCard => write!(f, "Expired Card"),
            Self::PINError => write!(f, "PIN Error"),
            Self::SystemError => write!(f, "System Error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_codes() {
        assert!(ResponseCode::APPROVED.is_approved());
        assert!(!ResponseCode::DO_NOT_HONOR.is_approved());
        assert!(ResponseCode::INSUFFICIENT_FUNDS.is_declined());
        assert!(ResponseCode::REFER_TO_ISSUER.is_referral());
        assert!(ResponseCode::ISSUER_UNAVAILABLE.is_system_error());
    }

    #[test]
    fn test_card_retention() {
        assert!(ResponseCode::LOST_CARD.should_retain_card());
        assert!(ResponseCode::STOLEN_CARD.should_retain_card());
        assert!(!ResponseCode::INSUFFICIENT_FUNDS.should_retain_card());
    }

    #[test]
    fn test_response_categories() {
        assert_eq!(
            ResponseCode::APPROVED.category(),
            ResponseCategory::Approved
        );
        assert_eq!(
            ResponseCode::INSUFFICIENT_FUNDS.category(),
            ResponseCategory::InsufficientFunds
        );
        assert_eq!(
            ResponseCode::INCORRECT_PIN.category(),
            ResponseCategory::PINError
        );
    }

    #[test]
    fn test_from_string() {
        let code = "00".parse::<ResponseCode>().unwrap();
        assert_eq!(code, ResponseCode::APPROVED);

        let code = "51".parse::<ResponseCode>().unwrap();
        assert_eq!(code, ResponseCode::INSUFFICIENT_FUNDS);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(ResponseCode::APPROVED.to_string(), "00");
        assert_eq!(ResponseCode::INSUFFICIENT_FUNDS.to_string(), "51");
    }
}
