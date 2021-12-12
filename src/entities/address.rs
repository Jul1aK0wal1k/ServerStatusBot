use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Address {
    pub host: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
}

impl Address {
    pub fn new(host: String, port: Option<u16>) -> Self {
        Address { host, port }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(port) = self.port {
            write!(f, "{}:{}", self.host, port)
        } else {
            write!(f, "{}", self.host)
        }
    }
}
