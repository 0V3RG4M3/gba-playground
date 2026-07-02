// GBA code is strictly single-threaded; references to mutable statics are safe here.
#![allow(static_mut_refs)]

use gba::mmio;
use gba::mmio::{Safe, VolAddress};
use gba::sound::{LeftRightVolume, PsgMix, SoundEnable, SoundMix};

// ── SoundRegisters ────────────────────────────────────────────────────────────
// A dictionary keyed by register address, storing the most recent (size, value)
// written to each address.  dirty_mask tracks which entries were written this
// frame so that write_to_registers only flushes new writes to hardware — this
// prevents re-triggering channels on every frame.

pub struct SoundRegisters {
    entries: [(u32, u8, u32); 16], // (addr, size, value)
    count: usize,
    dirty_mask: u16, // bit i = entries[i] was written this frame
}

impl SoundRegisters {
    pub const fn new() -> Self {
        Self {
            entries: [(0, 0, 0); 16],
            count: 0,
            dirty_mask: 0,
        }
    }

    /// Upsert by address and mark the entry dirty.
    pub fn set(&mut self, addr: u32, size: u8, value: u32) {
        for i in 0..self.count {
            if self.entries[i].0 == addr {
                self.entries[i] = (addr, size, value);
                self.dirty_mask |= 1 << i;
                return;
            }
        }
        if self.count < 16 {
            self.entries[self.count] = (addr, size, value);
            self.dirty_mask |= 1 << self.count;
            self.count += 1;
        }
    }

    /// Clear all dirty flags (call after flushing to hardware).
    pub fn clear_dirty(&mut self) {
        self.dirty_mask = 0;
    }

    /// Look up the most recent (size, value) stored for an address.
    pub fn get(&self, addr: u32) -> Option<(u8, u32)> {
        for i in 0..self.count {
            if self.entries[i].0 == addr {
                return Some((self.entries[i].1, self.entries[i].2));
            }
        }
        None
    }

    pub fn contains(&self, addr: u32) -> bool {
        self.get(addr).is_some()
    }

    /// Erase all entries and dirty flags (used when a new SFX starts).
    pub fn reset(&mut self) {
        self.count = 0;
        self.dirty_mask = 0;
    }
}

// ── Registers whose bit 15 ("enabled") restarts the audio channel ─────────────
// TONE1_FREQUENCY (0x04000064), TONE2_FREQUENCY (0x0400006C), NOISE_FREQ (0x0400007C).
// When restoring music after SFX, this bit must be cleared so we silently
// restore frequency/envelope state without re-triggering the channel.
const TRIGGER_REGISTERS: [u32; 3] = [0x04000064, 0x0400006C, 0x0400007C];

fn is_trigger_register(addr: u32) -> bool {
    TRIGGER_REGISTERS[0] == addr || TRIGGER_REGISTERS[1] == addr || TRIGGER_REGISTERS[2] == addr
}

// ── Static state ──────────────────────────────────────────────────────────────

static mut LOOP_DELTA_COUNT: i8 = 0;
static mut CURRENT_TIME_STEP_TUNE_1: u16 = 0;
static mut CURRENT_INDEX_TUNE_1: usize = 0;
static mut CURRENT_TUNE_1: *const (u16, u8, u32, u32) = core::ptr::null();
static mut CURRENT_TUNE_SIZE: usize = 0;
static mut CURRENT_TUNE_LOOP_SIZE: u16 = 0;

static mut MUSIC_REGISTERS: SoundRegisters = SoundRegisters::new();
static mut SFX_REGISTERS: SoundRegisters = SoundRegisters::new();

static mut SFX_PLAYING: bool = false;
static mut SFX_JUST_ENDED: bool = false;

static mut SFX_TUNE: *const (u16, u8, u32, u32) = core::ptr::null();
static mut SFX_TUNE_SIZE: usize = 0;
static mut SFX_LOOP_SIZE: u16 = 0;
static mut SFX_INDEX: usize = 0;
static mut SFX_TIME_STEP: u16 = 0;
static mut SFX_LOOP_DELTA: i8 = 0;
static mut SFX_STEPS_REMAINING: u16 = 0;

// ── Public API ────────────────────────────────────────────────────────────────

