use std::env;
use anyhow::{anyhow, Error};
use tokio::runtime::Runtime;

const DISCORD_API_ENV: &str = "DISCORD_API_KEY";

fn get_token() -> Result<String, Error> {
    Ok(env::var(DISCORD_API_ENV)?)
}

pub fn unpack<F: FnMut() -> Result<R, Error>, R>(mut f: F) {
    match f() {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

pub fn read_online_image(url: String) -> Result<String, Error> {
    let mut runtime = Runtime::new()?;
    let resp = runtime.block_on(reqwest::get(&url))?;
    let b: Vec<_> = runtime.block_on(resp.bytes())?
        .into_iter()
        .collect();
    
    let b64 = base64::encode(&b);
    
    let ext = url
        .split(".")
        .into_iter()
        .last()
        .ok_or_else(|| anyhow!("Failed to read url  {}", url))?;

    Ok(format!("data:image/{};base64,{}", ext, b64))
}

#[macro_export]
macro_rules! sharemap {
    ($ctx:ident) => {
        &*$ctx.data.read()
    };
    (mut $ctx:ident) => {
        $ctx.data.write()
    };
}

#[macro_export]
macro_rules! get_or_default {
    ($m:ident<$ty:ty>) => {{
        $m.entry::<$ty>().or_insert_with(Default::default)
    }};
}