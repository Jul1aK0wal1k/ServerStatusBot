use serenity::builder::CreateApplicationCommand;

pub fn list_servers_command(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand {
    command
        .name("list_servers")
        .description("Lists all added servers that are currently being tracked")
}
