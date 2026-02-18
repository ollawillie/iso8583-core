//! ISO 8583 Processing Codes
//!
//! Processing codes indicate the type of transaction and account types involved.
//! Format: TTFFTT where:
//! - TT (positions 1-2): Transaction type
//! - FF (positions 3-4): From account type
//! - TT (positions 5-6): To account type

use std::fmt;

/// Processing Code (6 digits)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProcessingCode {
    pub transaction_type: TransactionType,
    pub from_account: AccountType,
    pub to_account: AccountType,
}

/// Transaction Type (first 2 digits)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransactionType {
    /// Purchase (00)
    Purchase = 0,
    /// Cash withdrawal (01)
    CashWithdrawal = 1,
    /// Debit adjustment (02)
    DebitAdjustment = 2,
    /// Check guarantee (03)
    CheckGuarantee = 3,
    /// Check verification (04)
    CheckVerification = 4,
    /// Eurocheque (05)
    Eurocheque = 5,
    /// Travelers check (06)
    TravelersCheck = 6,
    /// Letter of credit (07)
    LetterOfCredit = 7,
    /// Giro (08)
    Giro = 8,
    /// Cash deposit (21)
    CashDeposit = 21,
    /// Check deposit (22)
    CheckDeposit = 22,
    /// Balance inquiry (31)
    BalanceInquiry = 31,
    /// Mini statement (38)
    MiniStatement = 38,
    /// Transfer from checking to savings (40)
    TransferCheckingToSavings = 40,
    /// Transfer from savings to checking (41)
    TransferSavingsToChecking = 41,
    /// Refund (20)
    Refund = 20,
    /// Payment (50)
    Payment = 50,
}

/// Account Type (positions 3-4 and 5-6)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccountType {
    /// Default/Unspecified (00)
    Default = 0,
    /// Savings account (10)
    Savings = 10,
    /// Checking account (20)
    Checking = 20,
    /// Credit account (30)
    Credit = 30,
    /// Universal account (40)
    Universal = 40,
    /// Investment account (50)
    Investment = 50,
}

impl ProcessingCode {
    /// Common processing codes as constants
    
    /// Purchase from default account (000000)
    pub const PURCHASE: Self = Self {
        transaction_type: TransactionType::Purchase,
        from_account: AccountType::Default,
        to_account: AccountType::Default,
    };

    /// Cash withdrawal from checking (010000)
    pub const WITHDRAWAL_CHECKING: Self = Self {
        transaction_type: TransactionType::CashWithdrawal,
        from_account: AccountType::Default,
        to_account: AccountType::Default,
    };

    /// Cash withdrawal from savings (011000)
    pub const WITHDRAWAL_SAVINGS: Self = Self {
        transaction_type: TransactionType::CashWithdrawal,
        from_account: AccountType::Savings,
        to_account: AccountType::Default,
    };

    /// Deposit to checking (210000)
    pub const DEPOSIT_CHECKING: Self = Self {
        transaction_type: TransactionType::CashDeposit,
        from_account: AccountType::Default,
        to_account: AccountType::Default,
    };

    /// Deposit to savings (211000)
    pub const DEPOSIT_SAVINGS: Self = Self {
        transaction_type: TransactionType::CashDeposit,
        from_account: AccountType::Savings,
        to_account: AccountType::Default,
    };

    /// Balance inquiry checking (310000)
    pub const BALANCE_INQUIRY_CHECKING: Self = Self {
        transaction_type: TransactionType::BalanceInquiry,
        from_account: AccountType::Default,
        to_account: AccountType::Default,
    };

    /// Balance inquiry savings (311000)
    pub const BALANCE_INQUIRY_SAVINGS: Self = Self {
        transaction_type: TransactionType::BalanceInquiry,
        from_account: AccountType::Savings,
        to_account: AccountType::Default,
    };

    /// Refund (200000)
    pub const REFUND: Self = Self {
        transaction_type: TransactionType::Refund,
        from_account: AccountType::Default,
        to_account: AccountType::Default,
    };

    /// Transfer checking to savings (401020)
    pub const TRANSFER_CHECKING_TO_SAVINGS: Self = Self {
        transaction_type: TransactionType::TransferCheckingToSavings,
        from_account: AccountType::Savings,
        to_account: AccountType::Checking,
    };

    /// Create new processing code
    pub fn new(
        transaction_type: TransactionType,
        from_account: AccountType,
        to_account: AccountType,
    ) -> Self {
        Self {
            transaction_type,
            from_account,
            to_account,
        }
    }

    /// Parse from 6-digit string
    pub fn from_str(s: &str) -> Option<Self> {
        if s.len() != 6 {
            return None;
        }

        let tt = s[0..2].parse::<u8>().ok()?;
        let from = s[2..4].parse::<u8>().ok()?;
        let to = s[4..6].parse::<u8>().ok()?;

        Some(Self {
            transaction_type: TransactionType::from_code(tt)?,
            from_account: AccountType::from_code(from)?,
            to_account: AccountType::from_code(to)?,
        })
    }

