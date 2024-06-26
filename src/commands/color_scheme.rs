use std::fmt::Display;
use std::str::FromStr;

use nvim_oxi as nvim;

use nvim_oxi::{
    api::{self, opts::ParseCmdOpts},
    Array,
};

use super::ModeCommand;

#[derive(Default, Debug, Clone, PartialEq)]
pub(crate) enum Background {
    #[default]
    Default,
    Dark,
    Light,
}

impl FromStr for Background {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, ()> {
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
    fn start(&self) -> nvim::Result<()> {
        let mut command = String::from("colorscheme ");
        let mut cmd = command.clone();
        command.push_str(&self.colorscheme);

        let opts = ParseCmdOpts::builder().build();

        let infos = api::parse_cmd(&command, &opts)?;
        let arg = infos.args.first();

        if let Some(arg) = arg {
            cmd.push_str(arg);
            api::command(&cmd)?;

            if self.background != Background::Default {
                api::command(&format!("set background={}", self.background))?;
            }
        }

        Ok(())
    }

    fn is_valid(&self) -> nvim::Result<bool> {
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

    fn stop(&self) -> nvim::Result<()> {
        api::command("colorscheme vscode")?;
        api::command("set background=dark")?;

        Ok(())
    }

    fn name(&self) -> String {
        format!("Color Scheme - {}", self.colorscheme)
    }
}
