use serenity::builder::CreateApplicationCommand;

pub fn ping_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("Pong")
}
