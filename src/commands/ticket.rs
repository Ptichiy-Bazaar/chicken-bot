use serenity::model::interactions::{
    application_command::ApplicationCommandInteraction,
    message_component::{ButtonStyle, MessageComponentInteraction},
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use tracing::debug;

pub const INTRACTION_IDS: [&str; 3] = ["complain", "proposal", "verification"];
const FIVE_HEAD_ICON: &str = "https://i.ibb.co/J2j8np4/images.jpg";
const TICKET_EMBED_TITLE_TEMPLATE: &str = " | Нажмите, чтобы создать тикет";
const TICKET_EMBED_DESCRIPTION: &str = "После создания тикета не забудьте описать свою проблему. Если вы этого не сделаете, то вскоре он будет закрыт";

pub async fn render_ticket(ctx: &Context, interaction: &ApplicationCommandInteraction) {
    let guild = interaction
        .guild_id
        .unwrap()
        .to_guild_cached(ctx)
        .await
        .unwrap();
    let guild_title = format!("{}{}", guild.name, TICKET_EMBED_TITLE_TEMPLATE);
    let guild_icon = guild
        .icon_url()
        .unwrap_or_else(|| String::from(FIVE_HEAD_ICON));

    interaction
        .channel_id
        .send_message(&ctx.http, |b| {
            b.embed(|b| {
                b.author(|b| b.name(guild_title).icon_url(guild_icon))
                    .description(TICKET_EMBED_DESCRIPTION)
            })
            .components(|b| {
                b.create_action_row(|b| {
                    b.create_button(|b| -> &mut serenity::builder::CreateButton {
                        b.custom_id("complain")
                            .label("Жалоба")
                            .style(ButtonStyle::Primary)
                            .emoji(ReactionType::Custom {
                                id: EmojiId(926169321414201465),
                                name: Some(String::from("report")),
                                animated: false,
                            })
                    });
                    b.create_button(|b| {
                        b.custom_id("proposal")
                            .label("Недоработка")
                            .style(ButtonStyle::Danger)
                            .emoji(ReactionType::Custom {
                                id: EmojiId(926169320617287760),
                                name: Some(String::from("bug")),
                                animated: false,
                            })
                    });
                    b.create_button(|b| {
                        b.custom_id("verification")
                            .label("Верификация")
                            .style(ButtonStyle::Success)
                            .emoji(ReactionType::Custom {
                                id: EmojiId(926169321045110824),
                                name: Some(String::from("verify")),
                                animated: false,
                            })
                    })
                })
            })
        })
        .await
        .expect("Cannot send embed interaction");

    interaction
        .create_interaction_response(&ctx.http, |b| {
            b.interaction_response_data(|b| {
                b.content("Success!")
                    .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
            })
        })
        .await
        .expect("Cannot send interaction response");

    debug!("Ticket embed created");
}

pub async fn message_component_ticket(ctx: &Context, interaction: &MessageComponentInteraction) {
    interaction
        .create_interaction_response(&ctx.http, |b| {
            b.interaction_response_data(|b| b.content("nice cock"))
        })
        .await;
}
