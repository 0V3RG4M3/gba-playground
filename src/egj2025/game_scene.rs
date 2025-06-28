use crate::backgrounds;
use crate::egj2025::context::Context;
use crate::egj2025::end_scene::EndScene;
use crate::egj2025::event_scene::EventScene;
use crate::egj2025::level::Level;
use crate::egj2025::levels;
use crate::egj2025::player::Player;
use crate::fixed::Fixed;
use crate::gba_synth;
use crate::mode7::{self, Camera, Sprite};
use crate::scene::{Scene, SceneRunner};
use crate::sprites;
use gba;
use gba::bios;
use gba::fixed::i16fx8;
use gba::interrupts::IrqBits;
use gba::mmio;
use gba::video::obj::{ObjAttr0, ObjDisplayStyle};
use gba::video::{BackgroundControl, DisplayControl, DisplayStatus, VideoMode};

pub struct GameScene {}

impl GameScene {
    fn run_level(&mut self, mut level: Level) {
        let mut player = Player::new();

        let mut camera = Camera::new();
        camera.pos.x = Fixed::from_int(128);
        camera.pos.y = Fixed::from_int(16);
        camera.pos.z = Fixed::from_int(128);
        camera.set_pitch_angle(16);

        gba_synth::init_synth();

        loop {
            bios::VBlankIntrWait();
            gba_synth::play_step();

            let key_input = mmio::KEYINPUT.read();
            player.process(&mut level.items, &mut camera, &key_input);
            if level.process(player.item_index()) {
                return;
            }

            mmio::BG2CNT.write(BackgroundControl::new().with_charblock(1));

            mode7::prepare_frame(1, &camera);

            let mut sprites = [Sprite::new(); 32];
            for sprite in &mut sprites {
                sprite.obj.0 = sprite.obj.0.with_style(ObjDisplayStyle::NotDisplayed);
            }

            for (i, item) in level.items.iter_mut().enumerate() {
                let Some(item) = item else { continue };
                let sprite = &mut item.sprite;
                if player.item_index() == Some(i) {
                    sprite.obj.0 = sprite.obj.0.with_style(ObjDisplayStyle::Normal);
                    sprite.obj.1 = sprite.obj.1.with_x(0);
                    sprite.obj.0 = sprite.obj.0.with_y(0);
                } else {
                    mode7::prepare_sprite(&camera, sprite);
                    let affine_index = sprite.obj.1.affine_index() as usize;
                    let scale = i16fx8::from_bits(sprite.scale.into_raw() as i16);
                    mmio::AFFINE_PARAM_A.index(affine_index).write(scale);
                    mmio::AFFINE_PARAM_D.index(affine_index).write(scale);
                }
                sprites[i] = *sprite;
            }

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

impl Scene for GameScene {
    type C = Context;

    fn new(_: &mut Context) -> GameScene {
        GameScene {}
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

        self.run_level(levels::LEVELS[context.level_index]());
        context.level_index += 1;
        if context.level_index < levels::LEVELS.len() {
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
