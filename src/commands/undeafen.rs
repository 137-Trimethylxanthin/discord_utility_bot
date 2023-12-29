use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<String, serenity::Error> {
    let guild_id = interaction.guild_id.unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    return if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        if let Err(e) = handler.deafen(false).await {
            println!("Failed to deafen: {:?}", e);
            return Ok(format!("Failed to deafen: {:?}", e));
        }

        Ok("Hey, I can hear again 😄".to_string())
    } else {
        Ok("Not in a voice channel to undeafen in".to_string())
    }


}

pub fn register() -> CreateCommand {
    CreateCommand::new("undeafen").description("Undeafens me")
}