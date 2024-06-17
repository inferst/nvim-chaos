use nvim_oxi::{Dictionary, Result};

mod commands;
mod plugin;
mod twitch;

use plugin::Plugin;

/// # Panics
///
/// Will panic if payload can't be received from AsyncHandle
///
/// # Errors
///
#[nvim_oxi::plugin]
pub fn nvim_chaos() -> Result<Dictionary> {
    let mut plugin = Plugin::default();
    let api = plugin.build_api()?;

    Ok(api)
}
