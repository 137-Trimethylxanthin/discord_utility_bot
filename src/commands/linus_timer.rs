use serenity::async_trait;
use serenity::builder::*;
use serenity::futures::TryFutureExt;
use serenity::model::prelude::*;
use serenity::prelude::*;
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};
use log::error;





pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<String, serenity::Error> {
    // get the GuildId
    let Some(guild_id) = interaction.guild_id else {
        error!("failed to retrieve GuildId from CommandInteraction");
        return Ok("I broke :(".to_string());
    };


    let user_id = UserId::new(395517793510096897);

    let (guild_id, channel_id) = {
        let guild = guild_id.to_guild_cached(&ctx.cache).unwrap();
        let channel_id = guild
            .voice_states
            .get(&user_id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };


    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            return Ok("Linus is no where in sight".to_string());
        },
    };

    //check if the user 889873976095027230 is in the channel
    

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
        // Attach an event handler to see notifications of all track errors.
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
    }

    Ok(format!("connected to channel: {:?}", channel_id.unwrap()))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("linus_timer").description("Starts a timer and waits for Linus")
}

struct TrackErrorNotifier;


#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                println!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }

        None
    }
}
