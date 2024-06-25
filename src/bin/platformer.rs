#![no_std]
#![no_main]

use gba_playground::tune;

use core::cmp;
use core::fmt::Write;

use gba::bios;
use gba::interrupts::IrqBits;
use gba::keys::KeyInput;
use gba::mgba::{MgbaBufferedLogger, MgbaMessageLevel};
use gba::mmio;
use gba::sound::{
    LeftRightVolume, PsgMix, SoundEnable, SoundMix, SweepControl, ToneFrequency, TonePattern,
};
use gba::video::obj::{ObjAttr, ObjAttr0, ObjAttr1, ObjAttr2, ObjDisplayStyle};
use gba::video::{BackgroundControl, Color, DisplayControl, DisplayStatus, TextEntry};

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if let Ok(mut logger) = MgbaBufferedLogger::try_new(MgbaMessageLevel::Fatal) {
        writeln!(logger, "{info}").ok();
    }
    loop {}
}

fn log(message_level: MgbaMessageLevel, message: impl core::fmt::Debug) {
    if let Ok(mut logger) = MgbaBufferedLogger::try_new(message_level) {
        // Concatenate all parameters into a single string
        writeln!(logger, "{:?}", message).ok();
    }
}

fn logd(message: impl core::fmt::Debug) {
    log(MgbaMessageLevel::Debug, message)
}

fn init_synth() {
    // turn sound on
    // REG_SNDSTAT= SSTAT_ENABLE;
    // REG_SNDSTAT --> SOUNDCNT_X --> SOUND_ENABLED
    mmio::SOUND_ENABLED.write(SoundEnable::new().with_enabled(true));

    // snd1 on left/right ; both full volume
    // REG_SNDDMGCNT = SDMG_BUILD_LR(SDMG_SQR1, 7);
    //      REG_SNDDMGCNT --> SOUNDCNT_L  --> LEFT_RIGHT_VOLUME
    mmio::LEFT_RIGHT_VOLUME.write(
        LeftRightVolume::new()
            .with_tone1_left(true)
            .with_left_volume(2)
            .with_tone1_right(true)
            .with_right_volume(2),
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
    mmio::TONE1_PATTERN.write(
        TonePattern::new()
            .with_volume(15) // Volume value in [0, 15]
            .with_duty(2) // Duty cycle    0: 12.5%, 1: 25%, 2: 50%, 3: 75%
            .with_length(63) // L in [0, 63]. Resulting length is: (64−val)/256 second. So L=0 -> 250 ms, and L=63 -> 3.9 ms
            .with_step_increasing(false)
            .with_step_time(7), // envelope decay time in [0, 7]. 0: inf, 1: shortest 7: long
    );
    mmio::TONE1_FREQUENCY.write(ToneFrequency::new().with_frequency(0));
}

const PITCH2RATE_MAP: [u16; 92] = [
    45, 156, 262, 362, 458, 547, 632, 712, 786, 856, 923, 986, 1046, 1102, 1155, 1205, 1253, 1297,
    1340, 1380, 1417, 1452, 1486, 1517, 1547, 1575, 1602, 1627, 1650, 1673, 1694, 1714, 1732, 1750,
    1767, 1783, 1798, 1812, 1825, 1837, 1849, 1860, 1871, 1881, 1890, 1899, 1907, 1915, 1923, 1930,
    1936, 1943, 1949, 1954, 1959, 1964, 1969, 1974, 1978, 1982, 1985, 1989, 1992, 1995, 1998, 2001,
    2004, 2006, 2009, 2011, 2013, 2015, 2017, 2018, 2020, 2022, 2023, 2025, 2026, 2027, 2028, 2029,
    2030, 2031, 2032, 2033, 2034, 2035, 2036, 2036, 2037, 2038,
];

fn play_tone1(pitch: u16, velocity: u16) {
    logd(pitch);
    logd(velocity);

    let volume = velocity >> 3;
    let rate = PITCH2RATE_MAP[(pitch - 36) as usize];
    logd(rate);

    mmio::TONE1_PATTERN.write(mmio::TONE1_PATTERN.read().with_volume(volume));
    mmio::TONE1_FREQUENCY.write(ToneFrequency::new().with_frequency(rate).with_enabled(true));
}

fn play_tune(frame_id: u16) {
    let note1 = tune::TUNE_TRACK1[frame_id as usize];
    if note1.0 > 0 {
        play_tone1(note1.0, note1.1);
    }
    let note2 = tune::TUNE_TRACK2[frame_id as usize];
    if note2.0 > 0 {
        play_tone1(note2.0, note2.1);
    }
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

    init_synth();

    let mut pitch = 60;
    let mut vel = 64;
    let mut key_was_pressed: KeyInput = KeyInput::new();
    let mut frame_id = 0;
    loop {
        bios::VBlankIntrWait();
        play_tune(frame_id);

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

            // on press
            if !key_was_pressed.left() {
                pitch -= 1;
                play_tone1(pitch, vel);
            }
        }

        if key_input.right() {
            vx += 2;

            // on press
            if !key_was_pressed.right() {
                pitch += 1;
                play_tone1(pitch, vel);
            }
        }

        if key_input.up() {
            // on press
            if !key_was_pressed.up() {
                vel += 8;
                play_tone1(pitch, vel);
            }
        }
        if key_input.down() {
            // on press
            if !key_was_pressed.down() {
                vel -= 8;
                play_tone1(pitch, vel);
            }
        }

        if py == 128 {
            vy = if key_input.up() { -8 } else { 0 };
        } else {
            vy += 1;
        }

        px = cmp::min(cmp::max(0, px + vx), 232);
        py = cmp::min(cmp::max(8, py + vy), 128);

        key_was_pressed = key_input;
        frame_id = (frame_id + 1) % tune::TUNE_STEP_COUNT;
    }
}
