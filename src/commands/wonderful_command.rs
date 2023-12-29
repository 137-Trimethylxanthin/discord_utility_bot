use serenity::all::ResolvedOption;
use serenity::builder::CreateCommand;

pub fn register() -> CreateCommand {
    CreateCommand::new("wonderful_command").description("An amazing command")
}

pub fn run(_options: &[ResolvedOption]) -> String {
    "This is a wonderful command!".to_string()
}