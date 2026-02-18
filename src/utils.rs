//! Utility functions for common ISO 8583 operations

use crate::error::{ISO8583Error, Result};
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};

/// Mask PAN for display (shows first 6 and last 4 digits)
///
/// # Example
/// ```
/// use rust_iso8583::utils::mask_pan;
///
/// assert_eq!(mask_pan("4111111111111111"), "411111****1111");
/// assert_eq!(mask_pan("5500000000000004"), "550000****0004");
/// ```
pub fn mask_pan(pan: &str) -> String {
    if pan.len() < 10 {
        return "*".repeat(pan.len());
    }
    let first = &pan[..6];
    let last = &pan[pan.len() - 4..];
    format!("{}****{}", first, last)
}

/// Format amount from minor units (cents/kobo) to major units with currency symbol
///
/// # Example
/// ```
/// use rust_iso8583::utils::format_amount;
///
/// assert_eq!(format_amount("000000010000", "$"), "$100.00");
/// assert_eq!(format_amount("000000020050", "₦"), "₦200.50");
/// ```
pub fn format_amount(amount_str: &str, currency_symbol: &str) -> String {
    let amount: i64 = amount_str.parse().unwrap_or(0);
    format!("{}{:.2}", currency_symbol, amount as f64 / 100.0)
}

/// Parse amount from decimal to minor units
///
/// # Example
/// ```
/// use rust_iso8583::utils::parse_amount;
///
/// assert_eq!(parse_amount(100.50), "000000010050");
/// assert_eq!(parse_amount(1234.56), "000000123456");
/// ```
pub fn parse_amount(amount: f64) -> String {
    let minor_units = (amount * 100.0).round() as i64;
    format!("{:012}", minor_units)
}

/// Generate transmission date/time (Field 7) - MMDDhhmmss
///
/// # Example
/// ```
/// use rust_iso8583::utils::generate_transmission_datetime;
///
/// let dt = generate_transmission_datetime();
/// assert_eq!(dt.len(), 10);
/// ```
pub fn generate_transmission_datetime() -> String {
    let now = Utc::now();
    now.format("%m%d%H%M%S").to_string()
}

/// Generate local transaction time (Field 12) - hhmmss
pub fn generate_local_time() -> String {
    let now = Utc::now();
    now.format("%H%M%S").to_string()
}

/// Generate local transaction date (Field 13) - MMDD
pub fn generate_local_date() -> String {
    let now = Utc::now();
    now.format("%m%d").to_string()
}

/// Parse transmission date/time (Field 7) - MMDDhhmmss
pub fn parse_transmission_datetime(s: &str) -> Result<(u32, u32, u32, u32, u32)> {
    if s.len() != 10 {
        return Err(ISO8583Error::invalid_datetime(
            7,
            "Must be 10 digits (MMDDhhmmss)",
        ));
    }

    let month: u32 = s[0..2]
        .parse()
        .map_err(|_| ISO8583Error::invalid_datetime(7, "Invalid month"))?;
    let day: u32 = s[2..4]
        .parse()
        .map_err(|_| ISO8583Error::invalid_datetime(7, "Invalid day"))?;
    let hour: u32 = s[4..6]
        .parse()
        .map_err(|_| ISO8583Error::invalid_datetime(7, "Invalid hour"))?;
    let minute: u32 = s[6..8]
        .parse()
        .map_err(|_| ISO8583Error::invalid_datetime(7, "Invalid minute"))?;
    let second: u32 = s[8..10]
        .parse()
        .map_err(|_| ISO8583Error::invalid_datetime(7, "Invalid second"))?;

    if month < 1 || month > 12 {
        return Err(ISO8583Error::invalid_datetime(7, "Month out of range"));
    }
    if day < 1 || day > 31 {
        return Err(ISO8583Error::invalid_datetime(7, "Day out of range"));
    }
    if hour >= 24 {
        return Err(ISO8583Error::invalid_datetime(7, "Hour out of range"));
    }
    if minute >= 60 {
        return Err(ISO8583Error::invalid_datetime(7, "Minute out of range"));
    }
    if second >= 60 {
        return Err(ISO8583Error::invalid_datetime(7, "Second out of range"));
    }

    Ok((month, day, hour, minute, second))
}

/// Format expiration date (Field 14) - YYMM
pub fn format_expiration_date(year: u32, month: u32) -> String {
    format!("{:02}{:02}", year % 100, month)
}

/// Parse expiration date (Field 14) - YYMM
pub fn parse_expiration_date(s: &str) -> Result<(u32, u32)> {
    if s.len() != 4 {
        return Err(ISO8583Error::invalid_datetime(
            14,
            "Must be 4 digits (YYMM)",
        ));
    }

    let year: u32 = s[0..2]
        .parse()
        .map_err(|_| ISO8583Error::invalid_datetime(14, "Invalid year"))?;
    let month: u32 = s[2..4]
        .parse()
        .map_err(|_| ISO8583Error::invalid_datetime(14, "Invalid month"))?;

    if month < 1 || month > 12 {
        return Err(ISO8583Error::invalid_datetime(14, "Month out of range"));
    }

    Ok((year, month))
}

