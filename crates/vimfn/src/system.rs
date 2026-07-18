use std::path::PathBuf;

use api::SuperIterator;
use api::opts::*;
use types::conversion::FromObject;
use types::{Array, Object, ObjectKind, String as NvimString};

use crate::Result;

/// Binding to [`stdpath()`][1].
///
/// Returns locations of various default files and directories.
///
/// [1]: https://neovim.io/doc/user/vimfn/#stdpath()
pub fn stdpath(what: StdPath) -> Result<impl SuperIterator<PathBuf> + use<>> {
    let vim_func_name = "stdpath";
    let args = (NvimString::from(what),);
    let ret = api::call_function::<_, Object>(vim_func_name, args)?;
    let arr = match ret.kind() {
        ObjectKind::String => Array::from_iter([ret]),
        ObjectKind::Array => unsafe { ret.into_array_unchecked() },
        _ => unreachable!("`stdpath` should return string or list of strings"),
    };
    Ok(arr.into_iter().map(|obj| {
        PathBuf::from(
            types::String::from_object(obj)
                .expect("All items returned by `stdpath` should be strings."),
        )
    }))
}
