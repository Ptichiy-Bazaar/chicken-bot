use std::fmt::Write as _;

use poise::serenity::model::{channel::ReactionType, interactions::message_component::ButtonStyle};
// use thiserror::Error;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::EmojiId;

use crate::{Context, Data, Error};
//#[derive(Error, Debug)]
//pub enum TicketError {
//    Already
//}

// TODO: Change to guild name
const TICKET_EMBED_TITLE: &'static str = " Птичий базар | Нажмите, чтобы создать тикет";
const TICKET_EMBED_DESCRIPTION: &'static str = "После создания тикета не забудьте описать свою проблему. Если вы этого не сделаете, то вскоре он будет закрыт";

#[poise::command(prefix_command, slash_command)]
pub async fn ticket(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|b| {
        b.embed(|b| {
            b.author(|b| {
                b.name(TICKET_EMBED_TITLE)
                    .icon_url("https://i.ibb.co/L9mnKM8/photo-2019-05-31-20-48-45.jpg")
            })
            .description(TICKET_EMBED_DESCRIPTION)
        })
        .components(|b| {
            b.create_action_row(|b| {
                b.create_button(|b| {
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
    .await?;

    // XXX: :flushed:
    // TODO: Make ephemerical
    ctx.say("SUCKSASS").await?;
    Ok(())
}
