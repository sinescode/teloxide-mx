//! Webhook IP filtering and security.
//!
//! Validates incoming webhook requests against Telegram's official IP ranges.
//!
//! # Example
//!
//! ```rust,no_run
//! # use teloxide::utils::webhook_security::TelegramIpFilter;
//! let filter = TelegramIpFilter::default();
//! assert!(filter.is_allowed("149.154.160.0"));
//! assert!(filter.is_allowed("91.108.4.0"));
//! ```

use std::net::IpAddr;

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
                self.allowed_networks
                    .iter()
                    .any(|&(base, mask)| (ip_u32 & mask) == (base & mask))
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
}