/// Generate System Trace Audit Number (Field 11)
/// In production, this should be a monotonically increasing counter
pub fn generate_stan() -> String {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(1);

    let value = COUNTER.fetch_add(1, Ordering::SeqCst) % 1_000_000;
    format!("{:06}", value)
}

/// Generate Retrieval Reference Number (Field 37)
/// Format: YYMMDD + 6-digit sequence
pub fn generate_rrn() -> String {
    let now = Utc::now();
    let date_part = now.format("%y%m%d").to_string();
    let sequence = generate_stan();
    format!("{}{}", date_part, sequence)
}

/// Convert currency code to symbol
pub fn currency_symbol(iso_code: &str) -> &str {
    match iso_code {
        "840" => "$", // USD
        "566" => "₦", // NGN
        "978" => "€", // EUR
        "826" => "£", // GBP
        "392" => "¥", // JPY
        _ => "",
    }
}

/// Get currency name from ISO 4217 code
pub fn currency_name(iso_code: &str) -> &str {
    match iso_code {
        "840" => "US Dollar",
        "566" => "Nigerian Naira",
        "978" => "Euro",
        "826" => "British Pound",
        "392" => "Japanese Yen",
        "356" => "Indian Rupee",
        "710" => "South African Rand",
        _ => "Unknown Currency",
    }
}

/// Validate Track 2 data format
pub fn validate_track2(track2: &str) -> bool {
    // Track 2 format: PAN=YYMM[service_code][discretionary_data]
    if !track2.contains('=') {
        return false;
    }

    let parts: Vec<&str> = track2.split('=').collect();
    if parts.len() != 2 {
        return false;
    }

    // PAN should be 13-19 digits
    let pan = parts[0];
    if pan.len() < 13 || pan.len() > 19 {
        return false;
    }

    // Expiration should be at least 4 digits
    let exp_and_more = parts[1];
    if exp_and_more.len() < 4 {
        return false;
    }

    true
}

/// Generate random authorization ID (Field 38)
pub fn generate_auth_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    format!("{:06X}", timestamp % 16_777_216) // 6 hex digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_pan() {
        assert_eq!(mask_pan("4111111111111111"), "411111****1111");
        assert_eq!(mask_pan("5500000000000004"), "550000****0004");
        assert_eq!(mask_pan("123"), "***"); // Too short
    }

    #[test]
    fn test_format_amount() {
        assert_eq!(format_amount("000000010000", "$"), "$100.00");
        assert_eq!(format_amount("000000020050", "₦"), "₦200.50");
        assert_eq!(format_amount("000000000001", "$"), "$0.01");
    }

    #[test]
    fn test_parse_amount() {
        assert_eq!(parse_amount(100.00), "000000010000");
        assert_eq!(parse_amount(200.50), "000000020050");
        assert_eq!(parse_amount(0.01), "000000000001");
    }

    #[test]
    fn test_datetime_generation() {
        let dt = generate_transmission_datetime();
        assert_eq!(dt.len(), 10);

        let time = generate_local_time();
        assert_eq!(time.len(), 6);

        let date = generate_local_date();
        assert_eq!(date.len(), 4);
    }

    #[test]
    fn test_parse_transmission_datetime() {
        let result = parse_transmission_datetime("0115120530");
        assert!(result.is_ok());
        let (month, day, hour, minute, second) = result.unwrap();
        assert_eq!(month, 1);
        assert_eq!(day, 15);
        assert_eq!(hour, 12);
        assert_eq!(minute, 5);
        assert_eq!(second, 30);
    }

    #[test]
    fn test_expiration_date() {
        assert_eq!(format_expiration_date(2025, 12), "2512");
        
        let result = parse_expiration_date("2512");
        assert!(result.is_ok());
        let (year, month) = result.unwrap();
        assert_eq!(year, 25);
        assert_eq!(month, 12);
    }

    #[test]
    fn test_stan_generation() {
        let stan1 = generate_stan();
        let stan2 = generate_stan();
        assert_eq!(stan1.len(), 6);
        assert_eq!(stan2.len(), 6);
        assert_ne!(stan1, stan2); // Should be different
    }

    #[test]
    fn test_currency_functions() {
        assert_eq!(currency_symbol("840"), "$");
        assert_eq!(currency_symbol("566"), "₦");
        assert_eq!(currency_name("840"), "US Dollar");
        assert_eq!(currency_name("566"), "Nigerian Naira");
    }

    #[test]
    fn test_track2_validation() {
        assert!(validate_track2("4111111111111111=25121011234567890"));
        assert!(validate_track2("5500000000000004=2512"));
        assert!(!validate_track2("4111111111111111")); // No =
        assert!(!validate_track2("123=25")); // PAN too short
    }

    #[test]
    fn test_auth_id_generation() {
        let auth_id = generate_auth_id();
        assert_eq!(auth_id.len(), 6);
        // Should be hex digits
        assert!(auth_id.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
