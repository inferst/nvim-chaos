use std::fmt::Display;
use std::str::FromStr;

use nvim_oxi::api::opts::SetHighlightOptsBuilder;
use nvim_oxi::schedule;

use nvim_oxi::{
    api::{self, opts::ParseCmdOpts},
    Array,
};

use crate::core::plugin::CONFIG;

use super::ModeCommand;

use crate::error::Result;

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) enum Background {
    #[default]
    Default,
    Dark,
    Light,
}

impl FromStr for Background {
    type Err = ();

    fn from_str(value: &str) -> core::result::Result<Self, ()> {
        match value {
            "dark" => Ok(Background::Dark),
            "light" => Ok(Background::Light),
            _ => Ok(Background::Default),
        }
    }
}

impl Display for Background {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Background::Dark => String::from("dark"),
            Background::Light => String::from("light"),
            Background::Default => String::new(),
        };

        write!(f, "{string}")
    }
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Command {
    pub colorscheme: String,
    pub background: Background,
}

impl ModeCommand for Command {
    fn start(&self) -> Result<()> {
        let mut command = String::from("colorscheme ");
        let mut cmd = command.clone();
        command.push_str(&self.colorscheme);

        let opts = ParseCmdOpts::builder().build();

        let infos = api::parse_cmd(&command, &opts)?;
        let arg = infos.args.first();

        if let Some(arg) = arg {
            cmd.push_str(arg);
            api::command(&cmd)?;

            // Some default vim color schemes have ugly background for floating windows
            schedule(move |()| {
                let highlight_opts = SetHighlightOptsBuilder::default().link("Float").build();
                api::set_hl(0, "NormalFloat", &highlight_opts).unwrap();
            });

            if self.background != Background::Default {
                api::command(&format!("set background={}", self.background))?;
            }
        }

        Ok(())
    }

    fn is_valid(&self) -> Result<bool> {
        let schemes: Array = api::call_function("getcompletion", Array::from_iter(["", "color"]))?;

        for scheme in schemes {
            unsafe {
                if scheme.into_string_unchecked() == self.colorscheme {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    fn stop(&self) -> Result<()> {
        let config = CONFIG.get().unwrap();
        let colorscheme = config.commands.colorscheme.clone();
        let scheme = colorscheme.default;
        let background = colorscheme.background;

        api::command(&format!("colorscheme {scheme}"))?;
        api::command(&format!("set background={background}"))?;

        Ok(())
    }

    fn name(&self) -> String {
        format!("Color Scheme - {}", self.colorscheme)
    }
}