    /// Convert to 6-digit string
    pub fn to_string(&self) -> String {
        format!(
            "{:02}{:02}{:02}",
            self.transaction_type.to_code(),
            self.from_account.to_code(),
            self.to_account.to_code()
        )
    }

    /// Get transaction description
    pub fn description(&self) -> String {
        let txn_desc = match self.transaction_type {
            TransactionType::Purchase => "Purchase",
            TransactionType::CashWithdrawal => "Cash Withdrawal",
            TransactionType::CashDeposit => "Deposit",
            TransactionType::BalanceInquiry => "Balance Inquiry",
            TransactionType::Refund => "Refund",
            TransactionType::Payment => "Payment",
            TransactionType::TransferCheckingToSavings => "Transfer",
            TransactionType::TransferSavingsToChecking => "Transfer",
            _ => "Transaction",
        };

        let from_desc = match self.from_account {
            AccountType::Savings => " from Savings",
            AccountType::Checking => " from Checking",
            AccountType::Credit => " from Credit",
            AccountType::Default => "",
            _ => " from Account",
        };

        let to_desc = match self.to_account {
            AccountType::Savings => " to Savings",
            AccountType::Checking => " to Checking",
            AccountType::Credit => " to Credit",
            AccountType::Default => "",
            _ => " to Account",
        };

        format!("{}{}{}", txn_desc, from_desc, to_desc)
    }

    /// Check if this is a balance inquiry
    pub fn is_inquiry(&self) -> bool {
        matches!(
            self.transaction_type,
            TransactionType::BalanceInquiry | TransactionType::MiniStatement
        )
    }

    /// Check if this is a cash transaction
    pub fn is_cash(&self) -> bool {
        matches!(
            self.transaction_type,
            TransactionType::CashWithdrawal | TransactionType::CashDeposit
        )
    }

    /// Check if this is a transfer
    pub fn is_transfer(&self) -> bool {
        matches!(
            self.transaction_type,
            TransactionType::TransferCheckingToSavings
                | TransactionType::TransferSavingsToChecking
        )
    }
}

impl TransactionType {
    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            0 => Some(Self::Purchase),
            1 => Some(Self::CashWithdrawal),
            2 => Some(Self::DebitAdjustment),
            20 => Some(Self::Refund),
            21 => Some(Self::CashDeposit),
            22 => Some(Self::CheckDeposit),
            31 => Some(Self::BalanceInquiry),
            38 => Some(Self::MiniStatement),
            40 => Some(Self::TransferCheckingToSavings),
            41 => Some(Self::TransferSavingsToChecking),
            50 => Some(Self::Payment),
            _ => None,
        }
    }

    pub fn to_code(&self) -> u8 {
        match self {
            Self::Purchase => 0,
            Self::CashWithdrawal => 1,
            Self::DebitAdjustment => 2,
            Self::Refund => 20,
            Self::CashDeposit => 21,
            Self::CheckDeposit => 22,
            Self::BalanceInquiry => 31,
            Self::MiniStatement => 38,
            Self::TransferCheckingToSavings => 40,
            Self::TransferSavingsToChecking => 41,
            Self::Payment => 50,
            _ => 0,
        }
    }
}

impl AccountType {
    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            0 => Some(Self::Default),
            10 => Some(Self::Savings),
            20 => Some(Self::Checking),
            30 => Some(Self::Credit),
            40 => Some(Self::Universal),
            50 => Some(Self::Investment),
            _ => Some(Self::Default), // Default for unrecognized codes
        }
    }

    pub fn to_code(&self) -> u8 {
        *self as u8
    }
}

impl fmt::Display for ProcessingCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processing_codes() {
        assert_eq!(ProcessingCode::PURCHASE.to_string(), "000000");
        assert_eq!(ProcessingCode::WITHDRAWAL_CHECKING.to_string(), "010000");
        assert_eq!(ProcessingCode::BALANCE_INQUIRY_SAVINGS.to_string(), "311000");
    }

    #[test]
    fn test_from_string() {
        let code = ProcessingCode::from_str("000000").unwrap();
        assert_eq!(code, ProcessingCode::PURCHASE);

        let code = ProcessingCode::from_str("010000").unwrap();
        assert_eq!(code, ProcessingCode::WITHDRAWAL_CHECKING);
    }

    #[test]
    fn test_descriptions() {
        assert_eq!(ProcessingCode::PURCHASE.description(), "Purchase");
        assert_eq!(
            ProcessingCode::WITHDRAWAL_SAVINGS.description(),
            "Cash Withdrawal from Savings"
        );
        assert_eq!(
            ProcessingCode::BALANCE_INQUIRY_CHECKING.description(),
            "Balance Inquiry"
        );
    }

    #[test]
    fn test_predicates() {
        assert!(ProcessingCode::BALANCE_INQUIRY_CHECKING.is_inquiry());
        assert!(ProcessingCode::WITHDRAWAL_CHECKING.is_cash());
        assert!(!ProcessingCode::PURCHASE.is_cash());
    }
}
