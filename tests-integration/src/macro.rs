//! Tests about the `#[nvim_oximlua::test]` macro.

#[should_panic]
#[nvim_oximlua::test]
fn panic_is_propagated() {
    panic!();
}

#[nvim_oximlua::test]
fn printing_to_stderr_is_ok() {
    eprintln!("AA!");
}
