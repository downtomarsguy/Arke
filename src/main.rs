mod commands;

use dotenv::dotenv;
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::{GuildId, UserId};
use serenity::prelude::*;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let guild_id = command.guild_id.unwrap();
            let user_id = command.user.id;

            match command.data.name.as_str() {
                "ping" => {
                    let content = commands::ping::run(&command.data.options());
                    let data = CreateInteractionResponseMessage::new().content(content);
                    let builder = CreateInteractionResponse::Message(data);
                    if let Err(why) = command.create_response(&ctx.http, builder).await {
                        println!("Cannot respond to slash command: {why}");
                    }
                }
                "ping_vc" => {
                    commands::ping_vc::run(&ctx, &command.data.options(), user_id, guild_id).await;
                    let data =
                        CreateInteractionResponseMessage::new().content("Pinging da VC Role...");
                    let builder = CreateInteractionResponse::Message(data);
                    if let Err(why) = command.create_response(&ctx.http, builder).await {
                        println!("Cannot respond to slash command: {why}");
                    }
                }
                _ => {
                    let data = CreateInteractionResponseMessage::new()
                        .content("Command not implemented :(".to_string());
                    let builder = CreateInteractionResponse::Message(data);
                    if let Err(why) = command.create_response(&ctx.http, builder).await {
                        println!("Cannot respond to slash command: {why}");
                    }
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = guild_id
            .set_commands(
                &ctx.http,
                vec![commands::ping::register(), commands::ping_vc::register()],
            )
            .await;

        println!("Registered guild slash commands: {commands:#?}");

        let global_command =
            Command::create_global_command(&ctx.http, commands::ping::register()).await;

        println!("Registered global slash command: {global_command:#?}");
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("API_KEY").expect("Expected API_KEY in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
