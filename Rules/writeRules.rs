use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct rules {
    name: String,
    action: String,       // "allow", "deny", "log"
    direction: String,    // "inbound", "outbound"
    protocol: String,     // "tcp", "udp", "icmp"
    src_ip: Option<String>,  // Opcional (pode ser null no JSON)
    dst_ip: Option<String>,
    dst_port: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirewallConfig {
    rules: Vec<rules>,
}