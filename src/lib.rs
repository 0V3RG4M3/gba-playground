#![cfg_attr(not(test), no_std)]

#[cfg(not(test))]
pub mod gba_synth;
#[cfg(not(test))]
pub mod log4gba;
pub mod scene;
#[cfg(not(test))]
pub mod static_sounds_lib;
pub mod tune;
