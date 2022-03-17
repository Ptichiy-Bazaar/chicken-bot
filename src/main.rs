mod commands;

use crate::commands::ticket;

use std::env;

use serenity::{
    async_trait,
    http::Http,
    model::{
        event::ResumedEvent,
        gateway::Ready,
        id::GuildId,
        interactions::{
            application_command::{
                ApplicationCommand, ApplicationCommandInteractionDataOptionValue,
                ApplicationCommandOptionType,
            },
            Interaction, InteractionResponseType,
        },
    },
    prelude::*,
    utils::Content,
};
use tracing::{debug, error, info};

const SERVER_GUILD_ID: GuildId = GuildId(765071175768342599);

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);

        let commands = GuildId::set_application_commands(&SERVER_GUILD_ID, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ticket").description("Создать тикет")
                })
                .create_application_command(|command| {
                    command
                        .name("id")
                        .description("Get a user id")
                        .create_option(|option| {
                            option
                                .name("id")
                                .description("The user to lookup")
                                .kind(ApplicationCommandOptionType::User)
                                .required(true)
                        })
                })
        })
        .await;

        debug!(
            "Registered the following guild slash commands: {:#?}",
            commands
        );
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "ticket" => {
                    ticket::render_ticket(&ctx, &command).await;
                }
                _ => (),
            };
        } else if let Interaction::MessageComponent(msg_component) = interaction {
            if ticket::INTRACTION_IDS
                .iter()
                .any(|&id| id == msg_component.data.custom_id)
            {
                ticket::message_component_ticket(&ctx, &msg_component).await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    // Retrieve application info via HTTP
    let application_info = http
        .get_current_application_info()
        .await
        .expect("Expected application info");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .application_id(application_info.id.0)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
