use serenity::{
    builder::{CreateActionRow, CreateButton},
    client::Context,
    model::{channel::Message, interactions::message_component::ButtonStyle},
};

pub async fn message(ctx: Context, msg: Message) -> Result<(), serenity::Error> {
    lazy_static::lazy_static! {
        static ref PASTE_SITE: String = std::env::var("PASTEBIN").expect("No pastebin domain set!");
    }
    if msg.attachments.is_empty() || msg.author.bot {
        return Ok(());
    }
    if let Some(channel) = msg.channel_id.to_channel(&ctx.http).await?.guild() {
        if let Ok(perms) = channel.permissions_for_user(&ctx.cache, &ctx.cache.current_user().id) {
            if !perms.send_messages() {
                return Ok(());
            }
        }
    }
    let mut rows: Vec<CreateActionRow> = Vec::new();
    let mut row = CreateActionRow::default();
    for attachment in &msg.attachments {
        if let Some(ctype) = &attachment.content_type {
            if !ctype.to_ascii_lowercase().contains("charset=utf-8") {
                continue;
            }
        }
        let mut button = CreateButton::default();
        button.style(ButtonStyle::Link);
        button.label(format!("View {}", attachment.filename));
        button.emoji('📜');
        button.url(format!(
            "https://{}/{}/{}/{}",
            *PASTE_SITE,
            msg.channel_id,
            attachment.id,
            attachment.filename
        ));
        row.add_button(button);
        if row.0.len() >= 5 {
            rows.push(row.clone());
            row = CreateActionRow::default()
        }
    }
    // If the length of the current row is less then 0, add it to the list of rows
    if !row.0.is_empty() {
        rows.push(row)
    }
    if rows.is_empty() {
        return Ok(());
    }
    msg.channel_id
        .send_message(&ctx, |m| {
            m.content(format!(
                "Web version of attachments from <@{}>",
                msg.author.id
            ))
            .allowed_mentions(|am| am.empty_parse())
            .components(|c| {
                for actionrow in rows {
                    c.add_action_row(actionrow);
                }
                c
            })
            .reference_message(&msg)
        })
        .await?;
    Ok(())
}
