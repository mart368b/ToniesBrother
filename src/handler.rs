
use serenity::{
    model::{
        id::GuildId,
        event::PresenceUpdateEvent,
        gateway::Ready,
        user::OnlineStatus
    },
    prelude::*,
};
use log::info;
use crate::{sharemap, get_or_default};
use crate::state::*;
use crate::util::{read_online_image, unpack};
use anyhow::Error;

pub struct Handler;

impl EventHandler for Handler {
    fn presence_update(&self, ctx: Context, mut status: PresenceUpdateEvent) {
        unpack(|| {
            
            
            let user_id = status.presence.user_id;
            let user = user_id.to_user(&ctx)?;

            if let Some(ref mut guild_id) = status.guild_id {
                
                let mvp = {
                    let mut map = sharemap!(mut ctx);

                    let state = get_or_default!(map<StateManager>);
                    let mvp: &MvpState = state.get(&guild_id);

                    (mvp.mvp.clone(), mvp.icon_url.clone())
                };

                if let (Some(mvp), default_icon) = mvp {
                    let current_status = &status.presence.status;
                    if mvp.id == user.id {
                        info!("{}: Mvp status changed to {:?}", guild_id.0, current_status);
                        match current_status {
                            OnlineStatus::DoNotDisturb
                            | OnlineStatus::Offline
                            | OnlineStatus::Invisible => {
                                if let Some(path) = default_icon {
                                    info!("{}: Default image", guild_id.0);
                                    guild_id.edit(&ctx.http, |g| g.icon(Some(path.as_str())))?;
                                }
                            },
                            OnlineStatus::Idle
                            | OnlineStatus::Online => {
                                info!("{}: Mvp image", guild_id.0);
                                guild_id.edit(&ctx.http, |g| g.icon(Some(mvp.icon_url.as_str())))?;
                            },
                            _ => {}
                        }
                    }
                }
            }
            
            Ok(())
        });
    }

    fn ready(&self, ctx: Context, _rdy: Ready) {
        let mut map = sharemap!(mut ctx);
        let _state = get_or_default!(map<StateManager>);
        /*
        let guilds = &rdy.guilds;
        if guilds.is_empty() {
            return;
        }
        for guild in rdy.guilds {
            let icon_url = match &guild {
                GuildStatus::OnlinePartialGuild(g) => {
                    g.icon_url().clone()
                },
                GuildStatus::OnlineGuild(g) => {
                    g.icon_url().clone()
                },
                GuildStatus::Offline(g) => {
                    let gl = g.id.to_partial_guild(&ctx.http).unwrap();
                    gl.icon_url().clone()
                }
                _ => None
            };
            set_default_icon(&ctx, icon_url, guild.id()).unwrap();
        }
        */
    }
}


pub fn set_default_icon(ctx: &Context, icon_url: Option<String>, guild_id: GuildId) -> Result<(), Error>{
    let mut map = sharemap!(mut ctx);
    let state = get_or_default!(map<StateManager>);

    if let Some(url) = icon_url {
        let data = read_online_image(url)?;
        state.update::<MvpState, _>(&guild_id, |v| v.icon_url = Some(data))?;
    }
    Ok(())
}