#![no_std]
#![no_main]

use core::fmt::Write;

use gba::mgba::{MgbaBufferedLogger, MgbaMessageLevel};

use gba_playground::game::game_scene::GameScene;
use gba_playground::game::screen_scene::ScreenScene;
use gba_playground::scene::SceneRunner;


#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
        writeln!(logger, "{info}").ok();
    }
    loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
    let mut scene_runner = SceneRunner::<()>::new::<ScreenScene>();
    loop {
        scene_runner = scene_runner.run(&mut ());
    }
}
