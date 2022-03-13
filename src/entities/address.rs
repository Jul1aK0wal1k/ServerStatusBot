use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

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

pub enum ParseAddressError {
    InvalidFormat,
    InvalidPort,
}

impl FromStr for Address {
    type Err = ParseAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split(':').collect();
        match splitted.len() {
            1 => {
                let host = splitted[0].to_owned();
                Ok(Self::new(host, None))
            }
            2 => {
                let host = splitted[0].to_owned();
                if let Ok(port) = u16::from_str(splitted[1]) {
                    Ok(Self::new(host, Some(port)))
                } else {
                    Err(ParseAddressError::InvalidPort)
                }
            }
            _ => Err(ParseAddressError::InvalidFormat),
        }
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
