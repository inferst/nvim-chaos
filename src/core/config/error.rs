use nvim_oxi::serde::DeserializeError;

use thiserror::Error as ThisError;

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
            option: err.path().clone(),
            why: err.into_inner().to_string(),
        }
    }
}
