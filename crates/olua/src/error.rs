use std::fmt::Display;

use mlua::ExternalError;
use thiserror::Error as ThisError;

#[derive(Clone, Debug, ThisError)]
#[error(transparent)]
pub struct MluaErrorWithEq(#[from] mlua::Error);

#[derive(Clone, Debug, ThisError)]
pub enum Error {
    #[error("{0}")]
    Access(String),

    #[error("{0}")]
    Initialization(String),

    #[error("{0}")]
    Other(String),

    #[error(transparent)]
    Mlua(#[from] mlua::Error),
}

impl Error {
    pub fn custom<M: Display>(message: M) -> Self {
        Self::Other(message.to_string())
    }

    pub(crate) fn main_thread_only_error(resource: &str) -> Self {
        Self::Access(format!(
            "access to {resource} is only allowed from the main thread"
        ))
    }

    pub(crate) fn missing_init_error() -> Self {
        Self::Initialization(
            "nvim-oximlua must be initialized before use".to_string(),
        )
    }

    pub(crate) fn multiple_init_error() -> Self {
        Self::Initialization(
            "nvim-oximlua can be initialized only once".to_string(),
        )
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Access(l), Self::Access(r))
            | (Self::Initialization(l), Self::Initialization(r))
            | (Self::Other(l), Self::Other(r)) => l == r,
            (Self::Mlua(l), Self::Mlua(r)) => l.to_string() == r.to_string(),
            _ => false,
        }
    }
}

impl Eq for Error {}

impl From<Error> for mlua::Error {
    fn from(err: Error) -> Self {
        err.into_lua_err()
    }
}
