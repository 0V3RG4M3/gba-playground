use core::marker::PhantomData;

use gba;
use gba::bios;
use gba::interrupts::IrqBits;
use gba::mmio;
use gba::video::obj::{ObjAttr0, ObjDisplayStyle};
use gba::video::{BackgroundControl, DisplayControl, DisplayStatus, VideoMode};

use crate::egj2025::backgrounds;
use crate::egj2025::context::Context;
use crate::egj2025::end_scene::EndScene;
use crate::egj2025::event_scene::EventScene;
use crate::egj2025::level::Level;
use crate::egj2025::level_scene_runners;
use crate::egj2025::sprites;
use crate::fixed::Fixed;
use crate::gba_synth;
use crate::mode7::{self, Camera, Sprite};
use crate::scene::{Scene, SceneRunner};

pub struct LevelScene<L: Level> {
    marker: PhantomData<L>,
}

impl<L: Level> LevelScene<L> {
    fn run_level(&mut self) {
        let mut level = L::new();

        let mut camera = Camera::new();
        camera.pos.x = Fixed::from_int(128);
        camera.pos.y = Fixed::from_int(16);
        camera.pos.z = Fixed::from_int(128);
        camera.set_pitch_angle(16);

        gba_synth::init_synth();

        loop {
            bios::VBlankIntrWait();
            gba_synth::play_step();

            let mut sprites = [Sprite::new(); 32];
            for sprite in &mut sprites {
                sprite.obj.0 = sprite.obj.0.with_style(ObjDisplayStyle::NotDisplayed);
            }

            let key_input = mmio::KEYINPUT.read();
            if level.process(&mut camera, &mut sprites, &key_input) {
                return;
            }

            mmio::BG2CNT.write(BackgroundControl::new().with_charblock(1));

            mode7::prepare_frame(1, &camera);

            sprites.sort_unstable_by_key(|sprite| sprite.z);
            for (i, sprite) in sprites.iter().enumerate() {
                mmio::OBJ_ATTR_ALL.index(i).write(sprite.obj);
            }

            mode7::process_line(0);

            gba::RUST_IRQ_HANDLER.write(Some(irq_handler));

            let display_control = DisplayControl::new()
                .with_video_mode(VideoMode::_2)
                .with_show_bg2(true)
                .with_show_obj(true)
                .with_obj_vram_1d(true);
            mmio::DISPCNT.write(display_control);
        }
    }
}

impl<L: Level> Scene for LevelScene<L> {
    type C = Context;

    fn new(_: &mut Context) -> LevelScene<L> {
        LevelScene { marker: PhantomData }
    }

    fn run(&mut self, context: &mut Context) -> SceneRunner<Context> {
        mmio::DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true).with_irq_hblank(true));
        mmio::IE.write(IrqBits::new().with_vblank(true).with_hblank(true));
        mmio::IME.write(true);

        mmio::OBJ_TILES.index(0).write([0x01010101; 8]);
        mmio::OBJ_TILES.index(1).write([0x01010101; 8]);
        for i in 0..128 {
            let va = mmio::OBJ_ATTR0.index(i);
            va.write(ObjAttr0::new().with_style(ObjDisplayStyle::NotDisplayed));
        }

        backgrounds::load();
        sprites::load();

        let frame = mmio::AFFINE1_SCREENBLOCKS.get_frame(1).unwrap();
        for y in 0..32 {
            for x in 0..16 {
                let indices = if y % 2 == 0 { [0, 1] } else { [2, 3] };
                frame.index(x, y).write(indices.into());
            }
        }

        self.run_level();
        context.level_index += 1;
        if context.level_index < level_scene_runners::LEVEL_SCENE_RUNNERS.len() {
            SceneRunner::<()>::new::<EventScene>()
        } else {
            SceneRunner::<()>::new::<EndScene>()
        }
    }
}

#[link_section = ".iwram"]
extern "C" fn irq_handler(irq_bits: IrqBits) {
    if !irq_bits.hblank() {
        return;
    }

    let vcount = mmio::VCOUNT.read();
    mode7::process_line(vcount as i32 + 1);
}
