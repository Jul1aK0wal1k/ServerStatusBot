use serenity::{
    builder::CreateApplicationCommand,
    model::interactions::application_command::ApplicationCommandOptionType,
};

pub fn add_server_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("add_server")
        .description("Add a server address to watch")
        .create_option(|option| {
            option
                .name("address")
                .description("The address to watch in the format of <host or IP>:<port>")
                .kind(ApplicationCommandOptionType::String)
                .required(true)
        })
}
