use nvim_oxi::api::{self, opts::*, types::*};

#[nvim_oxi::test]
fn get_highlights() {
    let (name, _) = api::get_color_map().next().unwrap();
    let id = api::get_hl_id_by_name(&name).unwrap();

    let GetHlInfos::Single(by_id) =
        api::get_hl(0, &GetHighlightOpts::builder().id(id).build()).unwrap()
    else {
        panic!("expected a single");
    };

    let GetHlInfos::Single(by_name) = api::get_hl(
        0,
        &GetHighlightOpts::builder().name(name.as_str()).build(),
    )
    .unwrap() else {
        panic!("expected a single")
    };

    assert_eq!(by_id, by_name);
}

#[nvim_oxi::test]
fn get_hl() {
    let infos = api::get_hl(0, &Default::default()).unwrap();
    let GetHlInfos::Map(map_iter) = infos else { panic!("expected a map") };
    assert!(!map_iter.collect::<Vec<_>>().is_empty());

    let opts = GetHighlightOpts::builder().name("Normal").build();
    let infos = api::get_hl(0, &opts).unwrap();
    let GetHlInfos::Single(infos) = infos else { panic!("expected a single") };
    assert!(infos.foreground.is_some());
    assert!(infos.background.is_some());
}

#[nvim_oxi::test]
fn hl_underline() {
    let opts = SetHighlightOpts::builder().underline(true).build();
    api::set_hl(0, "MatchParen", &opts).unwrap();

    let GetHlInfos::Single(infos) = api::get_hl(
        0,
        &GetHighlightOpts::builder().name("MatchParen").build(),
    )
    .unwrap() else {
        panic!("expected a single");
    };
    assert_eq!(Some(true), infos.underline);
}

#[nvim_oxi::test]
fn hl_foreground_bold() {
    let opts =
        SetHighlightOpts::builder().foreground("#3399cc").bold(true).build();
    api::set_hl(0, "Directory", &opts).unwrap();

    let GetHlInfos::Single(infos) =
        api::get_hl(0, &GetHighlightOpts::builder().name("Directory").build())
            .unwrap()
    else {
        panic!("expected a single");
    };
    assert_eq!(Some(0x3399cc), infos.foreground);
    assert_eq!(Some(true), infos.bold);
}

#[cfg(any(feature = "neovim-0-12", feature = "neovim-nightly"))]
#[nvim_oxi::test]
fn hl_cterm() {
    let mut cterm = HighlightCterm::builder().bold(true).build();
    cterm.italic = Some(true);
    let opts =
        SetHighlightOpts::builder().foreground("#3399cc").cterm(cterm).build();
    api::set_hl(0, "Directory", &opts).unwrap();

    let GetHlInfos::Single(infos) =
        api::get_hl(0, &GetHighlightOpts::builder().name("Directory").build())
            .unwrap()
    else {
        panic!("expected a single");
    };
    let cterm = infos.cterm.expect("cterm should be returned");
    assert_eq!(Some(true), cterm.bold);
    assert_eq!(Some(true), cterm.italic);
}
