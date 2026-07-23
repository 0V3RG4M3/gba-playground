// This file has been automatically generated
use crate::gba_synth::RegTune;

pub const TUNE: RegTune =
    RegTune { data: &TUNE_DATA, data_size: TUNE_DATA_SIZE, loop_size: TUNE_LOOP_SIZE };

pub const TUNE_LOOP_SIZE: u16 = 58;
pub const TUNE_DATA_SIZE: u16 = 7;
#[unsafe(link_section = ".rodata")]
pub static TUNE_DATA: [(u16, u8, u32, u32); TUNE_DATA_SIZE as usize] = [
    (0, 1, 67108960, 39),
    (0, 2, 67108962, 658),
    (0, 2, 67108964, 34315),
    (2, 2, 67108962, 29330),
    (2, 2, 67108964, 34315),
    (57, 2, 67108962, 658),
    (57, 2, 67108964, 34315),
];
