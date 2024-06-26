use nvim_oxi::{
    api::{
        self,
        opts::OptionOpts,
        types::{WindowConfig, WindowRelativeTo, WindowTitle},
        Buffer, Window,
    },
    Result,
};

#[derive(Clone)]
pub struct State {
    pub win: Option<Window>,
    pub buf: Buffer,
}

impl Default for State {
    fn default() -> Self {
        Self {
            buf: 0.into(),
            win: None,
        }
    }
}

impl State {
    pub fn init(&mut self) -> Result<()> {
        self.buf = api::create_buf(false, true)?;

        Ok(())
    }

    pub fn show_msg(&mut self, author: &str, message: &str) -> Result<()> {
        let mut i = 0;
        let width = 31;

        let msg = message.chars().fold(vec![], move |mut result, item| {
            if i % width == 0 {
                result.push(String::new());
            }

            let last = result.last_mut().unwrap();
            last.push(item);

            i += 1;

            result
        });

        let height: u32 = msg.clone().len().try_into().unwrap_or(0);

        self.buf.set_lines(0..1, false, msg)?;

        let opts = OptionOpts::builder()
            .scope(api::opts::OptionScope::Global)
            .build();

        let cols = api::get_option_value::<u32>("columns", &opts)?;
        let rows = api::get_option_value::<u32>("lines", &opts)?;

        let x = (cols / 2) - (width + 2) / 2;
        let y = (rows / 2) - (height + 2) / 2;

        let title_string = nvim_oxi::String::from(author);

        let title = WindowTitle::SimpleString(title_string);

        let config = WindowConfig::builder()
            .relative(WindowRelativeTo::Editor)
            .border(nvim_oxi::api::types::WindowBorder::Rounded)
            .style(nvim_oxi::api::types::WindowStyle::Minimal)
            .height(height)
            .width(width)
            .title(title)
            .col(x)
            .row(y)
            .focusable(true)
            .build();

        if self.win.is_some() {
            if let Some(win) = &mut self.win {
                if win.is_valid() {
                    win.set_config(&config)?;
                } else {
                    let win = api::open_win(&self.buf, false, &config)?;
                    self.win = Some(win);
                }
            }
        } else {
            let win = api::open_win(&self.buf, false, &config)?;
            self.win = Some(win);
        }

        if let Some(window) = &self.win {
            api::set_current_win(window)?;
        }

        Ok(())
    }
}
