use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BotSettings {
    pub max_hosts: i8,
}

impl BotSettings {
    pub fn new(max_hosts: i8) -> Self {
        BotSettings { max_hosts }
    }
}
