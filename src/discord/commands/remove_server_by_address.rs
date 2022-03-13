use serenity::{
    builder::CreateApplicationCommand,
    model::interactions::application_command::ApplicationCommandOptionType,
};

pub fn remove_server_by_address_command(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("remove_server_by_address")
        .description("Removes a server from tracking")
        .create_option(|option| {
            option
                .name("address")
                .description("Address in the format of <host or IP>:<port>")
                .kind(ApplicationCommandOptionType::String)
                .required(true)
        })
}
