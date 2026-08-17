//! This example shows how to export Rust functions and values to Lua.
//!
//! With this plugin/config in place the following should work:
//!
//! ```lua
//! local calc = require("calc")
//!
//! assert(calc.answer == calc.add(1, 41))
//! assert(calc.answer == calc.multiply(2, 21))
//! assert(calc.answer == calc.compute(function(a, b) return a + b; end, 1, 41))
//! assert(calc.answer == calc.compute(function(a, b) return a * b; end, 2, 21))
//! ```

use mlua::Table;
use nvim::mlua;
use nvim_oximlua as nvim;

const THE_ANSWER: i32 = 42;

#[mlua::lua_module]
fn calc(lua: &mlua::Lua) -> mlua::Result<Table> {
    nvim::init(lua)?;

    let add = nvim::Function::from_fn(|(a, b): (i32, i32)| a + b);

    let multiply = nvim::Function::from_fn(|(a, b): (i32, i32)| a * b);

    let compute = nvim::Function::from_fn(
        |(fun, a, b): (nvim::Function<(i32, i32), i32>, i32, i32)| {
            fun.call((a, b)).unwrap()
        },
    );

    let calc = lua.create_table()?;
    calc.set("the_answer", THE_ANSWER)?;
    calc.set("add", add)?;
    calc.set("multiply", multiply)?;
    calc.set("compute", compute)?;

    Ok(calc)
}
