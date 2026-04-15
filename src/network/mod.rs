//! Network configuration and backend selection.

pub mod addressing;
/// Backend selection and serialization helpers.
pub mod backend;
/// Launch-time backend planning and request validation rules.
pub mod launch;
pub mod policy;
pub mod virtio;

pub use backend::NetworkBackend;
pub use launch::{
    plan_launch_network, validate_requested_network_backend, EffectiveNetworkBackend,
    LaunchNetworkPlan, NetworkFallbackReason,
};
pub use policy::get_dns_server;
