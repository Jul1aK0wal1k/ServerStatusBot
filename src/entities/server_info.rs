use crate::entities::Address;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ServerInfo {
    name: String,
    address: Address,
    map: String,
    num_players: u16,
    max_players: u16,
}

impl ServerInfo {
    pub fn new(name: String, address: Address, map: String, num_players: u16, max_players: u16) -> Self {
        ServerInfo {
            name,
            address,
            map,
            num_players,
            max_players,
        }
    }
}
