use std::{fmt::Display, str::FromStr};

use types::{
    Object,
    conversion::{self, FromObject, ToObject},
};

use crate::types::RegisterType;

#[derive(Clone, Debug, Default, PartialEq, macros::OptsBuilder)]
pub struct SetregOpts {
    set_unnamed: bool,
    append: bool,
    reg_type: RegisterType,
}

impl Display for SetregOpts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.set_unnamed {
            write!(f, "u")?;
        };
        if self.append {
            write!(f, "a")?;
        };
        write!(f, "{}", self.reg_type)
    }
}

impl FromStr for SetregOpts {
    type Err = conversion::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set_unnamed = false;
        let mut append = false;
        let mut reg_type: Vec<u8> = Vec::with_capacity(s.len());

        for byte in s.bytes() {
            match byte {
                b'u' => set_unnamed = true,
                b'a' => append = true,
                _ => reg_type.push(byte),
            }
        }

        let reg_type = String::from_utf8_lossy(&reg_type).parse()?;

        Ok(Self { set_unnamed, append, reg_type })
    }
}

impl TryFrom<&str> for SetregOpts {
    type Error = conversion::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for SetregOpts {
    type Error = conversion::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<SetregOpts> for String {
    fn from(value: SetregOpts) -> Self {
        value.to_string()
    }
}

impl FromObject for SetregOpts {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        String::from_object(obj)?.parse()
    }
}

impl ToObject for SetregOpts {
    fn to_object(self) -> Result<Object, conversion::Error> {
        self.to_string().to_object()
    }
}
