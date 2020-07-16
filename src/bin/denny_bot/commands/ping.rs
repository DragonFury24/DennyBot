use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

//Responds to ping with "Pong!"
#[command]
pub fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| m.content("Pong!").reactions(vec!["✅"])) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
