use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::prelude::Member;
use serenity::prelude::{Context, EventHandler};

pub mod commands;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        println!("New member: '{}'", new_member.display_name());
        rename_member(&new_member, &ctx).await;
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Logged in as: {}", ready.user.name);
    }
}

#[inline]
async fn rename_member(member: &Member, ctx: &Context) {
    let name = member.display_name();
    if name.to_lowercase().ends_with("_ow") {
        return;
    }

    let success = member
        .edit(&ctx.http, |m| m.nickname(format!("{}_ow", name)))
        .await;
    match success {
        Ok(_) => {
            println!("Renamed '{}'", name);
        }
        Err(_) => {
            println!("Failed to rename user '{}'", name);
        }
    }
}
