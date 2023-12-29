use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<String, serenity::Error> {
    Ok("Hey, I'm alive!".to_string())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("check").description(" Checks that a message successfully sent; if not, then logs why to stdout.")
}