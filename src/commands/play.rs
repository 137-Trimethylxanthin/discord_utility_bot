use serenity::all::{CommandInteraction, Context, CreateCommand, CreateCommandOption, GuildId, ResolvedOption};
use songbird::input::YoutubeDl;
use crate::commands::join;
use crate::HttpKey;


async fn is_in_channel(ctx: &Context, guild_id: GuildId) -> bool {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    manager.get(guild_id).is_some()
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<String, serenity::Error> {

    let url = interaction.data.options.first().unwrap().value.as_str().unwrap();


    let do_search = !url.starts_with("http");

    let guild_id = interaction.guild_id.unwrap();

    let http_client = {
        let data = ctx.data.read().await;
        data.get::<HttpKey>()
            .cloned()
            .expect("Guaranteed to exist in the typemap.")
    };

    if !is_in_channel(ctx, guild_id).await {
        join::run(ctx, interaction).await.unwrap();
    }

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();


    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let mut src = if do_search {
            YoutubeDl::new(http_client, format!("ytsearch1:{}", url))
        } else {
            YoutubeDl::new(http_client, url.to_string())
        };
        let _ = handler.play_input(src.clone().into());

        return Ok(format!("Playing: {:?}", src));
    } else {
        return Ok("Not in a voice channel to play in. do you not have eyes?".to_string());
    }

}

pub fn register() -> CreateCommand {
    CreateCommand::new("play").description("Plays a song with the provided link").add_option(
        CreateCommandOption::new(serenity::all::CommandOptionType::String, "link", "The link to the song you want to play or a query for yt")
            .required(true),
    )
}