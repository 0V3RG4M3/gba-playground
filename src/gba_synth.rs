use gba::mmio;
use gba::mmio::{Safe, VolAddress};
use gba::sound::{LeftRightVolume, PsgMix, SoundEnable, SoundMix};

pub struct RegTune {
    pub data: &'static [(u16, u8, u32, u32)],
    pub data_size: u16,
    pub loop_size: u16,
}

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
        Self { entries: [(0, 0, 0); 16], count: 0, dirty_mask: 0 }
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

pub struct GbaSynth {
    loop_delta_count: i8,
    current_time_step_tune_1: u16,
    current_index_tune_1: usize,
    current_tune: Option<&'static RegTune>,
    music_registers: SoundRegisters,
    sfx_registers: SoundRegisters,
    sfx_playing: bool,
    sfx_just_ended: bool,
    sfx_tune: Option<&'static RegTune>,
    sfx_index: usize,
    sfx_time_step: u16,
    sfx_loop_delta: i8,
    sfx_steps_remaining: u16,
}

impl GbaSynth {
    pub const fn new() -> Self {
        Self {
            loop_delta_count: 0,
            current_time_step_tune_1: 0,
            current_index_tune_1: 0,
            current_tune: None,
            music_registers: SoundRegisters::new(),
            sfx_registers: SoundRegisters::new(),
            sfx_playing: false,
            sfx_just_ended: false,
            sfx_tune: None,
            sfx_index: 0,
            sfx_time_step: 0,
            sfx_loop_delta: 0,
            sfx_steps_remaining: 0,
        }
    }

    pub fn init(&mut self, tune: &'static RegTune) {
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
        self.current_tune = Some(tune);
        self.current_index_tune_1 = 0;
        self.current_time_step_tune_1 = 0;
        self.loop_delta_count = 0;
    }

    /// Advance the music sequencer by one frame and buffer any register writes
    /// into `music_registers`.  Does not touch hardware directly — call
    /// `write_to_registers` afterwards to flush.
    pub fn play_step(&mut self) {
        let Some(tune_obj) = self.current_tune else { return; };
        let tune = tune_obj.data;
        let tune_size = tune_obj.data_size as usize;
        let tune_loop_size = tune_obj.loop_size;
        
        loop {
            let line = tune[self.current_index_tune_1];
            let frame_id = line.0;
            let byte_count = line.1;
            let register_addr = line.2;
            let value = line.3;

            let wait_on_loop_back = self.loop_delta_count != 0;
            if (frame_id > self.current_time_step_tune_1) || wait_on_loop_back {
                break;
            }

            self.music_registers.set(register_addr, byte_count, value);

            self.current_index_tune_1 = (self.current_index_tune_1 + 1) % tune_size;
            if self.current_index_tune_1 == 0 {
                self.loop_delta_count += 1;
            }
        }

        self.current_time_step_tune_1 =
            (self.current_time_step_tune_1 + 1) % tune_loop_size;
        if self.current_time_step_tune_1 == 0 {
            self.loop_delta_count -= 1;
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
    pub fn write_to_registers(&mut self) {
        if self.sfx_just_ended {
            for i in 0..self.sfx_registers.count {
                let (addr, _, _) = self.sfx_registers.entries[i];
                if let Some((size, value)) = self.music_registers.get(addr) {
                    let restore_value =
                        if is_trigger_register(addr) { value & !0x8000u32 } else { value };
                    write(size, addr, restore_value);
                }
            }
            self.sfx_just_ended = false;
            self.sfx_registers.clear_dirty();
        } else if self.sfx_playing {
            for i in 0..self.sfx_registers.count {
                if self.sfx_registers.dirty_mask & (1 << i) != 0 {
                    let (addr, size, value) = self.sfx_registers.entries[i];
                    write(size, addr, value);
                }
            }
            self.sfx_registers.clear_dirty();
            self.music_registers.clear_dirty();
        } else {
            for i in 0..self.music_registers.count {
                if self.music_registers.dirty_mask & (1 << i) != 0 {
                    let (addr, size, value) = self.music_registers.entries[i];
                    write(size, addr, value);
                }
            }
            self.music_registers.clear_dirty();
        }
    }

    /// Start playing a sound effect.  The SFX tune uses the same
    /// `(time_step, size, addr, value)` format as the music tune.
    /// Call `play_sound_effect` and `write_to_registers` every frame afterwards.
    pub fn trigger_sfx(&mut self, tune: &'static RegTune) {
        self.sfx_tune = Some(tune);
        self.sfx_index = 0;
        self.sfx_time_step = 0;
        self.sfx_loop_delta = 0;
        self.sfx_steps_remaining = tune.loop_size;
        self.sfx_registers.reset();
        self.sfx_playing = true;
        self.sfx_just_ended = false;
    }

    /// Advance the SFX sequencer by one frame and buffer any register writes into
    /// `sfx_registers`.  Sets `sfx_just_ended` when the SFX loop completes so
    /// that the next `write_to_registers` call performs the music restore.
    pub fn play_sound_effect(&mut self) {
        if !self.sfx_playing {
            return;
        }

        let Some(tune_obj) = self.sfx_tune else { return; };
        let tune = tune_obj.data;
        let tune_size = tune_obj.data_size as usize;
        let tune_loop_size = tune_obj.loop_size;
        
        loop {
            let line = tune[self.sfx_index];
            let frame_id = line.0;
            let byte_count = line.1;
            let register_addr = line.2;
            let value = line.3;
            let wait_on_loop_back = self.sfx_loop_delta != 0;
            if (frame_id > self.sfx_time_step) || wait_on_loop_back {
                break;
            }

            self.sfx_registers.set(register_addr, byte_count, value);

            self.sfx_index = (self.sfx_index + 1) % tune_size;
            if self.sfx_index == 0 {
                self.sfx_loop_delta += 1;
            }
        }

        self.sfx_time_step = (self.sfx_time_step + 1) % tune_loop_size;
        if self.sfx_time_step == 0 {
            self.sfx_loop_delta -= 1;
        }

        self.sfx_steps_remaining -= 1;
        if self.sfx_steps_remaining == 0 {
            self.sfx_playing = false;
            self.sfx_just_ended = true;
        }
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
