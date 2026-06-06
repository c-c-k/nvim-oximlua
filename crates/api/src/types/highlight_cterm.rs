use macros::OptsBuilder;
use serde::{Deserialize, Serialize};
use types::{
    Dictionary,
    Object,
    conversion::{self, ToObject},
    serde::Serializer,
};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize, OptsBuilder,
)]
pub struct HighlightCterm {
    #[builder(argtype = "bool", inline = "Some({0})")]
    pub altfont: Option<bool>,

    #[builder(argtype = "bool", inline = "Some({0})")]
    pub blink: Option<bool>,

    #[builder(argtype = "bool", inline = "Some({0})")]
    pub bold: Option<bool>,

    #[builder(argtype = "bool", inline = "Some({0})")]
    pub conceal: Option<bool>,

    #[builder(argtype = "bool", inline = "Some({0})")]
    pub dim: Option<bool>,

    #[builder(argtype = "bool", inline = "Some({0})")]
    pub italic: Option<bool>,

    #[builder(argtype = "bool", inline = "Some({0})")]
    pub nocombine: Option<bool>,

    #[builder(argtype = "bool", inline = "Some({0})")]
    pub overline: Option<bool>,

    #[builder(argtype = "bool", inline = "Some({0})")]
    pub reverse: Option<bool>,

    #[builder(argtype = "bool", inline = "Some({0})")]
    pub standout: Option<bool>,

    #[builder(argtype = "bool", inline = "Some({0})")]
    pub strikethrough: Option<bool>,

    #[builder(argtype = "bool", inline = "Some({0})")]
    pub undercurl: Option<bool>,

    #[builder(argtype = "bool", inline = "Some({0})")]
    pub underdashed: Option<bool>,

    #[builder(argtype = "bool", inline = "Some({0})")]
    pub underdotted: Option<bool>,

    #[builder(argtype = "bool", inline = "Some({0})")]
    pub underdouble: Option<bool>,

    #[builder(argtype = "bool", inline = "Some({0})")]
    pub underline: Option<bool>,
}

impl ToObject for HighlightCterm {
    fn to_object(self) -> Result<Object, conversion::Error> {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}

impl From<HighlightCterm> for Dictionary {
    fn from(c: HighlightCterm) -> Self {
        Self::try_from(
            c.to_object().expect("HighlightCterm.to_object() failed"),
        )
        .expect("Dictionary::try_from(HighlightCterm.to_object()) failed")
    }
}
