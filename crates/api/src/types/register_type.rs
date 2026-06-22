use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use types::{
    Object,
    String as NvimString,
    conversion::{self, FromObject, ToObject},
};

#[non_exhaustive]
// #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
#[derive(
    Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize,
)]
#[serde(into = "String", try_from = "String")]
pub enum RegisterType {
    #[default]
    Guess,
    Char,
    Line,
    Block(Option<usize>),
}

impl Display for RegisterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegisterType::Guess => Ok(()),
            RegisterType::Char => write!(f, "c"),
            RegisterType::Line => write!(f, "l"),
            RegisterType::Block(None) => write!(f, "b"),
            RegisterType::Block(Some(width)) => {
                write!(f, "b{width}")
            },
        }
    }
}

impl FromStr for RegisterType {
    type Err = conversion::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(RegisterType::Guess),
            "v" | "c" => Ok(RegisterType::Char),
            "V" | "l" => Ok(RegisterType::Line),
            _ if let Some(width) = s.strip_prefix(['b', '\x16']) => {
                match width {
                    width if width.is_empty() => Ok(RegisterType::Block(None)),
                    width if let Ok(width) = width.parse::<usize>() => {
                        Ok(RegisterType::Block(Some(width)))
                    },
                    _ => Err(conversion::Error::Other(format!(
                        "invalid block width suffix: \"{width}\""
                    ))),
                }
            },
            _ => Err(conversion::Error::Other(format!(
                "invalid register type string: \"{s}\""
            ))),
        }
    }
}

impl TryFrom<&str> for RegisterType {
    type Error = conversion::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for RegisterType {
    type Error = conversion::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<RegisterType> for String {
    fn from(value: RegisterType) -> Self {
        value.to_string()
    }
}

impl From<RegisterType> for NvimString {
    fn from(value: RegisterType) -> Self {
        Self::from(value.to_string())
    }
}

impl FromObject for RegisterType {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        String::from_object(obj)?.parse()
    }
}

impl ToObject for RegisterType {
    fn to_object(self) -> Result<Object, conversion::Error> {
        self.to_string().to_object()
    }
}
