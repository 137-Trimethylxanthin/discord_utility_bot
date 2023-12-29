use serenity::all::{CommandInteraction, Context, CreateCommand, ResolvedOption};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<String, serenity::Error> {

    let guild_id = interaction.guild_id.unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            return Ok("Not in a voice channel to mute, why would you even try this? *idiot*".to_string());
        },
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_mute() {
        return Ok("I'm already muted! leave me alone now!!".to_string());
    } else {
        if let Err(e) = handler.mute(true).await {
            return Ok(format!("Failed to mute: {:?}", e));
        }

        return Ok("Now Muted".to_string());
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("mute").description("Mutes me :X")
}