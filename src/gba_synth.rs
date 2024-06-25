use gba::mmio;
use gba::sound::{
    LeftRightVolume, PsgMix, SoundEnable, SoundMix, SweepControl, ToneFrequency, TonePattern,
};

use crate::log4gba;
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
            .with_tone1_right(true)
            .with_tone2_right(true),
    );

    mmio::SOUND_MIX.write(SoundMix::new().with_psg(PsgMix::_25));

    // disable the sweep of tone 1 (to disable, set sweep time to 0)
    mmio::TONE1_SWEEP.write(SweepControl::new().with_sweep_time(0));

    const GLOCKENSPIEL: TonePattern = TonePattern::new()
        .with_volume(15) // Volume value in [0, 15]
        .with_duty(2) // Duty cycle    0: 12.5%, 1: 25%, 2: 50%, 3: 75%
        .with_length(0) // L in [0, 63]. Resulting length is: (64−val)/256 second. So L=0 -> 250 ms, and L=63 -> 3.9 ms
        .with_step_increasing(false)
        .with_step_time(7); // envelope decay time in [0, 7]. 0: inf, 1: shortest 7: long

    // const GLOCKENSPIEL_DAMPED: TonePattern = GLOCKENSPIEL
    //     .with_step_time(2); // envelope decay time in [0, 7]. 0: inf, 1: shortest 7: long

    mmio::TONE1_PATTERN.write(GLOCKENSPIEL);
    mmio::TONE1_FREQUENCY.write(ToneFrequency::new().with_frequency(0));

    mmio::TONE2_PATTERN.write(GLOCKENSPIEL);
    mmio::TONE2_FREQUENCY.write(ToneFrequency::new().with_frequency(0));
}

const PITCH2RATE_MAP: [u16; 92] = [
    45, 156, 262, 362, 458, 547, 632, 712, 786, 856, 923, 986, 1046, 1102, 1155, 1205, 1253, 1297,
    1340, 1380, 1417, 1452, 1486, 1517, 1547, 1575, 1602, 1627, 1650, 1673, 1694, 1714, 1732, 1750,
    1767, 1783, 1798, 1812, 1825, 1837, 1849, 1860, 1871, 1881, 1890, 1899, 1907, 1915, 1923, 1930,
    1936, 1943, 1949, 1954, 1959, 1964, 1969, 1974, 1978, 1982, 1985, 1989, 1992, 1995, 1998, 2001,
    2004, 2006, 2009, 2011, 2013, 2015, 2017, 2018, 2020, 2022, 2023, 2025, 2026, 2027, 2028, 2029,
    2030, 2031, 2032, 2033, 2034, 2035, 2036, 2036, 2037, 2038,
];

pub fn play_tone1(pitch: u16, velocity: u16) {
    log4gba::debug(pitch);
    log4gba::debug(velocity);

    let volume = velocity >> 3;
    let rate = PITCH2RATE_MAP[(pitch - 36) as usize];
    log4gba::debug(rate);

    mmio::TONE1_PATTERN.write(mmio::TONE1_PATTERN.read().with_volume(volume));
    mmio::TONE1_FREQUENCY.write(ToneFrequency::new().with_frequency(rate).with_enabled(true));
}

pub fn play_tone2(pitch: u16, velocity: u16) {
    let volume = velocity >> 3;
    let rate = PITCH2RATE_MAP[(pitch - 36) as usize];

    mmio::TONE2_PATTERN.write(mmio::TONE2_PATTERN.read().with_volume(volume));
    mmio::TONE2_FREQUENCY.write(ToneFrequency::new().with_frequency(rate).with_enabled(true));
}

pub fn get_tune_step_count() -> u16 {
    return tune::TUNE_STEP_COUNT;
}

pub fn play_tune(step_id: u16) {
    let note1 = tune::TUNE_TRACK1[step_id as usize];
    if note1.0 > 0 {
        play_tone1(note1.0, note1.1);
    }
    let note2 = tune::TUNE_TRACK2[step_id as usize];
    if note2.0 > 0 {
        play_tone2(note2.0, note2.1);
    }
}
