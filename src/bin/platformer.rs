#![no_std]
#![no_main]

use core::cmp;
use core::fmt::Write;

use gba::bios;
use gba::interrupts::IrqBits;
use gba::mgba::{MgbaBufferedLogger, MgbaMessageLevel};
use gba::mmio;
use gba::video::obj::{ObjAttr, ObjAttr0, ObjAttr1, ObjAttr2, ObjDisplayStyle};
use gba::video::{BackgroundControl, Color, DisplayControl, DisplayStatus, TextEntry};
use gba::sound::{LeftRightVolume, PsgMix, SoundEnable, SoundMix, SweepControl, ToneFrequency, TonePattern};


#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
        writeln!(logger, "{info}").ok();
    }
    loop {}
}

#[no_mangle]
extern "C" fn main() -> ! {
    mmio::DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
    mmio::IE.write(IrqBits::VBLANK);
    mmio::IME.write(true);

    mmio::BG_PALETTE.index(1).write(Color::BLACK);
    mmio::BG_PALETTE.index(2).write(Color::WHITE);
    mmio::OBJ_PALETTE.index(1).write(Color::MAGENTA);

    mmio::CHARBLOCK0_8BPP.index(0).write([0x01010101; 16]);
    mmio::CHARBLOCK0_8BPP.index(1).write([0x02020202; 16]);
    let screenblock = mmio::TEXT_SCREENBLOCKS.get_frame(1).unwrap();
    for y in 0..32 {
        for x in 0..32 {
            let tile = if y < 16 { 0 } else { 1 };
            screenblock.index(x, y).write(TextEntry::new().with_tile(tile));
        }
    }
    mmio::BG0CNT.write(BackgroundControl::new().with_bpp8(true).with_screenblock(1));


    mmio::OBJ_TILES.index(0).write([0x01010101; 8]);
    mmio::OBJ_TILES.index(1).write([0x01010101; 8]);
    for i in 1..128 {
        let va = mmio::OBJ_ATTR0.index(i);
        va.write(ObjAttr0::new().with_style(ObjDisplayStyle::NotDisplayed));
    }

    let mut vy = 0;
    let (mut px, mut py): (i16, i16) = (32, 128);

    // ------------------------------------- SOUND --------------------------------------
    // turn sound on
    // REG_SNDSTAT= SSTAT_ENABLE;  
    // REG_SNDSTAT --> SOUNDCNT_X --> SOUND_ENABLED
    mmio::SOUND_ENABLED.write(SoundEnable::new().with_enabled(true));

    // snd1 on left/right ; both full volume
    // REG_SNDDMGCNT = SDMG_BUILD_LR(SDMG_SQR1, 7);
    //      REG_SNDDMGCNT --> SOUNDCNT_L  --> LEFT_RIGHT_VOLUME
    mmio::LEFT_RIGHT_VOLUME.write(LeftRightVolume::new()
        .with_tone1_left(true)
        .with_left_volume(2)
        .with_tone1_right(true)
        .with_right_volume(2)
    );


    // DMG ratio to 100%
    // REG_SNDDSCNT= SDS_DMG100;
    //      REG_SNDDSCNT --> SOUNDCNT_H --> SoundMix
    //      SDS_DMG100 --> 0b10
    mmio::SOUND_MIX.write(SoundMix::new().with_psg(PsgMix::_100));

    // disable the sweep of tone 1 (to disable, set sweep time to 0)
    // REG_SND1SWEEP= SSW_OFF;
    //      REG_SND1SWEEP --> SOUND1CNT_L --> TONE1_SWEEP
    mmio::TONE1_SWEEP.write(SweepControl::new().with_sweep_time(0));

    // envelope: vol=12, decay, max step time (7) ; 50% duty
    // REG_SND1CNT= SSQR_ENV_BUILD(12, 0, 7) | SSQR_DUTY1_2;
    //      REG_SND1CNT --> SOUND1CNT_H --> TONE1_PATTERN
    mmio::TONE1_PATTERN.write(TonePattern::new()
        .with_volume(12)
        .with_duty(1)  // Duty cycle    0: 12.5%, 1: 25%, 2: 50%, 3: 75%
        .with_length(31)  // L in [0, 63]. Resulting length is: (64−val)/256 second. So L=0 -> 250 ms, and L=63 -> 3.9 ms
    );
    mmio::TONE1_FREQUENCY.write(ToneFrequency::new().with_frequency(0));

    let mut freq = 1751;
    loop {
        bios::VBlankIntrWait();

        let mut obj_attr = ObjAttr::new();
        obj_attr.0 = ObjAttr0::new().with_y(py as u16 - 8).with_bpp8(true);
        obj_attr.1 = ObjAttr1::new().with_x(px as u16);
        obj_attr.2 = ObjAttr2::new();
        mmio::OBJ_ATTR_ALL.index(0).write(obj_attr);

        mmio::DISPCNT.write(DisplayControl::new().with_show_bg0(true).with_show_obj(true));

        let key_input = mmio::KEYINPUT.read();

        

        let mut vx = 0;
        if key_input.left() {
            vx -= 2;
            freq -= 2;
            mmio::TONE1_FREQUENCY.write(ToneFrequency::new().with_frequency(freq).with_enabled(true));
        }
        if key_input.right() {
            vx += 2;
            freq += 2;
            mmio::TONE1_FREQUENCY.write(ToneFrequency::new().with_frequency(freq).with_enabled(true));
        }

        if py == 128 {
            vy = if key_input.up() { -8 } else { 0 };
        } else {
            vy += 1;
        }

        px = cmp::min(cmp::max(0, px + vx), 232);
        py = cmp::min(cmp::max(8, py + vy), 128);


        

    }
}
