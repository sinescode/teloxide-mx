//! Webhook IP filtering and security.
//!
//! Validates incoming webhook requests against Telegram's official IP ranges
//! and provides constant-time secret token comparison to prevent timing attacks.
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide_max::utils::webhook_security::{TelegramIpFilter, constant_time_eq};
//! let filter = TelegramIpFilter::default();
//! assert!(filter.is_allowed("149.154.160.0"));
//! assert!(filter.is_allowed("91.108.4.0"));
//!
//! // Constant-time comparison (prevents timing attacks)
//! assert!(constant_time_eq(b"secret", b"secret"));
//! assert!(!constant_time_eq(b"secret", b"other"));
//! ```

use std::net::IpAddr;

/// Performs constant-time comparison of two byte slices.
///
/// This prevents timing attacks where an attacker could determine the correct
/// secret token by measuring response times for different guesses.
///
/// Returns `false` if lengths differ (length is not constant-time).
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

/// Extracts client IP from common proxy headers.
///
/// Checks `X-Forwarded-For`, `X-Real-IP`, and `CF-Connecting-IP` headers
/// in order. Returns the first valid IP found.
pub fn extract_client_ip(headers: &std::collections::HashMap<String, String>) -> Option<String> {
    // X-Forwarded-For: client, proxy1, proxy2
    if let Some(forwarded) = headers.get("x-forwarded-for") {
        if let Some(first) = forwarded.split(',').next() {
            let ip = first.trim();
            if ip.parse::<IpAddr>().is_ok() {
                return Some(ip.to_string());
            }
        }
    }
    // X-Real-IP: single client IP
    if let Some(real_ip) = headers.get("x-real-ip") {
        if real_ip.parse::<IpAddr>().is_ok() {
            return Some(real_ip.clone());
        }
    }
    // CF-Connecting-IP: Cloudflare
    if let Some(cf_ip) = headers.get("cf-connecting-ip") {
        if cf_ip.parse::<IpAddr>().is_ok() {
            return Some(cf_ip.clone());
        }
    }
    None
}

/// Default Telegram server IP ranges (IPv4).
const TELEGRAM_IP_RANGES: &[&str] = &[
    "149.154.160.0/20",
    "91.108.4.0/22",
    "91.108.8.0/22",
    "91.108.12.0/22",
    "91.108.16.0/22",
    "91.108.20.0/22",
    "91.108.56.0/22",
];

/// A filter for validating Telegram webhook IP addresses.
pub struct TelegramIpFilter {
    allowed_networks: Vec<(u32, u32)>,
}

impl TelegramIpFilter {
    /// Creates a filter with Telegram's default IP ranges.
    pub fn new() -> Self {
        let mut allowed_networks = Vec::new();
        for range in TELEGRAM_IP_RANGES {
            if let Some((base, mask)) = parse_cidr(range) {
                allowed_networks.push((base, mask));
            }
        }
        Self { allowed_networks }
    }

    /// Returns `true` if the IP address is from Telegram's servers.
    pub fn is_allowed(&self, ip: &str) -> bool {
        let addr: IpAddr = match ip.parse() {
            Ok(a) => a,
            Err(_) => return false,
        };
        match addr {
            IpAddr::V4(v4) => {
                let ip_u32 = u32::from_be_bytes(v4.octets());
                self.allowed_networks.iter().any(|&(base, mask)| (ip_u32 & mask) == (base & mask))
            }
            IpAddr::V6(_) => false, // Telegram uses IPv4 for webhooks
        }
    }
}

impl Default for TelegramIpFilter {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_cidr(cidr: &str) -> Option<(u32, u32)> {
    let mut parts = cidr.split('/');
    let ip = parts.next()?;
    let prefix_len: u32 = parts.next()?.parse().ok()?;

    let addr: IpAddr = ip.parse().ok()?;
    match addr {
        IpAddr::V4(v4) => {
            let ip_u32 = u32::from_be_bytes(v4.octets());
            let mask = !0u32 << (32 - prefix_len);
            Some((ip_u32, mask))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn telegram_ips_allowed() {
        let filter = TelegramIpFilter::new();
        assert!(filter.is_allowed("149.154.160.1"));
        assert!(filter.is_allowed("91.108.4.1"));
    }

    #[test]
    fn random_ip_not_allowed() {
        let filter = TelegramIpFilter::new();
        assert!(!filter.is_allowed("1.2.3.4"));
    }

    #[test]
    fn invalid_ip_not_allowed() {
        let filter = TelegramIpFilter::new();
        assert!(!filter.is_allowed("not_an_ip"));
    }

    #[test]
    fn constant_time_eq_same() {
        assert!(constant_time_eq(b"secret", b"secret"));
    }

    #[test]
    fn constant_time_eq_different() {
        assert!(!constant_time_eq(b"secret", b"other"));
    }

    #[test]
    fn constant_time_eq_different_length() {
        assert!(!constant_time_eq(b"short", b"longer"));
    }

    #[test]
    fn constant_time_eq_empty() {
        assert!(constant_time_eq(b"", b""));
    }

    #[test]
    fn extract_client_ip_forwarded_for() {
        let mut headers = std::collections::HashMap::new();
        headers.insert("x-forwarded-for".to_string(), "1.2.3.4, 5.6.7.8".to_string());
        assert_eq!(extract_client_ip(&headers), Some("1.2.3.4".to_string()));
    }

    #[test]
    fn extract_client_ip_real_ip() {
        let mut headers = std::collections::HashMap::new();
        headers.insert("x-real-ip".to_string(), "1.2.3.4".to_string());
        assert_eq!(extract_client_ip(&headers), Some("1.2.3.4".to_string()));
    }

    #[test]
    fn extract_client_ip_cf() {
        let mut headers = std::collections::HashMap::new();
        headers.insert("cf-connecting-ip".to_string(), "1.2.3.4".to_string());
        assert_eq!(extract_client_ip(&headers), Some("1.2.3.4".to_string()));
    }

    #[test]
    fn extract_client_ip_none() {
        let headers = std::collections::HashMap::new();
        assert_eq!(extract_client_ip(&headers), None);
    }
}
