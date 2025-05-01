use crate::core::config::error::Error;

use nvim_oxi::{serde::Deserializer, Object, ObjectKind};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ColorSchemeCommand {
    #[serde(default = "default_colorscheme_command_name")]
    pub name: String,
    #[serde(default = "default_colorscheme_command_duration")]
    pub duration: u32,
    #[serde(default = "default_colorscheme_name")]
    pub default: String,
    #[serde(default = "default_background")]
    pub background: String,
}

fn default_colorscheme_command_name() -> String {
    String::from("!colorscheme")
}

fn default_colorscheme_command_duration() -> u32 {
    60 * 5
}

fn default_colorscheme_name() -> String {
    let scheme = nvim_oxi::api::get_var::<nvim_oxi::String>("colors_name")
        .unwrap_or(nvim_oxi::String::from("retrobox"));
    scheme.to_string()
}

fn default_background() -> String {
    String::from("dark")
}

impl Default for ColorSchemeCommand {
    fn default() -> Self {
        ColorSchemeCommand {
            name: default_colorscheme_command_name(),
            duration: default_colorscheme_command_duration(),
            default: default_colorscheme_name(),
            background: default_background(),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct VimMotionsHellCommand {
    #[serde(default = "default_vimhell_command_name")]
    pub name: String,
    #[serde(default = "default_vimhell_command_duration")]
    pub duration: u32,
}

fn default_vimhell_command_name() -> String {
    String::from("!vimhell")
}

fn default_vimhell_command_duration() -> u32 {
    60
}

impl Default for VimMotionsHellCommand {
    fn default() -> Self {
        VimMotionsHellCommand {
            name: default_vimhell_command_name(),
            duration: default_vimhell_command_duration(),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Commands {
    #[serde(default = "default_message_command_name")]
    pub message: String,

    #[serde(default)]
    pub colorscheme: ColorSchemeCommand,

    #[serde(default)]
    pub hell: VimMotionsHellCommand,
}

fn default_message_command_name() -> String {
    String::from("!msg")
}

impl Default for Commands {
    fn default() -> Self {
        Commands {
            message: default_message_command_name(),
            colorscheme: ColorSchemeCommand::default(),
            hell: VimMotionsHellCommand::default(),
        }
    }
}

#[derive(Default, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub channel: Option<String>,

    #[serde(default)]
    pub commands: Commands,
}

impl TryFrom<Object> for Config {
    type Error = Error;

    fn try_from(preferences: Object) -> Result<Self, Self::Error> {
        if let ObjectKind::Nil = preferences.kind() {
            Ok(Self::default())
        } else {
            let deserializer = Deserializer::new(preferences);
            serde_path_to_error::deserialize::<_, Self>(deserializer).map_err(Into::into)
        }
    }
}
