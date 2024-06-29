#![cfg_attr(not(test), no_std)]

#[cfg(not(test))]
pub mod backgrounds;
#[cfg(not(test))]
pub mod fixed;
#[cfg(not(test))]
pub mod game;
#[cfg(not(test))]
pub mod gba_synth;
#[cfg(not(test))]
pub mod log4gba;
#[cfg(not(test))]
pub mod math;
#[cfg(not(test))]
pub mod mode7;
pub mod scene;
#[cfg(not(test))]
pub mod sprites;
#[cfg(not(test))]
pub mod static_sounds_lib;
pub mod tune;
#[cfg(not(test))]
pub mod vec3;
