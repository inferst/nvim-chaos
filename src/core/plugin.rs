use std::io::Cursor;
use std::sync::OnceLock;
use std::{cell::RefCell, rc::Rc, str::FromStr, thread, time::Duration};

use nvim_oxi::{
    api::{self, opts::EchoOpts},
    libuv::{AsyncHandle, TimerHandle},
    schedule, Dictionary, Function, Object, Result,
};

use rodio::Decoder;
use tokio::sync::mpsc;

use crate::{
    commands::{Background, ColorSchemeCommand, Mode, ModeType, VimMotionsHellCommand},
    twitch::{self},
};

use super::{
    chaos_mode::{self},
    config::Config,
};

pub static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Clone, Default)]
pub struct State {
    pub chaos_mode: chaos_mode::State,
}

#[derive(Clone, Default)]
pub struct Plugin {
    pub state: Rc<RefCell<State>>,
}

impl Plugin {
    pub fn init(&mut self) -> Result<()> {
        let config = CONFIG.get().unwrap();

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

        let config = config.clone();

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
        let config = CONFIG.get().unwrap();

        match command {
            twitch::Command::Message(author, text) => {
                Plugin::show_msg(author.as_str(), text.as_str());
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
                    config.commands.colorscheme.duration,
                )?;
            }
            twitch::Command::VimMotionsHell => {
                let mode: Mode = VimMotionsHellCommand {}.into();
                self.set_mode(
                    mode,
                    ModeType::VimMotionsHellType,
                    config.commands.hell.duration,
                )?;
            }
        }

        Ok(())
    }

    fn parse_config(&mut self, preferences: Object) {
        let config = Config::try_from(preferences);

        match config {
            Ok(config) => {
                CONFIG.set(config).unwrap();
                self.init().unwrap();
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

    pub fn show_msg(author: &str, message: &str) {
        let mut option_opts = Dictionary::new();

        option_opts.insert("title", author);
        option_opts.insert("timeout", 20000);

        let message = wrap_text(message, 40);

        schedule(move |()| {
            api::notify(&message, api::types::LogLevel::Off, &option_opts).unwrap();
        });

        let _ = thread::spawn(move || {
            let stream = rodio::OutputStream::try_default();

            match stream {
                Ok((_stream, stream_handle)) => {
                    let data = include_bytes!("../../audio/icq.mp4");
                    let buff = Cursor::new(data);
                    let source = Decoder::new(buff);

                    match source {
                        Ok(source) => {
                            let sink = rodio::Sink::try_new(&stream_handle).unwrap();

                            sink.set_volume(0.1);
                            sink.append(source);
                            sink.sleep_until_end();
                        }
                        Err(err) => {
                            Plugin::err(&format!("[nvim-chaos] {err}"));
                        }
                    }
                }
                Err(err) => {
                    Plugin::err(&format!("[nvim-chaos] {err}"));
                }
            }
        });
    }
}

fn wrap_text(input: &str, max_len: usize) -> String {
    let mut result = String::new();
    let mut line_len = 0;

    for word in input.split_whitespace() {
        if line_len + word.len() + usize::from(line_len > 0) > max_len {
            result.push_str(&format!("\n{word}"));
            line_len = word.len();
        } else {
            if line_len > 0 {
                result.push(' ');
                line_len += 1;
            }
            result.push_str(word);
            line_len += word.len();
        }
    }

    result
}
