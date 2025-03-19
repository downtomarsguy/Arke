use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use serenity::model::guild::Member;
use serenity::model::id::*;
use serenity::model::mention::Mention;
use serenity::prelude::*;

pub async fn run(
    ctx: &Context,
    options: &[ResolvedOption<'_>],
    user_id: UserId,
    guild_id: GuildId,
) {
    let role_id = RoleId::new(1351463564468289557);
    let channel_id = ChannelId::new(1351463494847041570);
    let required_role_id = RoleId::new(1351465303258959962);
    let mentioned_role = Mention::from(role_id);

    match guild_id.member(&ctx.http, user_id).await {
        Ok(member) => {
            if member.roles.contains(&required_role_id) {
                if let Err(why) = channel_id.say(&ctx.http, mentioned_role.to_string()).await {
                    println!("Error sending message: {why:?}");
                }
            } else {
                if let Err(why) = channel_id
                    .say(
                        &ctx.http,
                        "You do not have the required role to use this command.",
                    )
                    .await
                {
                    println!("Error sending message: {why:?}");
                }
            }
        }
        Err(why) => {
            println!("Could not find the member in the guild: {why:?}");
        }
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping_vc").description("Ping VC Role")
}
