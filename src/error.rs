use thiserror::Error as ThisError;

/// `nvim-oximlua`'s result type.
pub type Result<T> = std::result::Result<T, Error>;

/// `nvim-oximlua`'s error type.
#[derive(Clone, Debug, ThisError)]
// TODO: add derive(Eq, PartialEq)
pub enum Error {
    #[error(transparent)]
    Api(#[from] api::Error),

    #[error(transparent)]
    Nvim(#[from] types::Error),

    #[error(transparent)]
    ObjectConversion(#[from] types::conversion::Error),

    #[error(transparent)]
    Serialize(#[from] types::serde::SerializeError),

    #[error(transparent)]
    Deserialize(#[from] types::serde::DeserializeError),

    #[cfg(feature = "libuv")]
    #[error(transparent)]
    Libuv(#[from] libuv::Error),

    #[error(transparent)]
    Mlua(#[from] mlua::Error),

    #[error(transparent)]
    OxiMlua(#[from] olua::Error),
}
