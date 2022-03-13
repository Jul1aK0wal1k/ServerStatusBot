mod add_server;
pub mod handlers;
mod list_servers;
mod ping;
mod remove_server_by_address;
mod remove_server_by_alias;
mod set_channel;

pub use {
    add_server::add_server_command, list_servers::list_servers_command, ping::ping_command,
    remove_server_by_address::remove_server_by_address_command,
    remove_server_by_alias::remove_server_by_alias_command, set_channel::set_channel_command,
};
// mod commands {
//     pub use set_channel::,
// }
