pub(crate) use color_scheme::{Background, Command as ColorSchemeCommand};
use enum_dispatch::enum_dispatch;
use nvim_oxi::Result;
pub(crate) use vim_motions_hell::Command as VimMotionsHellCommand;
pub(crate) use cursor_line::Command as CursorLineCommand;

mod color_scheme;
mod cursor_line;
mod vim_motions_hell;

#[enum_dispatch(Mode)]
pub trait ModeCommand {
    fn start(&self) -> Result<()>;

    fn stop(&self) -> Result<()>;

    fn name(&self) -> String;

    fn is_valid(&self) -> Result<bool>;
}

#[enum_dispatch]
#[derive(PartialEq, Clone, Debug)]
pub enum Mode {
    VimMotionsHellCommand,

    ColorSchemeCommand,

    CursorLineCommand,
}

#[derive(Clone, PartialEq, Eq)]
pub enum ModeType {
    VimMotionsHellType,

    ColorSchemeType,

    CursorLineType,
}
