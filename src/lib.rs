#![no_std]
#![crate_type="lib"]
#![crate_name="emdrv"]
#![feature(asm, core, no_std)]

#[macro_use]
extern crate core;

extern crate cmsis;
extern crate emlib;

pub mod gpioint;
pub mod flash;
pub mod tft;
pub mod i2c;
