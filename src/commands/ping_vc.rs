use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use serenity::model::id::*;
use serenity::model::mention::Mention;
use serenity::prelude::*;

pub async fn run(ctx: &Context, _options: &[ResolvedOption<'_>]) {
    let role_id = RoleId::new(1351463564468289557);
    let channel_id = ChannelId::new(1351463494847041570);
    let mentioned_role = Mention::from(role_id);

    if let Err(why) = channel_id.say(&ctx.http, mentioned_role.to_string()).await {
        println!("Error sending message: {why:?}");
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping_vc").description("Ping VC Role")
}
