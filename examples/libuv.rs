//! This example shows how to use the `nvim_oxi::libuv` module to trigger
//! a callback registered on the Neovim thread from other threads.

use std::thread;
use std::time::Duration;

use mlua::ExternalResult;
use nvim::libuv::{AsyncHandle, TimerHandle};
use nvim::mlua;
use nvim::{print, schedule};
use nvim_oximlua as nvim;
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::time;

#[mlua::lua_module]
fn libuv(lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
    nvim::init(lua)?;

    // --
    let mut n = 0;

    let callback = move |timer: &mut TimerHandle| {
        if n <= 10 {
            let i = n;
            schedule(move |_| print!("Callback called {i} times"));
            n += 1;
        } else {
            timer.stop().unwrap();
        }
    };

    let _handle = TimerHandle::start(
        Duration::from_millis(0),
        Duration::from_secs(1),
        callback,
    );

    // --
    let msg = String::from("Hey there!");

    let _handle = TimerHandle::once(Duration::from_secs(2), move || {
        schedule(move |_| print!("{msg}"));
    });

    // --
    let (sender, mut receiver) = mpsc::unbounded_channel::<i32>();

    let handle = AsyncHandle::new(move || {
        let i = receiver.blocking_recv().unwrap();
        schedule(move |_| print!("Received number {i} from backround thread"));
    })
    .into_lua_err()?;

    let _ = thread::spawn(move || send_numbers(handle, sender));

    Ok(mlua::Nil)
}

#[tokio::main]
async fn send_numbers(handle: AsyncHandle, sender: UnboundedSender<i32>) {
    let mut i = 0;

    loop {
        sender.send(i).unwrap();
        handle.send().unwrap();
        i += 1;

        time::sleep(Duration::from_secs(1)).await;
    }
}
