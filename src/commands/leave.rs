use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<String, serenity::Error> {
    let guild_id = interaction.guild_id.unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            return Ok(format!("Failed **F**: {:?}", e));
        }

        return Ok("Left voice channel (*i am never coming back to you again*)".to_string());
    } else {
        return Ok("Not in a voice channel to leave from. should i just leave the server? or what".to_string());
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("leave").description("Leaves the current vc")
}