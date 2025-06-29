#![cfg_attr(not(test), no_std)]

#[cfg(not(test))]
pub mod egj2024;
#[cfg(not(test))]
pub mod egj2025;
#[cfg(not(test))]
pub mod fixed;
#[cfg(not(test))]
pub mod gba_synth;
#[cfg(not(test))]
pub mod gba_synth2;
#[cfg(not(test))]
pub mod log4gba;
#[cfg(not(test))]
pub mod math;
#[cfg(not(test))]
pub mod mode7;
#[cfg(not(test))]
pub mod reg_tune;
pub mod scene;
pub mod sfx;
#[cfg(not(test))]
pub mod static_sounds_lib;
pub mod tune;
#[cfg(not(test))]
pub mod vec3;
