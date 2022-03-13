use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use std::{fmt, str::FromStr};

fn default_option<T>() -> Option<T> {
    None
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Address {
    pub host: String,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default = "default_option")]
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

#[cfg(test)]
mod tests {
    use super::Address;
    use mongodb::bson;
    use rstest::rstest;

    #[rstest]
    #[case("abc", None)]
    #[case("foo", Some(8081))]
    #[case("", Some(8081))]
    fn serde_bson_address_test(#[case] host: &str, #[case] port: Option<u16>) {
        let address = Address::new(host.to_string(), port);
        let result_ser = bson::to_document(&address);
        assert!(result_ser.is_ok());
        let result_de = bson::from_document::<Address>(result_ser.unwrap());
        assert!(result_de.is_ok())
    }
}
