/*
 * Made by Evan Goemer
 * Discord: @evangoemer
 */

#![no_std]
#![no_main]

use gba::mmio;
use gba::video::{self, DisplayControl, VideoMode};
use gba_playground::screens;

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn main() -> ! {
    video::video3_set_bitmap(&screens::SCREEN_SPLASH);
    mmio::DISPCNT.write(DisplayControl::new().with_video_mode(VideoMode::_3).with_show_bg2(true));
    loop {}
}
