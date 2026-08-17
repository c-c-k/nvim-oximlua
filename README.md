# 🔗 nvim-oxi

[![CI]](https://github.com/c-c-k/nvim-oximlua/actions) (`neovim-nightly` tests are temporarily disabled)

[CI]: https://github.com/c-c-k/nvim-oximlua/actions/workflows/ci.yaml/badge.svg

*First and foremost I am grateful and full of admiration towards
[noib3](https://github.com/noib3) for creating nvim-oxi,
the more that I work on my oximlua fork the more that I understand it to be
a bloated abomination disfiguring the minimalistic
elegance and beauty of nvim-oxi.*

*With that said, the core motivation for nvim-oximlua is
being enthusiastic about Rust and wanting to use a Rust based editor,
but being too attached to NVIM/NeoVIM to switch to Muon or Zed.
Consequentially, contrary to nvim-oxi's pragmatic goal of
creating a small collection of high-perfomance plugins,
nvim-oximlua's delusional and far over the rainbow goal is
to RIIR the nvim ecosystem and ultimetally maybe even nvim itself,
with that in mind, using the Rust ecosystem as much as possible rather than
trying to rewrite it too seems like a fair concession.*


## Features

- From [**nvim-oxi**]:
  - Direct Rust bindings to nvim's C API (`vim.api.*`)
    that avoid the overhead and limitations of using [RPC channels].
  - Native Rust types for the API's arguments and return values.
  - Binding to [`vim.schedule`].
  - Basic [`libuv`] integration.
  - A testing framework for running tests inside of a nvim instance.
- From [**mlua**]:
  - Robust error and panic propagation across the Rust-Lua boundary.
  - Generally an industry standard Rust-Lua API.
- From [**mlua-extras**]:
  - Tooling for generating [`lua-ls`] definition files from Rust types 
    and documentation.

[**nvim-oxi**]: https://github.com/noib3/nvim-oxi
[RPC channels]: https://neovim.io/doc/user/api.html#RPC
[`vim.schedule`]: https://neovim.io/doc/user/lua/#vim.schedule()
[`libuv`]: https://neovim.io/doc/user/lua/#vim.uv
[`lua-ls`]: https://github.com/luals/lua-language-server
[**mlua**]: https://github.com/mlua-rs/mlua
[**mlua-extras**]: https://github.com/tired-fox/mlua-extras

## Cargo Features

### `nvim-oximlua` specific

- `neovim-0-11` / `neovim-0-12` / `neovim-nightly`: Sets the target nvim
  release for which to compile `nvim-oximlua`.
  - Exactly one of those features should be used, not adding any of them will 
    generate a compile time error, using more than one will resolve to the
    one corresponding to the newest nvim version.
- `libuv`: Enable basic `libuv` integration.
- `test`: Enable the testing framework.
- TODO: `test-terminator` (enables `libuv` and `test`).
- `mlua-extras`: Enable `mlua-extras` re-export.

### `mlua` Re-Exports

The following features are re-exports of [`mlua` features] that enable 
corresponding functionality in `nvim-oximlua` and/or `mlua-extras` where 
relevant.<br>
NOTE: Since `nvim-oximlua` is locked to the `luajit` and `module` features,
the `lua*`, `vendored`, `module` and `send` (which is locked to `vendored`) 
features are not re-exported. Also the `serde` feature is not re-exported 
since `nvim-oximlua` heavily relies on [serde] and thus it is always enabled.

- `async`: enable async/await support
  (any executor can be used, eg. [tokio] or [async-std]).
- `error-send`: make `mlua:Error: Send + Sync`.
- `macros`: enable procedural macros (such as `chunk!`).
  - NOTE: This does not currently effect any of the `nvim-oximlua` macros.
- `anyhow`: enable `anyhow::Error` conversion into Lua (enables `error-send`).
- `userdata-wrappers`: opt into `impl UserData` for
  `Rc<T>`/`Arc<T>`/`Rc<RefCell<T>>`/`Arc<Mutex<T>>` where `T: UserData`

[`mlua` features]: https://github.com/mlua-rs/mlua#feature-flags
[serde]: https://github.com/serde-rs/serde
[tokio]: https://github.com/tokio-rs/tokio
[async-std]: https://github.com/async-rs/async-std

## Crate setup

The first step is to create a new library crate with `cargo new --lib
{your_plugin}` and edit the generated `Cargo.toml` to include:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
nvim-oximlua = { git = "https://github.com/c-c-k/nvim-oximlua" }
```

NOTE: `nvim-oximlua` hasn't been released to `crates.io` yet so it has to be 
installed from it's github source.

NOTE: `mlua` and `mlua-extras` should **NOT** be directly added to avoid 
potential version and feature conflicts, please use the `nvim-oximlua::mlua` 
and `nvim-oximlua::mlua_extras` re-exports instead. All relevant `mlua` and 
`mlua-extras` features are re-exported as `nvim-oximlua` features
(see [`mlua` Re-Exports](#mlua-re-exports) above).

Next, in `lib.rs` we'll annotate the entry point of the plugin with the
`#[mlua::lua_module]` macro and add the `nvim-oximlua::init` shim:

```rust
// lib.rs
use nvim_oximlua as nvim;
use nvim::mlua;

#[mlua::lua_module]
fn foo(lua: &mlua::Lua) -> mlua::Result<i32> {
    nvim::init(lua)?;
    Ok(42)
}
```

macOS users will also need to set a few linker arguments to tell the Rust
linker that the FFI functions `nvim-oxi` links to will only be available at
runtime. A possible way to do this is to create a `.cargo/config` file with the
following content:

```toml
[target.x86_64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]

[target.aarch64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]
```

After building the crate with `cargo build {--release}`, cargo will place the
compiled artifacts in `target/debug` or `target/release` depending on whether
you built a debug or release version of the crate. If the package name
specified in `Cargo.toml` is `"foo"`, the library will be named:

  - `libfoo.so` on Linux;
  - `libfoo.dylib` on macOS;
  - `foo.dll` on Windows.

Next, we need to tell nvim where to load the plugin from. Create a new
directory named `lua` and place the compiled library inside it, renaming it to

  - `foo.so` on Linux;
  - `foo.so` on macOS;
  - `foo.dll` on Windows (i.e. no renaming).

Now open nvim and add *the parent directory* of `lua` to the
[runtimepath](https://neovim.io/doc/user/options.html#'runtimepath'), for
example with `:set rtp+=~/foobar`, assuming `lua` is in `~/foobar/lua`.

And we're done. You can now call the `require` function to load the plugin just
like any other Lua plugin, which will return the output of the `foo()` function
defined in `lib.rs`:

```lua
print(require("foo")) -- prints `42`
```

## Compatibility With / Transition From `nvim-oxi`

### What Doesn't Work

Anything that uses `nvim_oxi::lua` directly (e.g. implementations of
`nvim_oxi::lua:{Pushable, Poppable}` for custom types).

If you need this to give `nvim-oximlua` a try please open an issue with 
a request for a compatibility layer for `nvim_oxi::lua` (preferably with
links to key places in your plugin/config where you need such compatibility).

Until such a request appears adding `nvim_oxi::lua` compatibility
is low priority.

### What Should Work

Hopefully everything else.
<br>
Usage of `nvim_oximlua::mlua::lua` will work but give deprecation warnings.

### Required Adjustments

#### Cargo Crate Dependency

The cargo dependency needs to changed from `nvim-oxi = {...}` to
`nvim-oximlua = { git = "https://github.com/c-c-k/nvim-oximlua", ... }`.
<br>
Or `nvim-oxi = { package = "nvim-oximlua", git = ... }`
To avoid the need to change `nvim_oxi` to `nvim_oximlua` everywhere.

#### Plugin / Config Entry Point Adjustments

Please see the [compatibility example], the overall list of changes is:

- The `mlua` re-export must be in scope (i.e. `use nvim_oxi::mlua;`).
- The entry point macro needs to be changed from `#[nvim_oxi::plugin]`
  to `#[mlua::lua_module]`.
- The entry point function's signature and body need to be adjusted to
  `#[mlua::lua_module]`:
  - A `lua: &mlua::Lua` parameter needs to be added.
  - The return type needs to be wrapped in `mlua::Result`.
  - `nvim::init(lua)?;` needs to be called at the function body's start.

[compatibility example]: https://github.com/c-c-k/nvim-oximlua/blob/main/examples/exports_compatibility.rs

## Examples

Please see the [examples] directory as well as the [mlua examples]
and the [mlua-extras examples].

[examples]: https://github.com/c-c-k/nvim-oximlua/tree/main/examples
[mlua examples]: https://github.com/mlua-rs/mlua/tree/main/examples
[mlua-extras examples]: https://github.com/tired-fox/mlua-extras/tree/main/examples

## Testing

Turning on the `test` feature enables `#[nvim_oxi::test]`, which replaces the
regular `#[test]` macro and allows you to test a piece of code from within a
nvim instance using Rust's testing framework.

For example:

```rust
use nvim_oximlua::api;

#[nvim_oximlua::test]
fn set_get_del_var() {
    api::set_var("foo", 42).unwrap();
    assert_eq!(Ok(42), api::get_var("foo"));
    assert_eq!(Ok(()), api::del_var("foo"));
}
```

When `cargo test` is executed, the generated code will spawn a new nvim
process with the `nvim` binary in your `$PATH`, test your code, and exit.

There's a gotcha: you can't have two tests with the same name in the same
crate, even if they belong to different modules. For example, this won't work:

```rust
mod a {
    #[nvim_oximlua::test]
    fn foo() {}
}

mod b {
    #[nvim_oximlua::test]
    fn foo() {}
}
```

Note that all integration tests must live inside a separate `cdylib` crate with
the following build script:

```rust
// build.rs
fn main() -> Result<(), nvim_oximlua::tests::BuildError> {
    nvim_oximlua::tests::build()
}
```
