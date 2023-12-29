use serenity::all::{CommandInteraction, Context, CreateCommand};

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<String, serenity::Error> {
    let guild_id = interaction.guild_id.unwrap();
    let mut response = "Deafening..".to_string();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            let response = "Not in a vc you dumb fuck".to_string();

            return Ok(response);
        },
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_deaf() {
        response = "Already deafened".to_string();
    } else {
        response = "Deafened".to_string();

        if let Err(e) = handler.deafen(true).await {
            response = format!("Failed: {:?}", e);
        }
    }



    Ok(response)
}



pub fn register() -> CreateCommand {
    CreateCommand::new("deafen").description("deafens the bot :)")
}