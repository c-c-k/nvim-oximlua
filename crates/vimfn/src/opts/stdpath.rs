use serde::Serialize;
use types::conversion::FromObject;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StdPath {
    Cache,
    Config,
    ConfigDirs,
    Data,
    DataDirs,
    Log,
    Run,
    State,
}

impl From<StdPath> for types::String {
    #[inline]
    fn from(ctx: StdPath) -> Self {
        types::String::from_object(
            ctx.serialize(types::serde::Serializer::new())
                .expect("`StdPath` is serializable"),
        )
        .expect("`StdPath` is serialized into a string")
    }
}
