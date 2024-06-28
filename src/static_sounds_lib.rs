use gba::sound::{NoiseFrequency, NoiseLenEnvelope, TonePattern};

// -----------------------------------------------
// -------------- DRUMS --------------------------
// -----------------------------------------------
pub const KICK1_NF: NoiseFrequency = NoiseFrequency::new()
.with_r(5)  //Divisor code, 0:8, 1:16, 2:32, 3:48 etc... 7:112
    .with_s(6)  // in [0, 15], Clock shift, 14 or 15 results in the LFSR receiving no clocks.
    .with_counter7(true)  // Width mode of LFSR
    .with_stop_when_expired(true)
    .with_enabled(true);

pub const KICK1_NLE: NoiseLenEnvelope = NoiseLenEnvelope::new()
    .with_length(11)  // in [0, 63]
    .with_step_time(1);  // in [0, 7]

pub const KICK2_NF: NoiseFrequency = NoiseFrequency::new()
    .with_r(9)
    .with_s(1)
    .with_counter7(false)
    .with_stop_when_expired(true)
    .with_enabled(true);

pub const KICK2_NLE: NoiseLenEnvelope = NoiseLenEnvelope::new().with_length(26).with_step_time(1);

pub const SNARE1_NF: NoiseFrequency = NoiseFrequency::new()
    .with_r(7)
    .with_s(1)
    .with_counter7(false)
    .with_stop_when_expired(true)
    .with_enabled(true);

pub const SNARE1_NLE: NoiseLenEnvelope = NoiseLenEnvelope::new().with_length(26).with_step_time(1);

pub const SNARE2_NF: NoiseFrequency = NoiseFrequency::new()
    .with_r(6)
    .with_s(1)
    .with_counter7(false)
    .with_stop_when_expired(true)
    .with_enabled(true);

pub const SNARE2_NLE: NoiseLenEnvelope = NoiseLenEnvelope::new().with_length(26).with_step_time(1);

pub const HIHAT_NF: NoiseFrequency = NoiseFrequency::new()
    .with_r(2)
    .with_s(2)
    .with_counter7(false)
    .with_stop_when_expired(true)
    .with_enabled(true);

pub const HIHAT_NLE: NoiseLenEnvelope = NoiseLenEnvelope::new().with_length(51).with_step_time(1);

pub const OPEN_HIHAT_NF: NoiseFrequency = NoiseFrequency::new()
    .with_r(0)
    .with_s(2)
    .with_counter7(false)
    .with_stop_when_expired(false)
    .with_enabled(true);

pub const OPEN_HIHAT_NLE: NoiseLenEnvelope =
    NoiseLenEnvelope::new().with_length(51).with_step_time(1);

pub const CLAP_NF: NoiseFrequency = NoiseFrequency::new()
    .with_r(4)
    .with_s(7)
    .with_counter7(true)
    .with_stop_when_expired(true)
    .with_enabled(true);

pub const CLAP_NLE: NoiseLenEnvelope = NoiseLenEnvelope::new().with_length(47).with_step_time(0);

pub fn pitch2drum_map(pitch: u16) -> (NoiseFrequency, NoiseLenEnvelope) {
    match pitch {
        // Match a single value
        36 => (KICK1_NF, KICK1_NLE),
        38 => (SNARE1_NF, SNARE1_NLE),
        39 => (CLAP_NF, CLAP_NLE),
        40 => (SNARE2_NF, SNARE2_NLE),
        41 => (KICK2_NF, KICK2_NLE),
        42 => (HIHAT_NF, HIHAT_NLE),
        46 => (OPEN_HIHAT_NF, OPEN_HIHAT_NLE),
        _ => (KICK1_NF, KICK1_NLE),
    }
}

// -----------------------------------------------
// -------------- SOUND EFFECTS ------------------
// -----------------------------------------------
pub const SFX_CASTLE_CRUSH_NF: NoiseFrequency = NoiseFrequency::new()
    .with_r(8)
    .with_s(2)
    .with_counter7(false)
    .with_stop_when_expired(true)
    .with_enabled(true);

pub const SFX_CASTLE_CRUSH_NLE: NoiseLenEnvelope =
    NoiseLenEnvelope::new().with_length(8).with_step_time(1);

pub const SFX_ELECTRIC_ARC_NF: NoiseFrequency = NoiseFrequency::new()
    .with_r(3)
    .with_s(6)
    .with_counter7(true)
    .with_stop_when_expired(false)
    .with_enabled(true);

