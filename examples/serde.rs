//! For idiomatic `serde` usage with `mlua` please see the
//! [mlua serde example](https://github.com/mlua-rs/mlua/blob/main/examples/serde.rs).
//!
//! This original `nvim-oxi` example is currently not working due to lack of
//! a `nvim-oximlua` compatibility layer for `nvim-oxi::lua::{Pushable, Poppable}`
//! but is kept for the time being in case such a compatibility layer is added
//! in the future.

#![cfg(false)] // TODO: Adjust to nvim-oximlua

//! ## Original `nvim-oxi` description
//!
//! Shows how to deserialize Lua tables into Rust objects using
//! [`serde`](https://serde.rs).
//!
//! ```lua
//! local mechanic = require("mechanic")
//!
//! local fixed = mechanic.fix({
//!   manufacturer = "Tesla",
//!   miles = 69420,
//!   works = false,
//!   problem = "kills_pedestrians",
//! })
//!
//! assert(fixed.works)
//! assert(fixed.problem == nil)
//! ```

use nvim_oxi::conversion::{Error as ConversionError, FromObject, ToObject};
use nvim_oxi::serde::{Deserializer, Serializer};
use nvim_oxi::{Dictionary, Function, Object, api, lua, print};
use serde::{Deserialize, Serialize};

#[nvim_oxi::plugin]
fn mechanic() -> Dictionary {
    Dictionary::from_iter([("fix", Function::from_fn(fix))])
}

fn fix(mut car: Car) -> Car {
    if car.works {
        return car;
    }

    if car.problem.is_none() {
        api::err_writeln("Well, what's the issue?");
        return car;
    }

    use CarManufacturer::*;
    use CarProblem::*;

    match (car.manufacturer, car.problem.unwrap()) {
        (Nikola, DoesntMove) => print!("Try going downhill"),
        (Tesla, KillsPedestrians) => print!("Hands on the wheel!!"),
        (Volkswagen, Pollutes) => print!("Software update?"),
        _ => {},
    }

    car.works = true;
    car.problem = None;

    car
}

#[derive(Serialize, Deserialize)]
struct Car {
    manufacturer: CarManufacturer,
    miles: u32,
    #[serde(default)]
    problem: Option<CarProblem>,
    #[serde(default = "yep")]
    works: bool,
}

fn yep() -> bool {
    true
}

impl FromObject for Car {
    fn from_object(obj: Object) -> Result<Self, ConversionError> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

impl ToObject for Car {
    fn to_object(self) -> Result<Object, ConversionError> {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}

impl lua::Poppable for Car {
    unsafe fn pop(lstate: *mut lua::ffi::State) -> Result<Self, lua::Error> {
        unsafe {
            let obj = Object::pop(lstate)?;
            Self::from_object(obj)
                .map_err(lua::Error::pop_error_from_err::<Self, _>)
        }
    }
}

impl lua::Pushable for Car {
    unsafe fn push(
        self,
        lstate: *mut lua::ffi::State,
    ) -> Result<std::ffi::c_int, lua::Error> {
        unsafe {
            self.to_object()
                .map_err(lua::Error::push_error_from_err::<Self, _>)?
                .push(lstate)
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
enum CarManufacturer {
    Nikola,
    Tesla,
    Volkswagen,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum CarProblem {
    DoesntMove,
    KillsPedestrians,
    Pollutes,
}
