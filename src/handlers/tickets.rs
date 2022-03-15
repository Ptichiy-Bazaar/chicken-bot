use poise::serenity::model::{channel::ReactionType, interactions::message_component::ButtonStyle};
// use thiserror::Error;
use poise::serenity_prelude::EmojiId;

use crate::{Context, Error};
//#[derive(Error, Debug)]
//pub enum TicketError {
//    Already
//}

const FIVE_HEAD_ICON: &str = "https://i.ibb.co/J2j8np4/images.jpg";
const TICKET_EMBED_TITLE_TEMPLATE: &str = " | Нажмите, чтобы создать тикет";
const TICKET_EMBED_DESCRIPTION: &str = "После создания тикета не забудьте описать свою проблему. Если вы этого не сделаете, то вскоре он будет закрыт";

#[poise::command(slash_command)]
pub async fn ticket(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().expect("A🦈");
    let guild_title = format!("{}{}", guild.name, TICKET_EMBED_TITLE_TEMPLATE);
    let guild_icon = guild
        .icon_url()
        .unwrap_or_else(|| String::from(FIVE_HEAD_ICON));

    ctx.send(|b| {
        b.embed(|b| {
            b.author(|b| b.name(guild_title).icon_url(guild_icon))
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

    ctx.send(|b| b.content("Success!").ephemeral(true)).await?;
    Ok(())
}
