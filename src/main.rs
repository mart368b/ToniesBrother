use anyhow::Error;
use serenity::Client;
use serenity::framework::standard::StandardFramework;

pub mod state;
mod handler;
mod command;
pub mod util;

pub use command::*;
pub use handler::*;
use log::info;
use std::env::set_var;

fn main() -> Result<(), Error> {
    
    let token = "NzI0OTEwOTczNzcyNzU5Mjgx.XwjOSg.nS70cTRUe5uatp9Uf6DY32P4dC4";
    
    let mut client = Client::new(&token, Handler)?;
    
    let framework = StandardFramework::new()
    .configure(|c| c.prefix("-"))
    .group(&GENERAL_GROUP);
    
    client.with_framework(framework);
    
    set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Starting");
    client.start()?;

    Ok(())
}