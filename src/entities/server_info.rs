use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ServerInfo {
    name: String,
    host: String,
    port: u16,
    map: String,
    num_players: u16,
    max_players: u16,
}

impl ServerInfo {
    pub fn new(
        name: String,
        host: String,
        port: u16,
        map: String,
        num_players: u16,
        max_players: u16,
    ) -> Self {
        ServerInfo {
            name,
            host,
            port,
            map,
            num_players,
            max_players,
        }
    }
}
