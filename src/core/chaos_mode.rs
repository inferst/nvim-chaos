use nvim_oxi::{
    api::{
        self,
        opts::OptionOpts,
        types::{WindowConfig, WindowRelativeTo, WindowTitle, WindowTitlePosition},
        Buffer, Window,
    },
    Result,
};

use crate::commands::{Mode, ModeCommand, ModeType};

#[derive(Clone)]
pub struct ModeState {
    pub mode: Mode,
    pub mode_type: ModeType,
    pub seconds: u32,
}

#[derive(Clone)]
pub struct State {
    pub buf: Buffer,
    pub win: Option<Window>,
    pub commands: Vec<ModeState>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            buf: 0.into(),
            win: None,
            commands: Vec::default(),
        }
    }
}

impl State {
    pub fn init(&mut self) -> Result<()> {
        self.buf = api::create_buf(false, true)?;

        Ok(())
    }

    pub fn set_mode(&mut self, mode: Mode, mode_type: ModeType, seconds: u32) -> Result<()> {
        if mode.is_valid()? {
            self.commands.retain(|x| x.mode_type != mode_type);

            mode.start()?;
            self.commands.push(ModeState {
                mode,
                mode_type,
                seconds,
            });

            self.update()?;
        }

        Ok(())
    }

    pub fn tick(&mut self) -> Result<()> {
        let commands = self.commands.iter();

        for command in commands {
            if command.seconds == 0 {
                command.mode.stop()?;
            }
        }

        self.commands.retain(|x| x.seconds != 0);

        let count = self.commands.len();

        let commands = self.commands.iter_mut();

        for command in commands {
            let seconds = command.seconds;

            if seconds > 0 {
                command.seconds = seconds - 1;
            }
        }

        if count == 0 {
            self.close_win()?;
        } else {
            self.update()?;
        }

        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        let lines: Vec<String> = self
            .commands
            .iter_mut()
            .map(|x| {
                let seconds = x.seconds;

                let minutes = seconds / 60;
                let seconds = seconds % 60;

                let mode = format!("  {:0>2}:{:0>2}  {}  ", minutes, seconds, x.mode.name());

                mode
            })
            .collect();

        let width: u32 = lines
            .clone()
            .into_iter()
            .fold(0, |result, item| {
                if item.len() > result {
                    item.len()
                } else {
                    result
                }
            })
            .try_into()
            .unwrap();

        let mut lines: Vec<String> = lines;

        lines.insert(0, String::new());
        lines.push(String::new());

        let height: u32 = lines.len().try_into().unwrap();

        self.buf.set_lines(0..lines.len(), false, lines)?;

        self.open_win(width, height)?;

        Ok(())
    }

    pub fn close_win(&mut self) -> Result<()> {
        if let Some(win) = self.win.take() {
            win.close(false)?;
        }

        Ok(())
    }

    pub fn open_win(&mut self, width: u32, height: u32) -> Result<()> {
        let title = WindowTitle::SimpleString(nvim_oxi::String::from("Chaos Neovim"));
        let title_pos = WindowTitlePosition::Center;

        let opts = OptionOpts::builder()
            .scope(api::opts::OptionScope::Global)
            .build();

        let cols = api::get_option_value::<u32>("columns", &opts)?;

        let x = cols - 4 - width;
        let y = 1;

        let config = WindowConfig::builder()
            .relative(WindowRelativeTo::Editor)
            .border(nvim_oxi::api::types::WindowBorder::Rounded)
            .style(nvim_oxi::api::types::WindowStyle::Minimal)
            .title(title)
            .title_pos(title_pos)
            .width(width)
            .height(height)
            .col(x)
            .row(y)
            .build();

        if let Some(win) = &mut self.win {
            if win.is_valid() {
                win.set_config(&config)?;
            } else {
                let win = api::open_win(&self.buf, false, &config)?;
                self.win = Some(win);
            }
        } else {
            let win = api::open_win(&self.buf, false, &config)?;
            self.win = Some(win);
        }

        Ok(())
    }
}
