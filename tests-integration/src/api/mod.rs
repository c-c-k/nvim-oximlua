#[cfg(feature = "neovim-0-12")] // On 0.12 and Nightly.
mod autocmd;
// TODO: Delete when dropping support for NVIM 0.11
#[cfg(not(feature = "neovim-0-12"))] // Only on 0.11
mod autocmd_neovim_0_11;
mod buffer;
mod command;
mod extmark;
mod global;
mod highlight;
mod tabpage;
mod vimscript;
mod win_config;
mod window;
