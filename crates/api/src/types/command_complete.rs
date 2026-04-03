use serde::{Deserialize, Serialize, de::Error};
use types::{
    Function,
    Object,
    conversion::{self, FromObject, ToObject},
    serde::{Deserializer, Serializer},
};

use crate::ToFunction;

pub type CompleteCallbackArgs = (String, String, usize);
pub type CompleteCallbackRet = Vec<String>;
pub type CompleteCallbackFunc =
    Function<CompleteCallbackArgs, CompleteCallbackRet>;

/// See `:h command-complete` for details.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandComplete {
    Arglist,
    Augroup,
    Buffer,
    Breakpoint,
    Color,
    Command,
    Compiler,
    DiffBuffer,
    Dir,
    DirInPath,
    Environment,
    Event,
    Expression,
    File,
    FileInPath,
    Filetype,
    Function,
    Help,
    Highlight,
    History,
    Keymap,
    Locale,
    Lua,
    Mapclear,
    Mapping,
    Menu,
    Messages,
    Option,
    Packadd,
    #[cfg(feature = "neovim-0-12")] // On 0.12 and Nightly
    Retab,
    Runtime,
    Scriptnames,
    Shellcmd,
    Shellcmdline,
    Sign,
    Syntax,
    Syntime,
    Tag,
    TagListfiles,
    User,
    Var,

    /// See `:h command-completion-customlist` for details.
    /// *Note*: This variant contains a nvim_oxi::Function.
    #[serde(untagged, deserialize_with = "deserialize_callback")]
    Callback(CompleteCallbackFunc),

    /// See `:h command-completion-customlist` for details.
    /// *Note*: This variant contains the name of a Vim Script function.
    #[serde(
        untagged,
        serialize_with = "serialize_customlist",
        deserialize_with = "deserialize_customlist"
    )]
    Customlist(String),

    /// See `:h command-completion-custom` for details.
    /// *Note*: This variant contains the name of a Vim Script function.
    #[serde(
        untagged,
        serialize_with = "serialize_custom",
        deserialize_with = "deserialize_custom"
    )]
    Custom(String),
}

impl ToObject for CommandComplete {
    fn to_object(self) -> Result<Object, conversion::Error> {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}

impl FromObject for CommandComplete {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

pub trait CommandCompleteOrFunction {
    fn to_object(self) -> Object;
}

impl<T> CommandCompleteOrFunction for T
where
    T: ToFunction<CompleteCallbackArgs, CompleteCallbackRet>,
{
    fn to_object(self) -> Object {
        Object::from_luaref(self.into_luaref())
    }
}

impl CommandCompleteOrFunction for CommandComplete {
    fn to_object(self) -> Object {
        ToObject::to_object(self).unwrap()
    }
}

fn deserialize_callback<'de, D>(
    deserializer: D,
) -> Result<CompleteCallbackFunc, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    CompleteCallbackFunc::deserialize(deserializer)
}

macro_rules! serde_complete_custom {
    ($ser_fn_name:ident, $de_fn_name:ident, $variant:literal) => {
        fn $ser_fn_name<S>(
            vim_fn_name: &str,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            serializer.serialize_str(&[$variant, vim_fn_name].join(","))
        }

        fn $de_fn_name<'de, D>(deserializer: D) -> Result<String, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            let value = String::deserialize(deserializer)?;
            if let Some(remainder) = value.strip_prefix($variant) {
                if remainder.is_empty() {
                    return Ok(String::new());
                }
                if let Some(vim_fn_name) = remainder.strip_prefix(",") {
                    return Ok(vim_fn_name.to_string());
                }
            }
            Err(D::Error::custom("not custom or customlist"))
        }
    };
}

serde_complete_custom! {serialize_custom, deserialize_custom, "custom"}
serde_complete_custom! {serialize_customlist, deserialize_customlist, "customlist"}
