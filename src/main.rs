mod commands;

use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use dotenv::dotenv;
use log::error;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::id::GuildId;
use serenity::prelude::*;
use serenity::client::Context;
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};
use songbird::SerenityInit;
use tokio::signal;

use serenity::{
    async_trait,
    client::{Client, EventHandler},
    framework::{
        standard::{
            macros::{command, group},
            Args,
            CommandResult,
            Configuration,
        },
        StandardFramework,
    },
    model::{channel::Message, gateway::Ready},
    prelude::{GatewayIntents, TypeMapKey},
    Result as SerenityResult,
};

use reqwest::Client as HttpClient;
use serenity::all::{ChannelId, ChannelType, VoiceState};


struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}


#[derive(Clone)]
struct Handler {
    bot_id: Arc<AtomicU64>,
}


#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );


        // local commands
        let commands = guild_id
            .set_commands(&ctx.http, vec![
                commands::ping::register(),
                commands::deafen::register(),
                commands::join::register(),
                commands::leave::register(),
                commands::play::register(),
                commands::undeafen::register(),
                commands::mute::register(),
                commands::unmute::register(),
                commands::check::register(),
            ])
            .await;


        // global commands
        let guild_command =
            Command::create_global_command(&ctx.http, commands::wonderful_command::register())
                .await;


        self.bot_id
            .store(ready.user.id.get(), Ordering::Relaxed);
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        if let Some(channel) = old.and_then(|c| c.channel_id) {
            self.disconnect_if_alone(&ctx, channel).await;
        }
        if let Some(channel) = new.channel_id {
            self.disconnect_if_alone(&ctx, channel).await;
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");


            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                "wonderful_command" => Some(commands::wonderful_command::run(&command.data.options())),
                "deafen" => Some(commands::deafen::run(&ctx, &command).await.unwrap()),
                "join" => Some(commands::join::run(&ctx, &command).await.unwrap()),
                "leave" => Some(commands::leave::run(&ctx, &command).await.unwrap()),
                "play" => Some(commands::play::run(&ctx, &command).await.unwrap()),
                "undeafen" => Some(commands::undeafen::run(&ctx, &command).await.unwrap()),
                "mute" => Some(commands::mute::run(&ctx, &command).await.unwrap()),
                "unmute" => Some(commands::unmute::run(&ctx, &command).await.unwrap()),
                "check" => Some(commands::check::run(&ctx, &command).await.unwrap()),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }
}


impl Handler {

    async fn disconnect_if_alone(&self, ctx: &Context, channel: ChannelId) {
        let channel = channel.to_channel(&ctx).await.ok().unwrap().guild().unwrap();

        if channel.kind != ChannelType::Voice {
            return;
        }

        let members = channel
            .members(&ctx)
            .expect("Cannot fetch member list");
        if !(members.len() == 1 && members[0].user.id == self.bot_id.load(Ordering::Relaxed)) {
            return;
        }

        let manager = songbird::get(ctx)
            .await
            .expect("Failed to get songbird manager");
        if let Some(call) = manager.get(channel.guild_id) {
            let mut call_lock = call.lock().await;
            call_lock
                .leave()
                .await
                .expect("Voice disconnection failure");
            call_lock.remove_all_global_events();
        }
    }
}


#[tokio::main]
async fn main() {


    dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_VOICE_STATES;
    // Build our client.
    let mut client = Client::builder(token, intents)
        .event_handler(Handler {
            bot_id: Arc::new(AtomicU64::new(0)),
        })
        .register_songbird()
        .type_map_insert::<HttpKey>(HttpClient::new())
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    tokio::spawn(async move {
        let _ = client
            .start()
            .await
            .map_err(|why| println!("Client ended: {:?}", why));
    });

    let _signal_err = signal::ctrl_c().await;
    println!("Received Ctrl-C, shutting down.");
}


