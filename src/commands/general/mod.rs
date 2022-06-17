use crate::rename_member;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use serenity::futures::future::join_all;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

#[group]
#[only_in(guilds)]
#[commands(overwatching)]
struct General;

#[command]
#[required_permissions("MANAGE_NICKNAMES")]
async fn overwatching(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg
        .guild(&ctx.cache)
        .expect("Command was not called from a guild");

    join_all(
        guild
            .members(&ctx.http, None, None)
            .await?
            .iter()
            .map(|member| rename_member(member, ctx)),
    )
    .await;
    Ok(())
}
