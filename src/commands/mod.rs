use crate::error::Result;
pub(crate) use color_scheme::{Background, Command as ColorSchemeCommand};
use enum_dispatch::enum_dispatch;
pub(crate) use vim_motions_hell::Command as VimMotionsHellCommand;

mod color_scheme;
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
}

#[derive(Clone, PartialEq, Eq)]
pub enum ModeType {
    VimMotionsHellType,

    ColorSchemeType,
}
