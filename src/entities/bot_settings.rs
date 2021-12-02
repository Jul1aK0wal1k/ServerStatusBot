use crate::entities::Address;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BotSettings {
    pub hosts: Vec<Address>,
}

impl BotSettings {
    pub fn new(hosts: &Vec<Address>) -> Self {
        BotSettings {
            hosts: hosts.to_vec(),
        }
    }
}
