use commands::{ColorSchemeCommand, VimMotionsHellCommand};
use nvim_oxi::{libuv::AsyncHandle, schedule, Result};
use plugin::chaos_mode::Mode;
use std::thread;
use tokio::sync::mpsc::{self};

mod commands;
mod plugin;
mod twitch;

use plugin::Plugin;
use twitch::{TwitchCommand, TwitchCommandPayload};

/// # Panics
///
/// Will panic if payload can't be received from AsyncHandle
///
/// # Errors
///
#[nvim_oxi::plugin]
pub fn nvim_chaos() -> Result<()> {
    let (sender, mut receiver) = mpsc::unbounded_channel::<TwitchCommandPayload>();

    let mut plugin = Plugin::default();
    plugin.init()?;

    let handle = AsyncHandle::new(move || {
        let payload = receiver.blocking_recv().expect("Payload receiving error");

        let mut plugin = plugin.clone();

        schedule(move |()| match payload.command {
            TwitchCommand::Message(author, text) => {
                plugin
                    .show_msg(author.as_str(), text.as_str())
                    .unwrap_or_else(|e| {
                        let error_string = format!("Plugin Error: {e}");
                        Plugin::err(&error_string);
                    });
            }
            TwitchCommand::ColorScheme(colorscheme) => {
                plugin
                    .set_mode(
                        Mode::ColorScheme(ColorSchemeCommand { colorscheme }),
                        5 * 60,
                    )
                    .unwrap_or_else(|e| {
                        let error_string = format!("Plugin Error: {e}");
                        Plugin::err(&error_string);
                    });
            }
            TwitchCommand::VimMotionsHell => {
                plugin
                    .set_mode(Mode::VimMotionsHell(VimMotionsHellCommand {}), 60)
                    .unwrap_or_else(|e| {
                        let error_string = format!("Plugin Error: {e}");
                        Plugin::err(&error_string);
                    });
            }
        });
    })?;

    thread::spawn(move || {
        twitch::init(handle, sender, "mikerime".to_owned()).unwrap_or_else(|e| {
            println!("{e}");
        });
    });

    Ok(())
}
