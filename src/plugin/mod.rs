use std::{cell::RefCell, rc::Rc, thread, time::Duration};

use config::Config;
use message::MessageState;
use nvim_oxi::{
    api::{self, opts::EchoOpts},
    libuv::{AsyncHandle, TimerHandle},
    schedule, Dictionary, Function, Object, Result,
};

pub mod chaos_mode;
pub mod config;
pub mod message;

use chaos_mode::ChaosModeState;
use tokio::sync::mpsc;

use crate::{
    commands::{ColorSchemeCommand, Mode, ModeType, VimMotionsHellCommand},
    twitch::{self, TwitchCommand, TwitchCommandPayload},
};

#[derive(Clone, Default)]
pub struct State {
    pub message: MessageState,
    pub chaos_mode: ChaosModeState,
}

#[derive(Clone, Default)]
pub struct Plugin {
    pub state: Rc<RefCell<State>>,
}

impl Plugin {
    pub fn init(&mut self, config: Config) -> Result<()> {
        let (sender, mut receiver) = mpsc::unbounded_channel::<TwitchCommandPayload>();

        let plugin = self.clone();

        let handle = AsyncHandle::new(move || {
            let payload = receiver.blocking_recv().expect("Payload receiving error");
            plugin.handle_payload(payload);
        })?;

        if config.channel.is_some() {
            thread::spawn(move || {
                twitch::init(handle, sender, config).unwrap_or_else(|e| {
                    println!("{e}");
                });
            });

            self.start_timer();

            let mut state = self.state.borrow_mut();
            state.chaos_mode.init()?;
            state.message.init()?;
        }

        Ok(())
    }

    fn handle_payload(&self, payload: TwitchCommandPayload) {
        let mut plugin = self.clone();

        schedule(move |()| match plugin.parse_command(payload.command) {
            Err(error) => {
                Plugin::err(&format!("[nvim-chaos] {}", error));
            }
            _ => {}
        });
    }

    fn parse_command(&mut self, command: TwitchCommand) -> Result<()> {
        match command {
            TwitchCommand::Message(author, text) => {
                self.show_msg(author.as_str(), text.as_str())?;
            }
            TwitchCommand::ColorScheme(colorscheme) => {
                let mode: Mode = ColorSchemeCommand { colorscheme }.into();
                self.set_mode(mode, ModeType::ColorSchemeType, 5 * 60)?;
            }
            TwitchCommand::VimMotionsHell => {
                let mode: Mode = VimMotionsHellCommand {}.into();
                self.set_mode(mode, ModeType::VimMotionsHellType, 60)?;
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
                    (&format!("{}", error), None),
                ];
                let _ = api::echo(chunks, true, &opts);
            }
        }
    }

    pub fn build_api(&mut self) -> Result<Dictionary> {
        let plugin = self.clone();

        let setup = Function::from_fn(move |preferences: Object| {
            let mut plugin = plugin.clone();
            plugin.parse_config(preferences);
        });

        let api = Dictionary::from_iter([("setup", setup)]);

        Ok(api)
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

    fn start_timer(&mut self) {
        let plugin = self.clone();

        let callback = move |_timer: &mut TimerHandle| {
            let mut plugin = plugin.clone();

            schedule(move |()| {
                plugin.update().unwrap();
            });
        };

        let _handle =
            TimerHandle::start(Duration::from_millis(0), Duration::from_secs(1), callback);
    }

    pub fn show_msg(&mut self, author: &str, message: &str) -> Result<()> {
        let mut state = self.state.borrow_mut();
        state.message.show_msg(author, message)?;

        Ok(())
    }
}
