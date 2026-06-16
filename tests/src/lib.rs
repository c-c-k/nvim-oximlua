mod api;
mod r#macro;

// // Libuv bindings don't work on Windows.
// #[cfg(not(any(target_os = "windows", target_env = "msvc")))]
#[cfg(not(any(
    // Libuv bindings don't work on Windows.
    target_os = "windows", target_env = "msvc",
    // libuv not implemented for oximlua yet
    feature = "oximlua"
)))]
mod libuv;
