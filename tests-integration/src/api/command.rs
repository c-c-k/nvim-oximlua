use nvim_oxi::api::{self, opts::*, types::*};

#[nvim_oxi::test]
fn command_nargs_0() {
    let opts = CreateCommandOpts::builder().nargs(CommandNArgs::Zero).build();
    api::create_user_command("Foo", ":", &opts).unwrap();
    assert_eq!(api::command("Foo"), Ok(()));
    let err = api::command("Foo foo");
    assert!(err.is_err(), "expected an error when passing an argument");
}

#[nvim_oxi::test]
fn command_nargs_1() {
    let opts = CreateCommandOpts::builder().nargs(CommandNArgs::One).build();
    api::create_user_command("Foo", ":", &opts).unwrap();
    let err = api::command("Foo");
    assert!(err.is_err(), "expected an error when passing 0 arguments");
    assert_eq!(api::command("Foo foo"), Ok(()));
}

#[nvim_oxi::test]
fn regression_1() {
    let opts = CreateCommandOpts::builder()
        .bang(true)
        .desc("play audio using echo.nvim")
        .nargs(CommandNArgs::ZeroOrOne)
        .build();
    api::create_user_command("Echo", "", &opts).unwrap();
}

#[cfg(feature = "neovim-0-12")]
#[nvim_oxi::test]
fn command_complete_customlist() {
    api::command("comclear").unwrap();
    let opts = CreateCommandOpts::builder()
        .nargs(CommandNArgs::Any)
        .complete(CommandComplete::Customlist("VimFunc".to_string()))
        .build();
    api::create_user_command("Foo", ":", &opts).unwrap();
    let cmd_info = unsafe { api::get_commands(&Default::default()) }
        .unwrap()
        .find(|cmd| cmd.name == "Foo")
        .unwrap();
    let complete =
        cmd_info.complete.expect("Missing `CommandInfos::complete` value");
    assert_eq!(complete, CommandComplete::Customlist(String::new()));
    let complete_arg = cmd_info
        .complete_arg
        .expect("Missing `CommandInfos::complete_arg` value");
    assert_eq!(complete_arg, "VimFunc".to_string());
}

#[cfg(feature = "neovim-0-12")]
#[nvim_oxi::test]
fn command_complete_callback() {
    api::command("comclear").unwrap();
    let opts = CreateCommandOpts::builder()
        .nargs(CommandNArgs::Any)
        .complete(|_args: CompleteCallbackArgs| vec!["Bar".to_string()])
        .build();
    api::create_user_command("Foo", ":", &opts).unwrap();
    let cmd_info = unsafe { api::get_commands(&Default::default()) }
        .unwrap()
        .find(|cmd| cmd.name == "Foo")
        .unwrap();
    let complete =
        cmd_info.complete.expect("Missing `CommandInfos::complete` value");
    match complete {
        CommandComplete::Callback(fun) => {
            let res = fun.call((String::new(), String::new(), 0)).unwrap();
            assert_eq!(Some(&"Bar".to_string()), res.first())
        },
        _ => panic!("Wrong `complete::Callback` value"),
    }
}
