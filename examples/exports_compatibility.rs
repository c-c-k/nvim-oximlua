//! This example shows how to initially transition from an
//! `nvim_oxi-oxi` entry point to a `nvim-oximlua` one.

// ADD: `mlua` import.
use nvim_oxi::mlua;
use nvim_oxi::{Dictionary, Function, Object};
// ADD: Alias `nvim-oximlua` to `nvim-oxi`, alternatively this might be
//      more convenient to do via the package name in `cargo.toml`.
use nvim_oximlua as nvim_oxi;

// REPLACE: `#[nvim_oxi::plugin]` macro with `#[mlua::lua_module]`.
#[mlua::lua_module]
// ADD: `lua` argument and `mlua::Result` to the entry point signature
//      (nvim-oxi signature: `fn calc() -> Dictionary {`)
fn calc(lua: &mlua::Lua) -> mlua::Result<Dictionary> {
    // ADD: `nvim-oximlua` initialization.
    nvim_oxi::init(lua)?;

    let add = Function::from_fn(|(a, b): (i32, i32)| a + b);

    let multiply = Function::from_fn(|(a, b): (i32, i32)| a * b);

    let compute = Function::from_fn(
        |(fun, a, b): (Function<(i32, i32), i32>, i32, i32)| {
            fun.call((a, b)).unwrap()
        },
    );

    let calc = Dictionary::from_iter([
        ("add", Object::from(add)),
        ("multiply", Object::from(multiply)),
        ("compute", Object::from(compute)),
    ]);

    // ADD: Wrap the return type in `Ok<...>`.
    Ok(calc)
}
