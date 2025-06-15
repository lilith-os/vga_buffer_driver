#![cfg_attr(feature = "no_std", no_std)]
extern crate core;

mod color;
mod screen_char;
mod writer;
pub(crate) mod buffer;
pub mod prelude;