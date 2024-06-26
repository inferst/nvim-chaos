use nvim_oxi::{
    serde::{DeserializeError, Deserializer},
    Object, ObjectKind,
};
use serde::Deserialize;
use thiserror::Error as ThisError;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Commmands {
    #[serde(default)]
    pub message: String,

    #[serde(default)]
    pub colorscheme: String,

    #[serde(default)]
    pub hell: String,
}

impl Default for Commmands {
    fn default() -> Self {
        Commmands {
            message: "!msg".to_owned(),
            colorscheme: "!colorscheme".to_owned(),
            hell: "!vimhell".to_owned(),
        }
    }
}

#[derive(Default, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub channel: Option<String>,

    #[serde(default)]
    pub commands: Commmands,
}

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("error parsing `{prefix}{option}`: {why}")]
    BadConfig {
        prefix: String,
        option: serde_path_to_error::Path,
        why: String,
    },
}

impl From<serde_path_to_error::Error<DeserializeError>> for Error {
    fn from(err: serde_path_to_error::Error<DeserializeError>) -> Self {
        Self::BadConfig {
            prefix: String::new(),
            option: err.path().to_owned(),
            why: err.into_inner().to_string(),
        }
    }
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
