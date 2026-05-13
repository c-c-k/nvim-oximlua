use thiserror::Error as ThisError;

/// `nvim-oxi`'s result type.
pub type Result<T> = std::result::Result<T, Error>;

/// `nvim-oxi`'s error type.
#[derive(Clone, Debug, ThisError)]
#[cfg_attr(
    not(any(feature = "mlua", feature = "oximlua")),
    derive(Eq, PartialEq)
)]
pub enum Error {
    #[cfg(not(feature = "oximlua"))]
    #[error(transparent)]
    Lua(#[from] luajit::Error),

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

    #[cfg(any(feature = "mlua", feature = "oximlua"))]
    #[error(transparent)]
    Mlua(#[from] mlua::Error),

    #[cfg(feature = "oximlua")]
    #[error(transparent)]
    OxiMlua(#[from] oximlua::Error),
}
