use std::path::PathBuf;

use serde::Deserialize;
use types::Object;

use crate::Buffer;

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct AutocmdCallbackArgs {
    /// The `Buffer` specified by `<abuf>`.
    #[serde(rename = "buf")]
    pub buffer: Buffer,

    /// Arbitrary data passed to
    /// [`nvim_oximlua::api::exec_autocmds`](crate::exec_autocmds).
    #[serde(default)]
    pub data: Object,

    /// The name of the event that triggered the autocommand.
    pub event: String,

    /// The expanded value of `<afile>`.
    pub file: PathBuf,

    /// The `id` of the autocommand group that the autocommand belongs to, if
    /// any.
    #[serde(default)]
    pub group: Option<u32>,

    /// The `id` of the autocommand.
    pub id: u32,

    /// The expanded value of `<amatch>`.
    pub r#match: String,
}

impl mlua::FromLua for AutocmdCallbackArgs {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        Self::deserialize(mlua::serde::Deserializer::new(value))
    }
}