pub const SFX_ELECTRIC_ARC_NLE: NoiseLenEnvelope =
    NoiseLenEnvelope::new().with_length(10).with_step_time(2);

// -----------------------------------------------
// -------------- TONAL INSTRUMENTS --------------
// -----------------------------------------------

pub const GLOCKENSPIEL: TonePattern = TonePattern::new()
    .with_volume(15) // Volume value in [0, 15]
    .with_duty(2) // Duty cycle    0: 12.5%, 1: 25%, 2: 50%, 3: 75%
    .with_length(0) // L in [0, 63]. Resulting length is: (64−val)/256 second. So L=0 -> 250 ms, and L=63 -> 3.9 ms
    .with_step_increasing(false)
    .with_step_time(7); // envelope decay time in [0, 7]. 0: inf, 1: shortest 7: long

pub const GLOCKENSPIEL_DAMPED: TonePattern = GLOCKENSPIEL.with_step_time(2); // envelope decay time in [0, 7]. 0: inf, 1: shortest 7: long

// convert midi pitch in[0, 127] to rates used in gba tone1 and tone2. lowest available pitch is 36
pub const PITCH2RATE_MAP: [u16; 128] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 45, 156, 262, 362, 458, 547, 632, 712, 786, 856, 923, 986, 1046, 1102, 1155, 1205,
    1253, 1297, 1340, 1380, 1417, 1452, 1486, 1517, 1547, 1575, 1602, 1627, 1650, 1673, 1694, 1714,
    1732, 1750, 1767, 1783, 1798, 1812, 1825, 1837, 1849, 1860, 1871, 1881, 1890, 1899, 1907, 1915,
    1923, 1930, 1936, 1943, 1949, 1954, 1959, 1964, 1969, 1974, 1978, 1982, 1985, 1989, 1992, 1995,
    1998, 2001, 2004, 2006, 2009, 2011, 2013, 2015, 2017, 2018, 2020, 2022, 2023, 2025, 2026, 2027,
    2028, 2029, 2030, 2031, 2032, 2033, 2034, 2035, 2036, 2036, 2037, 2038,
];


/*
SFX: Castle crush
[DEBUG] GBA Debug:	8
[DEBUG] GBA Debug:	2
[DEBUG] GBA Debug:	false
[DEBUG] GBA Debug:	true
[DEBUG] GBA Debug:	8
[DEBUG] GBA Debug:	1

SFX: Electric arc (add randomness on shift and div_code)
[DEBUG] GBA Debug:	3
[DEBUG] GBA Debug:	6
[DEBUG] GBA Debug:	true
[DEBUG] GBA Debug:	false
[DEBUG] GBA Debug:	10
[DEBUG] GBA Debug:	2

Kick 1
[DEBUG] GBA Debug:	5
[DEBUG] GBA Debug:	6
[DEBUG] GBA Debug:	true
[DEBUG] GBA Debug:	true
[DEBUG] GBA Debug:	11
[DEBUG] GBA Debug:	1

Kick 2
[DEBUG] GBA Debug:	9
[DEBUG] GBA Debug:	1
[DEBUG] GBA Debug:	false
[DEBUG] GBA Debug:	true
[DEBUG] GBA Debug:	26
[DEBUG] GBA Debug:	1

Snare 1
[DEBUG] GBA Debug:	7
[DEBUG] GBA Debug:	1
[DEBUG] GBA Debug:	false
[DEBUG] GBA Debug:	true
[DEBUG] GBA Debug:	26
[DEBUG] GBA Debug:	1

Snare 2
[DEBUG] GBA Debug:	6
[DEBUG] GBA Debug:	1
[DEBUG] GBA Debug:	false
[DEBUG] GBA Debug:	true
[DEBUG] GBA Debug:	26
[DEBUG] GBA Debug:	1

HiHat
[DEBUG] GBA Debug:	2
[DEBUG] GBA Debug:	2
[DEBUG] GBA Debug:	false
[DEBUG] GBA Debug:	true
[DEBUG] GBA Debug:	51
[DEBUG] GBA Debug:	1

Open HiHat
[DEBUG] GBA Debug:	0
[DEBUG] GBA Debug:	2
[DEBUG] GBA Debug:	false
[DEBUG] GBA Debug:	false
[DEBUG] GBA Debug:	51
[DEBUG] GBA Debug:	1

Clap
[DEBUG] GBA Debug:	4
[DEBUG] GBA Debug:	7
[DEBUG] GBA Debug:	true
[DEBUG] GBA Debug:	true
[DEBUG] GBA Debug:	47
[DEBUG] GBA Debug:	0
*/