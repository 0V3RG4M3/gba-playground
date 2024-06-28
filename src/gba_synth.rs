use gba::mmio;
use gba::sound::{
    LeftRightVolume, NoiseFrequency, NoiseLenEnvelope, PsgMix, SoundEnable, SoundMix, SweepControl,
    ToneFrequency,
};

use crate::static_sounds_lib;
use crate::tune;

pub fn init_synth() {
    // turn sound on
    mmio::SOUND_ENABLED.write(SoundEnable::new().with_enabled(true));

    mmio::LEFT_RIGHT_VOLUME.write(
        LeftRightVolume::new()
            .with_right_volume(15)
            .with_left_volume(15)
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
}

pub fn play_tone1(pitch: u16, velocity: u16) {
    let volume = velocity >> 3;
    let rate = static_sounds_lib::PITCH2RATE_MAP[(pitch) as usize];

    mmio::TONE1_PATTERN.write(static_sounds_lib::GLOCKENSPIEL_DAMPED.with_volume(volume));
    mmio::TONE1_FREQUENCY.write(ToneFrequency::new().with_frequency(rate).with_enabled(true));
}

pub fn play_tone2(pitch: u16, velocity: u16) {
    let volume = velocity >> 3;
    let rate = static_sounds_lib::PITCH2RATE_MAP[(pitch) as usize];

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
    // let formatted_string: String = format!("play_noise({}, {}, {}, {}, {}, {}, 127)", shift_5, div_code_3, counter7, stop_when_expired, length_6, step_time_3);

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

pub fn play_noise_drum(pitch: u16, velocity: u16) {
    let volume = velocity >> 3;

    let (noise_freq, noise_len_env): (NoiseFrequency, NoiseLenEnvelope) =
        static_sounds_lib::pitch2drum_map(pitch);

    mmio::NOISE_FREQ.write(noise_freq);
    mmio::NOISE_LEN_ENV.write(noise_len_env.with_volume(volume));
}

pub fn get_tune_step_count() -> u16 {
    return tune::TUNE_STEP_COUNT;
}

pub fn play_tune(step_id: u16) {
    let (pitch, velocity): (u16, u16) = tune::TUNE_TRACK1[step_id as usize];
    if pitch > 0 {
        play_tone1(pitch, velocity);
    }
    let (pitch, velocity): (u16, u16) = tune::TUNE_TRACK2[step_id as usize];
    if pitch > 0 {
        play_tone2(pitch, velocity);
    }
    let (pitch, velocity): (u16, u16) = tune::TUNE_DRUMS[step_id as usize];
    if pitch > 0 {
        play_noise_drum(pitch, velocity);
    }
}
