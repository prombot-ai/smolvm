//! Addressing helpers for the virtio-net backend.
//!
//! Context
//! =======
//!
//! The Phase 1 virtio-net path does not use DHCP, SLAAC, or any dynamic address
//! negotiation. Instead, the host runtime and the guest agent agree on one
//! small static link layout:
//!
//! ```text
//! guest eth0                 host-side smolvm gateway
//!   MAC 02:53:4d:00:00:02      MAC 02:53:4d:00:00:01
//!   IP  100.96.0.2/30          IP  100.96.0.1/30
//!             \                /
//!              \__ virtual L2 _/

use std::net::Ipv4Addr;

/// Static guest network configuration for the virtio-net MVP.
///
/// This struct describes the two endpoints of the single virtual Ethernet link:
/// - the guest NIC (`guest_*`)
/// - the host-side gateway implemented by smolvm (`gateway_*`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuestNetworkConfig {
    /// Guest IPv4 address.
    pub guest_ip: Ipv4Addr,
    /// Gateway IPv4 address.
    pub gateway_ip: Ipv4Addr,
    /// Prefix length.
    pub prefix_len: u8,
    /// Guest MAC address.
    pub guest_mac: [u8; 6],
    /// Gateway MAC address.
    pub gateway_mac: [u8; 6],
    /// DNS server address presented to the guest.
    pub dns_server: Ipv4Addr,
}

impl GuestNetworkConfig {
    /// Default Phase 1 guest network configuration.
    ///
    /// The values are chosen to give the guest a tiny point-to-point style
    /// subnet with a single host-side gateway:
    ///
    /// - `100.96.0.1/30`: gateway
    /// - `100.96.0.2/30`: guest
    ///
    /// A `/30` gives exactly two usable host addresses, which fits the MVP:
    /// one address for the gateway and one for the guest. The MAC addresses use
    /// locally administered unicast values with a fixed `02:53:4d` prefix so
    /// the host and guest endpoints stay deterministic across boots.
    pub const fn default() -> Self {
        Self {
            guest_ip: Ipv4Addr::new(100, 96, 0, 2),
            gateway_ip: Ipv4Addr::new(100, 96, 0, 1),
            prefix_len: 30,
            guest_mac: [0x02, 0x53, 0x4d, 0x00, 0x00, 0x02],
            gateway_mac: [0x02, 0x53, 0x4d, 0x00, 0x00, 0x01],
            dns_server: Ipv4Addr::new(100, 96, 0, 1),
        }
    }
}
