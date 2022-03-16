use serenity::model::interactions::{
    application_command::ApplicationCommandInteraction, message_component::ButtonStyle,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use tracing::{debug, error, info};

const TICKET_EMBED_DESCRIPTION: &str = "После создания тикета не забудьте описать свою проблему. Если вы этого не сделаете, то вскоре он будет закрыт";

pub async fn render_ticket(ctx: &Context, interaction: &ApplicationCommandInteraction) {
    interaction
        .channel_id
        .send_message(&ctx.http, |b| {
            b.embed(|b| {
                b.author(|b| {
                    b.name("Title")
                        .icon_url("https://i.ibb.co/J2j8np4/images.jpg")
                })
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
    debug!("Ticket embed created");

    interaction
        .channel_id
        .send_message(&ctx.http, |b| {
            b.content("Success!").flags(MessageFlags::EPHEMERAL)
        })
        .await
        .expect("Cannot send ephemeral message");
}
