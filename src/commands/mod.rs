use enum_dispatch::enum_dispatch;
use nvim_oxi::{
    api::{self, opts::ParseCmdOpts},
    Result,
};

#[enum_dispatch(Mode)]
pub trait ModeCommand {
    fn start(&self) -> Result<()>;

    fn stop(&self) -> Result<()>;

    fn name(&self) -> String;
}

#[enum_dispatch]
#[derive(PartialEq, Clone, Debug, strum::Display)]
pub enum Mode {
    VimMotionsHellCommand,

    ColorSchemeCommand,
}

#[derive(Clone, PartialEq, Eq)]
pub enum ModeType {
    VimMotionsHellType,

    ColorSchemeType,
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct ColorSchemeCommand {
    pub colorscheme: String,
}

impl ModeCommand for ColorSchemeCommand {
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
        }

        Ok(())
    }

    fn stop(&self) -> Result<()> {
        api::command("colorscheme vscode")?;
        api::command("set background=dark")?;

        Ok(())
    }

    fn name(&self) -> String {
        format!("Color Scheme: {}", self.colorscheme)
    }
}

#[derive(Default, PartialEq, Clone, Debug)]
pub struct VimMotionsHellCommand {}

impl ModeCommand for VimMotionsHellCommand {
    fn start(&self) -> Result<()> {
        api::command("noremap l h")?;
        api::command("noremap k j")?;
        api::command("noremap j k")?;
        api::command("noremap h l")?;

        Ok(())
    }

    fn stop(&self) -> Result<()> {
        api::command("noremap l l")?;
        api::command("noremap k k")?;
        api::command("noremap j j")?;
        api::command("noremap h h")?;

        Ok(())
    }

    fn name(&self) -> String {
        String::from("Vim Motions Hell")
    }
}
