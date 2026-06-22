use nvim_oxi::vimfn::{self, opts::*};

#[nvim_oxi::test]
fn stdpath() {
    let _ = vimfn::stdpath(StdPath::Config)
        .expect("calling `stdpath` for `StdPath::Config` failed")
        .next()
        .expect(
            "calling `stdpath` for `StdPath::Config` didn't return any paths",
        );
}
