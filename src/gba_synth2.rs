use gba::mmio;
use gba::mmio::{Safe, VolAddress};
use gba::sound::{LeftRightVolume, PsgMix, SoundEnable, SoundMix};

use crate::reg_tune;

static mut LOOP_DELTA_COUNT: i8 = 0;
static mut CURRENT_TIME_STEP_TUNE_1: u16 = 0;
static mut CURRENT_INDEX_TUNE_1: usize = 0; // frame step on tune 1
static mut CURRENT_TUNE_1: [(u16, u8, u32, u32); reg_tune::TUNE_SIZE as usize] =
    [(0, 0, 0, 0); reg_tune::TUNE_SIZE as usize];

pub fn init_synth() {
    mmio::SOUND_ENABLED.write(SoundEnable::new().with_enabled(true));

    mmio::LEFT_RIGHT_VOLUME.write(
        LeftRightVolume::new()
            .with_right_volume(7)
            .with_left_volume(7)
            .with_tone1_left(true)
            .with_tone2_left(true)
            .with_noise_left(true)
            .with_tone1_right(true)
            .with_tone2_right(true)
            .with_noise_right(true),
    );

    mmio::SOUND_MIX.write(SoundMix::new().with_psg(PsgMix::_50));

    play_tune(reg_tune::TUNE_TRACK1);
}

pub fn write(size: u8, addr: u32, value: u32) {
    unsafe {
        match size {
            1 => VolAddress::<u8, Safe, Safe>::new(addr as usize).write(value as u8),
            2 => VolAddress::<u16, Safe, Safe>::new(addr as usize).write(value as u16),
            4 => VolAddress::<u32, Safe, Safe>::new(addr as usize).write(value),
            _ => panic!("Unsupported size"),
        }
    }
}

pub fn play_tune(reg_tune1: [(u16, u8, u32, u32); reg_tune::TUNE_SIZE as usize]) {
    unsafe {
        CURRENT_TUNE_1 = reg_tune1;
    }
}

pub fn play_step() {
    unsafe {
        loop {
            //if LOOP_DELTA_COUNT < 0 || LOOP_DELTA_COUNT > 1{
            //    throw "Invalid LOOP_DELTA_COUNT value";
            //}

            let (next_time_step, size, addr, value) = CURRENT_TUNE_1[CURRENT_INDEX_TUNE_1]; // time step
            let wait_on_loop_back = LOOP_DELTA_COUNT != 0;
            if (next_time_step > CURRENT_TIME_STEP_TUNE_1) || (wait_on_loop_back) {
                break;
            }

            // write the value to the address
            write(size, addr, value);

            CURRENT_INDEX_TUNE_1 = (CURRENT_INDEX_TUNE_1 + 1) % reg_tune::TUNE_SIZE as usize;
            if CURRENT_INDEX_TUNE_1 == 0 {
                LOOP_DELTA_COUNT += 1;
            }
        }

        // update time steps
        CURRENT_TIME_STEP_TUNE_1 = (CURRENT_TIME_STEP_TUNE_1 + 1) % reg_tune::TUNE_LOOP_SIZE;
        if CURRENT_TIME_STEP_TUNE_1 == 0 {
            LOOP_DELTA_COUNT -= 1;
        }
    }
}
