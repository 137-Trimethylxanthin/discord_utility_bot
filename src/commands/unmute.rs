use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<String, serenity::Error> {
    let guild_id = interaction.guild_id.unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        if let Err(e) = handler.mute(false).await {
            println!("Failed to mute: {:?}", e);
            return Ok(format!("Failed to mute: {:?}", e));
        }

        return Ok("Unmuted :)".to_string());
    } else {
        return Ok("Not in a voice channel to mute in. do you not have eyes?".to_string());
    }
}


pub fn register() -> CreateCommand {
    CreateCommand::new("unmute").description("Unmutes me :)")
}