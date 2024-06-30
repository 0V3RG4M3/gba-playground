/*
 * Made by Evan Goemer
 * Discord: @evangoemer
 */

#![no_std]
#![no_main]

use gba::{mem_fns::__aeabi_memcpy, prelude::*};
use gba_playground::screens;

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn main() -> ! {
    let a = TEXT_SCREENBLOCKS.get_frame(0).unwrap().as_usize();
    unsafe {
        __aeabi_memcpy(
            a as _,
            screens::SCREEN_SPLASH.as_ptr().cast(),
            core::mem::size_of_val(screens::SCREEN_SPLASH) as _,
        )
    };
    DISPCNT.write(DisplayControl::new().with_video_mode(VideoMode::_3).with_show_bg2(true));
    loop {}
}
