use rodio::{decoder::DecoderError, PlayError, StreamError};
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    Nvim(#[from] nvim_oxi::Error),

    #[error(transparent)]
    Libuv(#[from] nvim_oxi::libuv::Error),

    #[error(transparent)]
    Api(#[from] nvim_oxi::api::Error),

    #[error(transparent)]
    Stream(#[from] StreamError),

    #[error(transparent)]
    Decoder(#[from] DecoderError),

    #[error(transparent)]
    Play(#[from] PlayError),
}
