use api::SuperIterator;
use api::types::{RegisterInfos, RegisterType};
use types::conversion::{FromObject, ToObject};
use types::{Array, Object, ObjectKind, String as NvimString};

use crate::Error;
use crate::Result;

/// Binding to [`getreg()`][1].
///
/// TODO: copy and adjust documentation
///
/// [1]: https://neovim.io/doc/user/vimfn/#getreg()
pub fn getreg(
    reg_name: Option<String>,
    as_raw_expr: bool,
    split_lines: bool,
) -> Result<impl SuperIterator<NvimString> + use<>> {
    let vim_func_name = "getreg";
    let args = (reg_name.to_object()?, as_raw_expr, split_lines);
    let ret = api::call_function::<_, Object>(vim_func_name, args)?;
    let arr = match ret.kind() {
        ObjectKind::String => Array::from_iter([ret]),
        ObjectKind::Array => unsafe { ret.into_array_unchecked() },
        _ => unreachable!("`getreg` should return string or list of strings"),
    };
    Ok(arr.into_iter().map(|obj| {
        NvimString::from_object(obj)
            .expect("All items returned by `getreg` should be strings.")
    }))
}

/// Binding to [`getreginfo()`][1].
///
/// TODO: copy and adjust documentation
///
/// [1]: https://neovim.io/doc/user/vimfn/#getreginfo()
pub fn getreginfo(reg_name: Option<String>) -> Result<RegisterInfos> {
    let vim_func_name = "getreginfo";
    let args = (reg_name.to_object()?,);
    let ret = api::call_function::<_, Object>(vim_func_name, args)?;
    Ok(RegisterInfos::from_object(ret)?)
}

/// Binding to [`getregtype()`][1].
///
/// TODO: copy and adjust documentation
///
/// [1]: https://neovim.io/doc/user/vimfn/#getregtype()
pub fn getregtype(reg_name: Option<String>) -> Result<RegisterType> {
    let vim_func_name = "getregtype";
    let args = (reg_name.to_object()?,);
    let ret = api::call_function::<_, Object>(vim_func_name, args)?;
    Ok(RegisterType::from_object(ret)?)
}

/// Binding to [`setreg()`][1].
///
/// TODO: copy and adjust documentation
///
/// [1]: https://neovim.io/doc/user/vimfn/#setreg()
pub fn setreg() -> Result<()> {
    todo!()
}
