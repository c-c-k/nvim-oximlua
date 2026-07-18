use macros::OptsBuilder;
use serde::{Deserialize, Serialize, ser};
use types::{
    Object,
    String as NvimString,
    conversion::{self, FromObject, ToObject},
    serde::{Deserializer, Serializer},
};

use crate::types::RegisterType;

#[derive(
    Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize, OptsBuilder,
)]
pub struct RegisterInfos {
    #[serde(serialize_with = "serialize_as_vec_vec_bytes")]
    pub regcontents: Vec<NvimString>,

    pub regtype: RegisterType,
    pub isunnamed: bool,
    pub points_to: bool,
    // #[builder(argtype = "bool", inline = "Some({0})")]
    // pub conceal: Option<bool>,
}

impl ToObject for RegisterInfos {
    fn to_object(self) -> Result<Object, conversion::Error> {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}

impl FromObject for RegisterInfos {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

fn serialize_as_vec_vec_bytes<S>(
    v: &[NvimString],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    serializer.collect_seq(v.iter().map(|s| s.as_bytes()))
}
