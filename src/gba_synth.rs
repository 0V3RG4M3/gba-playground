use gba::mmio;
use gba::sound::{
    LeftRightVolume, NoiseFrequency, NoiseLenEnvelope, PsgMix, SoundEnable, SoundMix, SweepControl,
    ToneFrequency,
};

use crate::log4gba;

use crate::static_sounds_lib;
use crate::tune;

static mut TUNE_CURRENT_TIME_STEP: u16 = 0;
static mut SFX_CURRENT_TIME_STEP: u16 = 0;
static mut IS_SFX_PLAYING: bool = false;
const SFX_STEP_COUNT: u16 = 30;
static mut CURRENT_SFX: [(u8, u8); SFX_STEP_COUNT as usize] = [(0, 0); SFX_STEP_COUNT as usize];
static mut CURRENT_TUNE_1: [(u8, u8); tune::TUNE_STEP_COUNT as usize] =
    [(0, 0); tune::TUNE_STEP_COUNT as usize];
static mut CURRENT_TUNE_2: [(u8, u8); tune::TUNE_STEP_COUNT as usize] =
    [(0, 0); tune::TUNE_STEP_COUNT as usize];
static mut CURRENT_TUNE_DRUMS: [(u8, u8); tune::TUNE_STEP_COUNT as usize] =
    [(0, 0); tune::TUNE_STEP_COUNT as usize];

pub fn init_synth() {
    // turn sound on
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

    // disable the sweep of tone 1 (to disable, set sweep time to 0)
    mmio::TONE1_SWEEP.write(SweepControl::new().with_sweep_time(0));

    mmio::TONE1_PATTERN.write(static_sounds_lib::GLOCKENSPIEL);
    mmio::TONE1_FREQUENCY.write(ToneFrequency::new().with_frequency(0));

    mmio::TONE2_PATTERN.write(static_sounds_lib::GLOCKENSPIEL);
    mmio::TONE2_FREQUENCY.write(ToneFrequency::new().with_frequency(0));

    play_tune(tune::TUNE_TRACK1, tune::TUNE_TRACK2, tune::TUNE_DRUMS);
}

pub fn play_tone1(pitch: u8, velocity: u8) {
    let volume = (velocity >> 3) as u16;
    let rate = static_sounds_lib::PITCH2RATE_MAP[pitch as usize];

    mmio::TONE1_PATTERN.write(static_sounds_lib::GLOCKENSPIEL_DAMPED.with_volume(volume));
    mmio::TONE1_FREQUENCY.write(ToneFrequency::new().with_frequency(rate).with_enabled(true));
}

pub fn play_tone2(pitch: u8, velocity: u8) {
    let volume = (velocity >> 3) as u16;
    let rate = static_sounds_lib::PITCH2RATE_MAP[pitch as usize];

    mmio::TONE2_PATTERN.write(static_sounds_lib::GLOCKENSPIEL_DAMPED.with_volume(volume));
    mmio::TONE2_FREQUENCY.write(ToneFrequency::new().with_frequency(rate).with_enabled(true));
}

pub fn play_noise(
    shift_5: u16,
    div_code_3: u16,
    counter7: bool,
    stop_when_expired: bool,
    length_6: u16,
    step_time_3: u16,
    velocity: u16,
) {
    let volume = velocity >> 3;

    mmio::NOISE_LEN_ENV.write(
        mmio::NOISE_LEN_ENV
            .read()
            .with_volume(volume)
            .with_length(length_6)
            .with_step_time(step_time_3),
    );

    mmio::NOISE_FREQ.write(
        mmio::NOISE_FREQ
            .read()
            .with_r(div_code_3)
            .with_s(shift_5)
            .with_counter7(counter7)
            .with_stop_when_expired(stop_when_expired)
            .with_enabled(true),
    );
}

pub fn play_noise_drum(pitch: u8, velocity: u8) {
    let volume = (velocity >> 3) as u16;

    let (noise_freq, noise_len_env): (NoiseFrequency, NoiseLenEnvelope) =
        static_sounds_lib::pitch2drum_map(pitch as u16);

    mmio::NOISE_FREQ.write(noise_freq);
    mmio::NOISE_LEN_ENV.write(noise_len_env.with_volume(volume));
}

pub fn get_tune_step_count() -> u16 {
    return tune::TUNE_STEP_COUNT;
}

pub fn play_sfx(midi_array: [(u8, u8); SFX_STEP_COUNT as usize]) {
    unsafe {
        IS_SFX_PLAYING = true;
        SFX_CURRENT_TIME_STEP = 0;
        CURRENT_SFX = midi_array;
    }
}

pub fn play_tune(
    track1: [(u8, u8); tune::TUNE_STEP_COUNT as usize],
    track2: [(u8, u8); tune::TUNE_STEP_COUNT as usize],
    drums: [(u8, u8); tune::TUNE_STEP_COUNT as usize],
) {
    log4gba::debug("ok");
    unsafe {
        CURRENT_TUNE_1 = track1;
        CURRENT_TUNE_2 = track2;
        CURRENT_TUNE_DRUMS = drums;
        TUNE_CURRENT_TIME_STEP = 0;
    }
}

pub fn play_step() {
    unsafe {
        // GBA TONE 1
        let (pitch, velocity): (u8, u8);
        if IS_SFX_PLAYING {
            (pitch, velocity) = CURRENT_SFX[SFX_CURRENT_TIME_STEP as usize]
        } else {
            (pitch, velocity) = CURRENT_TUNE_1[TUNE_CURRENT_TIME_STEP as usize];
        }
        if pitch > 0 {
            play_tone1(pitch, velocity);
        }

        // GBA TONE 2
        let (pitch, velocity): (u8, u8) = CURRENT_TUNE_2[TUNE_CURRENT_TIME_STEP as usize];
        if pitch > 0 {
            play_tone2(pitch, velocity);
        }

        // GBA NOISE
        let (pitch, velocity): (u8, u8) = CURRENT_TUNE_DRUMS[TUNE_CURRENT_TIME_STEP as usize];
        if pitch > 0 {
            play_noise_drum(pitch, velocity);
        }

        // update time steps
        TUNE_CURRENT_TIME_STEP = (TUNE_CURRENT_TIME_STEP + 1) % tune::TUNE_STEP_COUNT;

        if IS_SFX_PLAYING {
            SFX_CURRENT_TIME_STEP = SFX_CURRENT_TIME_STEP + 1;

            if SFX_CURRENT_TIME_STEP >= SFX_STEP_COUNT {
                IS_SFX_PLAYING = false;
            }
        }
    }
}
