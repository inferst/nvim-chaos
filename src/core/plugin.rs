use std::{cell::RefCell, rc::Rc, str::FromStr, thread, time::Duration};

use nvim_oxi::{
    api::{self, opts::EchoOpts},
    libuv::{AsyncHandle, TimerHandle},
    schedule, Dictionary, Function, Object, Result,
};

use tokio::sync::mpsc;

use crate::{
    commands::{Background, ColorSchemeCommand, Mode, ModeType, VimMotionsHellCommand},
    twitch::{self},
};

use super::{
    chaos_mode::{self},
    config::Config,
    message::{self},
};

#[derive(Clone, Default)]
pub struct State {
    pub message: message::State,
    pub chaos_mode: chaos_mode::State,
}

#[derive(Clone, Default)]
pub struct Plugin {
    pub state: Rc<RefCell<State>>,
    pub config: Config,
}

impl Plugin {
    pub fn init(&mut self, config: Config) -> Result<()> {
        self.config = config;

        let (sender, mut receiver) = mpsc::unbounded_channel::<twitch::CommandPayload>();

        let plugin = self.clone();

        let handle = AsyncHandle::new(move || {
            let payload = receiver.blocking_recv();

            if let Some(payload) = payload {
                plugin.handle_payload(payload);
            } else {
                schedule(move |()| {
                    Plugin::err("[nvim-chaos] Payload receiving error");
                });
            }
        })?;

        let config = self.config.clone();

        if config.channel.is_some() {
            thread::spawn(move || {
                twitch::init(handle, sender, config).unwrap_or_else(|error| {
                    schedule(move |()| {
                        Plugin::err(&format!("[nvim-chaos] {error}"));
                    });
                });
            });

            self.start_timer()?;

            let mut state = self.state.borrow_mut();

            state.chaos_mode.init()?;
            state.message.init()?;
        }

        Ok(())
    }

    fn handle_payload(&self, payload: twitch::CommandPayload) {
        let mut plugin = self.clone();

        schedule(move |()| {
            if let Err(error) = plugin.parse_command(payload.command) {
                Plugin::err(&format!("[nvim-chaos] {error}"));
            }
        });
    }

    fn parse_command(&mut self, command: twitch::Command) -> Result<()> {
        match command {
            twitch::Command::Message(author, text) => {
                self.show_msg(author.as_str(), text.as_str())?;
            }
            twitch::Command::ColorScheme(colorscheme, background) => {
                let background = Background::from_str(&background).unwrap();
                let mode: Mode = ColorSchemeCommand {
                    colorscheme,
                    background,
                }
                .into();
                self.set_mode(
                    mode,
                    ModeType::ColorSchemeType,
                    self.config.commands.colorscheme.duration,
                )?;
            }
            twitch::Command::VimMotionsHell => {
                let mode: Mode = VimMotionsHellCommand {}.into();
                self.set_mode(
                    mode,
                    ModeType::VimMotionsHellType,
                    self.config.commands.hell.duration,
                )?;
            }
        }

        Ok(())
    }

    fn parse_config(&mut self, preferences: Object) {
        let config = Config::try_from(preferences);

        match config {
            Ok(config) => {
                self.init(config).unwrap();
            }
            Err(error) => {
                let opts = EchoOpts::builder().build();
                let chunks = [
                    ("[nvim-chaos]", Some("NvimChaosErrTag")),
                    (" ", None),
                    (&format!("{error}"), None),
                ];
                let _ = api::echo(chunks, true, &opts);
            }
        }
    }

    pub fn build_api(&mut self) -> nvim_oxi::Dictionary {
        let plugin = self.clone();

        let setup = Function::from_fn(move |preferences: Object| {
            let mut plugin = plugin.clone();
            plugin.parse_config(preferences);
        });

        Dictionary::from_iter([("setup", setup)])
    }

    pub fn err(str: &str) {
        api::err_writeln(str);
    }

    fn update(&mut self) -> Result<()> {
        let mut state = self.state.borrow_mut();
        state.chaos_mode.tick()?;

        Ok(())
    }

    pub fn set_mode(&mut self, mode: Mode, mode_type: ModeType, seconds: u32) -> Result<()> {
        let mut state = self.state.borrow_mut();
        state.chaos_mode.set_mode(mode, mode_type, seconds)?;

        Ok(())
    }

    fn start_timer(&mut self) -> Result<()> {
        let plugin = self.clone();

        let callback = move |_timer: &mut TimerHandle| {
            let mut plugin = plugin.clone();

            schedule(move |()| {
                plugin.update().unwrap();
            });
        };

        TimerHandle::start(Duration::from_millis(0), Duration::from_secs(1), callback)?;

        Ok(())
    }

    pub fn show_msg(&mut self, author: &str, message: &str) -> Result<()> {
        let mut state = self.state.borrow_mut();
        state.message.show_msg(author, message)?;

        Ok(())
    }
}
