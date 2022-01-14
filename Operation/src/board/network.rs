use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct SerialTemplate {
    pub baud_rate: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EthernetTemplate {
    ip_address: String,
    port: u32,
}
