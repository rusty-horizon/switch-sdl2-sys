#![allow(warnings)]

extern crate core;
include!("../bindgen/sdl2.rs");

#[cfg(feature = "ttf")]
pub mod ttf;

#[cfg(feature = "image")]
pub mod image;