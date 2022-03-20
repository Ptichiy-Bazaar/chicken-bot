use serenity::model::channel::{PermissionOverwrite, PermissionOverwriteType};
use serenity::model::interactions::{
    application_command::ApplicationCommandInteraction,
    message_component::{ButtonStyle, MessageComponentInteraction},
};
use serenity::model::permissions::Permissions;
use serenity::model::prelude::*;
use serenity::prelude::*;

use tracing::debug;

pub const INTRACTION_IDS: [&str; 3] = ["complain", "proposal", "verification"];
const FIVE_HEAD_ICON: &str = "https://i.ibb.co/J2j8np4/images.jpg";
const TICKET_EMBED_TITLE_TEMPLATE: &str = " | Нажмите, чтобы создать тикет";
const TICKET_EMBED_DESCRIPTION: &str = "После создания тикета не забудьте описать свою проблему. Если вы этого не сделаете, то вскоре он будет закрыт";
const SUPPORT_SERVER_CATEGORY_ID: u64 = 955079859934220300;
const SUPPORT_CHANNEL_USER_PERMISSIONS: Permissions = Permissions { bits: 68608 };
const SUPPORT_ROLE_ID: RoleId = RoleId(823186689316093992);

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
    let channel_prefix = match interaction.data.custom_id.as_str() {
        "complain" => "compl",
        "proposal" => "propl",
        "verification" => "verif",
        _ => unreachable!(),
    };
    // Actually just discord snowflake timestamp
    let channel_hash = *interaction.id.as_u64() >> 22;
    // NOTE: mAybE UsE BiGgER AlPhaBet?
    let channel_name = format!("{}-{:x}", channel_prefix, channel_hash);

    let guild_id = interaction
        .guild_id
        .expect("Impossible because we create only slash commands");
    let user = interaction.member.as_ref().expect("IMPOSSIBLE").user.id;
    let channel_permissions = vec![
        PermissionOverwrite {
            allow: SUPPORT_CHANNEL_USER_PERMISSIONS,
            kind: PermissionOverwriteType::Member(user),
            deny: Permissions::empty(),
        },
        PermissionOverwrite {
            allow: SUPPORT_CHANNEL_USER_PERMISSIONS,
            kind: PermissionOverwriteType::Role(SUPPORT_ROLE_ID),
            deny: Permissions::empty(),
        },
    ];
    let ticket_channel = guild_id
        .create_channel(&ctx.http, |c| {
            c.name(channel_name)
                .kind(ChannelType::Text)
                .category(SUPPORT_SERVER_CATEGORY_ID)
                .permissions(channel_permissions)
        })
        .await
        .expect("Impossible");

    // TODO: Use guild for channel creation
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
    let ticket_answer_description = match interaction.data.custom_id.as_str() {
        "verification" => "Пожалуйста, терпеливо ждите ответа от команды сервера, а пока, чтобы убедиться в вашем мастерстве, пожалуйста, отправьте пару своих работ",
        _ => "Пожалуйста, терпеливо ожидайте ответа от команды сервера, а пока опишите вашу проблему как можно подробнее"
    };

    let ticket_answer_msg_id = ticket_channel
        .send_message(&ctx.http, |b| {
            b.embed(|b| {
                b.author(|b| b.name(guild_title).icon_url(guild_icon))
                    .description(ticket_answer_description)
            })
            .content(format!("<@&{}>", SUPPORT_ROLE_ID))
        })
        .await
        .expect("Impossible")
        .id;

    interaction
        .create_interaction_response(&ctx.http, |b| {
            b.interaction_response_data(|b| {
                b.components(|b| {
                    b.create_action_row(|b| {
                        b.create_button(|b| {
                            b.label("Перейти к заявке")
                                .style(ButtonStyle::Link)
                                .url(ticket_answer_msg_id.link(ticket_channel.id, Some(guild_id)))
                        })
                    })
                })
                .content("Запрос оставлен")
                .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
            })
        })
        .await
        .expect("Impossible");
}
