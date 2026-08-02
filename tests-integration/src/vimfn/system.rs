use nvim_oximlua::api::opts::*;
use nvim_oximlua::vimfn;

#[nvim_oximlua::test]
fn stdpath() {
    let _ = vimfn::stdpath(StdPath::Config)
        .expect("calling `stdpath` for `StdPath::Config` failed")
        .next()
        .expect(
            "calling `stdpath` for `StdPath::Config` didn't return any paths",
        );
}
