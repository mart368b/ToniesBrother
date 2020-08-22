use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::CommandResult;
use crate::*;
use crate::util::read_online_image;
use crate::state::*;

#[command]
fn help(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, format!(
        "{}\n{}\n{}\n{}\n{}",
        "To add a person as mvp",
        "Simply write to me \"-mvp @SomePerson\" and attach an image on the message",
        "The servers profile image will then be updated when the mvp changes status",
        "To remove the mvp just write -mvp",
        "To set the image when the mvp is not here use -default with an image"
    ))?;
    Ok(())
}

#[command]
fn reset(ctx: &mut Context, msg: &Message) -> CommandResult {
    if let Some(guild_id) = msg.guild_id {
        let guild = msg.guild(&ctx.cache).unwrap();
        let icon_url = guild.read().icon_url().to_owned();
        set_default_icon(ctx, icon_url, guild_id.clone())?;

        info!("{}: I have now reset the default image", guild_id.0);
        msg.channel_id.say(&ctx.http, "I have now reset the default image")?;
    }
    Ok(())
}

#[command]
fn default(ctx: &mut Context, msg: &Message) -> CommandResult {
    let attachments = &msg.attachments;
    match attachments.as_slice() {
        [a] => {
            if let Some(guild_id) = msg.guild_id {
                let img = read_online_image(a.proxy_url.to_owned())?;
                
                let mut map = sharemap!(mut ctx);
                let manager = get_or_default!(map<StateManager>);
                manager.update(&guild_id, |v| v.icon_url = Some(img))?;

                info!("{}: Default have been set", guild_id.0);
                msg.channel_id.say(&ctx.http, "Default have been set")?;
            } else {
                msg.channel_id.say(&ctx.http, "I can only set default inside a server")?;
            }
        }
        _ => {
            msg.channel_id.say(&ctx.http, "Missing attached photo (-mvp {user mention} {img url})")?;
        }
    }
    Ok(())
}

#[command]
fn mvp(ctx: &mut Context, msg: &Message) -> CommandResult {   
    let mentions = &msg.mentions;
    match mentions.as_slice() {
        [mentioned] => {
            let attachments = &msg.attachments;
            match attachments.as_slice() {
                [a] => {
                    

                    if let Some(guild_id) = msg.guild_id {
                        let img = read_online_image(a.proxy_url.to_owned())?;
                        let mut map = sharemap!(mut ctx);
                        let manager = get_or_default!(map<StateManager>);
                        manager.update(&guild_id,  |v| {
                            v.mvp = Some(Mvp {
                                id: mentioned.id.clone(),
                                icon_url: img
                            });
                        })?;
                        info!("{}: New mvp is {:?}", guild_id.0, mentioned.name);
                        msg.channel_id.say(&ctx.http, format!("New mvp is {:?}", mentioned.name))?;
                    } else {
                        msg.channel_id.say(&ctx.http, "I can only set mvp inside a server")?;
                    }
                    
                }
                _ => {
                    msg.channel_id.say(&ctx.http, "Missing attached photo (-mvp {user mention} {img url})")?;
                }
            }
        },
        [] => {
            if let Some(guild_id) = msg.guild_id {
                let mut map = sharemap!(mut ctx);
                let manager = get_or_default!(map<StateManager>);
                manager.remove(&guild_id)?;
                info!("{}: The mvp have been removed", guild_id.0);
                msg.channel_id.say(&ctx.http, "The mvp have been removed")?;
            } else {
                msg.channel_id.say(&ctx.http, "I can only remove mvp inside a server")?;
            }
        }
        _ => {
            msg.channel_id.say(&ctx.http, "Missing a mention of who is the new mvp (-mvp {user mention} {img url})")?;
        }
    };

    Ok(())
}

#[group]
#[commands(mvp, help, reset, default)]
struct General;