pub fn init_synth(tune: &'static [(u16, u8, u32, u32)], tune_size: u16, tune_loop_size: u16) {
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
    unsafe {
        CURRENT_TUNE_1 = tune.as_ptr();
        CURRENT_TUNE_SIZE = tune_size as usize;
        CURRENT_TUNE_LOOP_SIZE = tune_loop_size;
        CURRENT_INDEX_TUNE_1 = 0;
        CURRENT_TIME_STEP_TUNE_1 = 0;
        LOOP_DELTA_COUNT = 0;
    }
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

/// Advance the music sequencer by one frame and buffer any register writes
/// into `MUSIC_REGISTERS`.  Does not touch hardware directly — call
/// `write_to_registers` afterwards to flush.
pub fn play_step() {
    unsafe {
        let tune = core::slice::from_raw_parts(CURRENT_TUNE_1, CURRENT_TUNE_SIZE);
        loop {
            let (next_time_step, size, addr, value) = tune[CURRENT_INDEX_TUNE_1];
            let wait_on_loop_back = LOOP_DELTA_COUNT != 0;
            if (next_time_step > CURRENT_TIME_STEP_TUNE_1) || wait_on_loop_back {
                break;
            }

            MUSIC_REGISTERS.set(addr, size, value);

            CURRENT_INDEX_TUNE_1 = (CURRENT_INDEX_TUNE_1 + 1) % CURRENT_TUNE_SIZE;
            if CURRENT_INDEX_TUNE_1 == 0 {
                LOOP_DELTA_COUNT += 1;
            }
        }

        CURRENT_TIME_STEP_TUNE_1 = (CURRENT_TIME_STEP_TUNE_1 + 1) % CURRENT_TUNE_LOOP_SIZE;
        if CURRENT_TIME_STEP_TUNE_1 == 0 {
            LOOP_DELTA_COUNT -= 1;
        }
    }
}

/// Flush the active `SoundRegisters` to GBA hardware.
///
/// Priority rules:
/// - SFX just ended → restore music state for every register the SFX touched,
///   clearing the trigger bit (bit 15) on channel-frequency registers so the
///   channel restores silently without a retrigger.
/// - SFX playing → write SFX dirty entries; discard music dirty entries.
/// - Otherwise → write music dirty entries.
///
/// Call once per frame, after `play_step` and `play_sound_effect`.
pub fn write_to_registers() {
    unsafe {
        if SFX_JUST_ENDED {
            for i in 0..SFX_REGISTERS.count {
                let (addr, _, _) = SFX_REGISTERS.entries[i];
                if let Some((size, value)) = MUSIC_REGISTERS.get(addr) {
                    let restore_value = if is_trigger_register(addr) {
                        value & !0x8000u32
                    } else {
                        value
                    };
                    write(size, addr, restore_value);
                }
            }
            SFX_JUST_ENDED = false;
            SFX_REGISTERS.clear_dirty();
        } else if SFX_PLAYING {
            for i in 0..SFX_REGISTERS.count {
                if SFX_REGISTERS.dirty_mask & (1 << i) != 0 {
                    let (addr, size, value) = SFX_REGISTERS.entries[i];
                    write(size, addr, value);
                }
            }
            SFX_REGISTERS.clear_dirty();
            MUSIC_REGISTERS.clear_dirty();
        } else {
            for i in 0..MUSIC_REGISTERS.count {
                if MUSIC_REGISTERS.dirty_mask & (1 << i) != 0 {
                    let (addr, size, value) = MUSIC_REGISTERS.entries[i];
                    write(size, addr, value);
                }
            }
            MUSIC_REGISTERS.clear_dirty();
        }
    }
}

/// Start playing a sound effect.  The SFX tune uses the same
/// `(time_step, size, addr, value)` format as the music tune.
/// Call `play_sound_effect` and `write_to_registers` every frame afterwards.
pub fn trigger_sfx(tune: &'static [(u16, u8, u32, u32)], tune_size: u16, tune_loop_size: u16) {
    unsafe {
        SFX_TUNE = tune.as_ptr();
        SFX_TUNE_SIZE = tune_size as usize;
        SFX_LOOP_SIZE = tune_loop_size;
        SFX_INDEX = 0;
        SFX_TIME_STEP = 0;
        SFX_LOOP_DELTA = 0;
        SFX_STEPS_REMAINING = tune_loop_size;
        SFX_REGISTERS.reset();
        SFX_PLAYING = true;
        SFX_JUST_ENDED = false;
    }
}

/// Advance the SFX sequencer by one frame and buffer any register writes into
/// `SFX_REGISTERS`.  Sets `SFX_JUST_ENDED` when the SFX loop completes so
/// that the next `write_to_registers` call performs the music restore.
pub fn play_sound_effect() {
    unsafe {
        if !SFX_PLAYING {
            return;
        }

        let tune = core::slice::from_raw_parts(SFX_TUNE, SFX_TUNE_SIZE);
        loop {
            let (next_time_step, size, addr, value) = tune[SFX_INDEX];
            let wait_on_loop_back = SFX_LOOP_DELTA != 0;
            if (next_time_step > SFX_TIME_STEP) || wait_on_loop_back {
                break;
            }

            SFX_REGISTERS.set(addr, size, value);

            SFX_INDEX = (SFX_INDEX + 1) % SFX_TUNE_SIZE;
            if SFX_INDEX == 0 {
                SFX_LOOP_DELTA += 1;
            }
        }

        SFX_TIME_STEP = (SFX_TIME_STEP + 1) % SFX_LOOP_SIZE;
        if SFX_TIME_STEP == 0 {
            SFX_LOOP_DELTA -= 1;
        }

        SFX_STEPS_REMAINING -= 1;
        if SFX_STEPS_REMAINING == 0 {
            SFX_PLAYING = false;
            SFX_JUST_ENDED = true;
        }
    }
}


