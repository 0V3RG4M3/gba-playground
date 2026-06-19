# GBAudioNoiseChannel Structure Analysis

## Overview

The `GBAudioNoiseChannel` structure implements **Channel 4** of the Game Boy's audio system—the noise channel. This channel generates pseudo-random noise by combining two key components:

1. **Linear Feedback Shift Register (LFSR)**: Generates pseudo-random bit patterns
2. **Volume Envelope**: Controls the amplitude over time (see [envelope.md](envelope.md))

The noise channel is essential for percussion sounds, explosions, and sound effects in Game Boy games.

---

## Structure Definition

```c
struct GBAudioNoiseChannel {
    struct GBAudioEnvelope envelope;  // Volume envelope controller
    
    // LFSR frequency control
    int ratio;                         // Clock divisor (0-7)
    int frequency;                     // Shift clock frequency (0-15)
    bool power;                        // Width mode: false=15-bit, true=7-bit
    
    // Playback control
    bool stop;                         // Length counter enable flag
    int length;                        // Remaining length (0-64)
    
    // LFSR state
    uint32_t lfsr;                    // Linear Feedback Shift Register value
    
    // Sample accumulation (for averaging)
    int nSamples;                     // Number of accumulated samples
    int samples;                      // Sum of accumulated samples
    uint32_t lastEvent;               // Timestamp of last LFSR update
    
    // Output
    int8_t sample;                    // Current output sample value
};
```

---

## Component Integration: LFSR + Envelope

The noise channel combines the LFSR (for randomness) with the envelope (for volume control):

```
┌─────────────────────────────────────────────────┐
│         Noise Channel Signal Flow               │
└─────────────────────────────────────────────────┘

    ┌──────────┐         ┌──────────┐
    │   LFSR   │         │ Envelope │
    │  (15/7)  │         │  (0-15)  │
    └────┬─────┘         └────┬─────┘
         │                    │
         │ LSB = 0 or 1       │ currentVolume
         │                    │
         └────────┬───────────┘
                  │
              MULTIPLY
                  │
                  ▼
            ┌─────────┐
            │ sample  │  = LSB × currentVolume
            └─────────┘
```

### Mathematical Formula

```c
sample = (lfsr & 1) * envelope.currentVolume
```

- **LFSR bit** (0 or 1): Determines if sound is "on" or "off"
- **Envelope volume** (0-15): Sets the amplitude
- **Result** (0-15): Final sample value

---

## LFSR Clock Frequency

The LFSR is clocked at a configurable frequency determined by two parameters:

### Formula

```c
cycles = (ratio ? 2 * ratio : 1) << frequency;
cycles *= 8 * timingFactor;
```

### Parameters

| Parameter | Range | Description |
|-----------|-------|-------------|
| `ratio` | 0-7 | Base divisor: 0→1, 1→2, 2→4, ..., 7→14 |
| `frequency` | 0-15 | Shift multiplier: cycles = base × 2^frequency |
| `timingFactor` | 2 or 4 | Platform-dependent (DMG/CGB=2, GBA=4) |

### Example Frequencies

| ratio | frequency | cycles (DMG) | Approximate Hz |
|-------|-----------|--------------|----------------|
| 0 | 0 | 16 | ~131 kHz |
| 0 | 4 | 256 | ~8.2 kHz |
| 0 | 8 | 4096 | ~512 Hz |
| 4 | 0 | 128 | ~16 kHz |
| 4 | 4 | 2048 | ~1 kHz |
| 4 | 8 | 32768 | ~64 Hz |

---

## LFSR Update Algorithm

The LFSR generates pseudo-random bits through a feedback mechanism:

### Standard Update (per clock cycle)

```c
int lsb = (audio->ch4.lfsr ^ (audio->ch4.lfsr >> 1) ^ 1) & 1;
audio->ch4.lfsr >>= 1;

if (lsb) {
    audio->ch4.lfsr |= coeff;
} else {
    audio->ch4.lfsr &= ~coeff;
}
```

Where `coeff` depends on the width mode:
- **15-bit mode** (`power = false`): `coeff = 0x4000`
- **7-bit mode** (`power = true`): `coeff = 0x4040`

### Width Modes Comparison

| Mode | Bits | XOR Pattern | Period | Sound Character |
|------|------|-------------|--------|-----------------|
| 15-bit | 15 | `0x4000` | 32767 | White noise (smooth) |
| 7-bit | 7 | `0x4040` | 127 | Metallic/buzzy |

The 7-bit mode produces a shorter, more repetitive pattern, creating a distinctive "metallic" sound.

---

## Batch Processing Optimization

For performance, mGBA batches multiple LFSR updates when possible (15-bit mode only):

### Lookup Tables

```c
const uint16_t noiseMaskTable[0x40] = {
    0x3f, 0x3e, 0x3c, 0x3d, 0x39, 0x38, 0x3a, 0x3b,
    // ... computes 5 LFSR steps in one lookup
};

const uint16_t noisePopulationTable[0x40] = {
    6, 5, 4, 5, 4, 3, 4, 5, 4, 3, 2, 3, 4, 3, 4, 5,
    // ... counts positive bits in 5 steps
};
```

### Batch Update Code

```c
// Process 5 LFSR steps at once
int bits = audio->ch4.lfsr & 0x3F;
audio->ch4.lfsr >>= 5;
audio->ch4.lfsr |= 0x4000 * noiseMaskTable[bits] >> 4;
audio->ch4.lfsr &= 0x7FFF;

samples += 5;
positiveSamples += noisePopulationTable[bits];
```

This optimization processes 5 LFSR steps at once using precomputed lookup tables.

---

## Sample Accumulation

The noise channel accumulates samples for averaging, providing smoother output:

```c
audio->ch4.sample = lsb * audio->ch4.envelope.currentVolume;
audio->ch4.nSamples += samples;
audio->ch4.samples += positiveSamples * audio->ch4.envelope.currentVolume;
```

### Coalescing Function

```c
static int16_t _coalesceNoiseChannel(struct GBAudioNoiseChannel* ch) {
    if (ch->nSamples <= 1) {
        return ch->sample << 3;  // Single sample case
    }
    
    // Average multiple samples
    int16_t sample = (ch->samples << 3) / ch->nSamples;
    ch->nSamples = 0;
    ch->samples = 0;
    return sample;
}
```

**Purpose**: When multiple LFSR updates occur between audio samples, averaging prevents aliasing and produces cleaner sound.

---

## Envelope Integration

The noise channel uses the standard envelope system (see [envelope.md](envelope.md) for details):

### Envelope Update (Frame 7)

```c
if (audio->playingCh4 && !audio->ch4.envelope.dead) {
    --audio->ch4.envelope.nextStep;
    if (audio->ch4.envelope.nextStep == 0) {
        int8_t sample = audio->ch4.sample;
        _updateEnvelope(&audio->ch4.envelope);
        
        // Recompute sample with new volume
        audio->ch4.sample = (sample > 0) * audio->ch4.envelope.currentVolume;
        
        // Update accumulated samples
        if (audio->ch4.nSamples) {
            audio->ch4.samples -= sample;
            audio->ch4.samples += audio->ch4.sample;
        }
    }
}
```

**Key behavior**: When the envelope volume changes, the current sample is recalculated, and accumulated samples are updated accordingly.

---

## Register Control (NR4x)

The noise channel is controlled by four registers:

### NR41 - Sound Length

```c
void GBAudioWriteNR41(struct GBAudio* audio, uint8_t value) {
    _writeDuty(&audio->ch4.envelope, value);
    audio->ch4.length = 64 - audio->ch4.envelope.length;
}
```

**Format**:
```
Bit 5-0: Sound length (0-63)
Length in seconds = (64 - value) / 256
```

---

### NR42 - Volume Envelope

```c
void GBAudioWriteNR42(struct GBAudio* audio, uint8_t value) {
    if (!_writeEnvelope(&audio->ch4.envelope, value, audio->style)) {
        audio->playingCh4 = false;  // Disable if invalid
    }
}
```

**Format**:
```
Bit 7-4: Initial Volume (0-15)
Bit 3:   Direction (0=decrease, 1=increase)
Bit 2-0: Step Time (0-7, in 1/64 second units)
```

See [envelope.md](envelope.md) for detailed envelope behavior.

---

### NR43 - Polynomial Counter

```c
void GBAudioWriteNR43(struct GBAudio* audio, uint8_t value) {
    audio->ch4.ratio = GBAudioRegisterNoiseFeedbackGetRatio(value);
    audio->ch4.frequency = GBAudioRegisterNoiseFeedbackGetFrequency(value);
    audio->ch4.power = GBAudioRegisterNoiseFeedbackGetPower(value);
}
```

**Format**:
```
Bit 7-4: Shift Clock Frequency (0-15)
Bit 3:   Counter Width (0=15-bit, 1=7-bit)
Bit 2-0: Dividing Ratio (0-7)
```

---

### NR44 - Counter/Initial

```c
void GBAudioWriteNR44(struct GBAudio* audio, uint8_t value) {
    audio->ch4.stop = GBAudioRegisterNoiseControlGetStop(value);
    
    if (GBAudioRegisterNoiseControlIsRestart(value)) {
        audio->playingCh4 = _resetEnvelope(&audio->ch4.envelope, audio->style);
        audio->ch4.lfsr = 0;  // Reset LFSR
        audio->ch4.lastEvent = mTimingCurrentTime(audio->timing);
        
        if (!audio->ch4.length) {
            audio->ch4.length = 64;
        }
    }
}
```

**Format**:
```
Bit 7:   Initial (1=restart sound)
Bit 6:   Counter/continuous (1=use length counter)
Bit 5-0: Unused
```

**Important**: When restarted, the LFSR is reset to 0, ensuring consistent sound generation.

---

## Complete Signal Flow

### Initialization (NR44 restart)

```
1. Reset envelope → currentVolume = initialVolume
2. Reset LFSR → lfsr = 0
3. Set lastEvent timestamp
4. Reset length counter
```

### Per-Frame Update (Frame 7, ~64Hz)

```
1. Decrement envelope.nextStep
2. If nextStep == 0:
   a. Update envelope.currentVolume (+1 or -1)
   b. Recompute sample = (lfsr_bit) × currentVolume
   c. Update accumulated samples
```

### Per-Cycle Update (Variable frequency)

```
1. Calculate cycles since lastEvent
2. While cycles >= clockPeriod:
   a. Update LFSR (shift + feedback)
   b. Extract LSB → bit (0 or 1)
   c. sample = bit × currentVolume
   d. Accumulate for averaging
3. Update lastEvent
```

### Audio Sampling (Output)

```
1. Coalesce accumulated samples (average)
2. Apply left/right panning
3. Apply master volume
4. Output to audio buffer
```

---

## Practical Examples

### Example 1: Drum Hit

```c
// NR42: Volume 15, decrease, fast (step=1)
envelope.initialVolume = 15;
envelope.direction = false;
envelope.stepTime = 1;

// NR43: Mid frequency, 15-bit mode
ratio = 4;
frequency = 4;
power = false;

// Result: Sharp attack, quick decay
// Duration: ~0.23 seconds
```

### Example 2: Metallic Buzz

```c
// NR42: Volume 8, constant (step=0)
envelope.initialVolume = 8;
envelope.stepTime = 0;

// NR43: High frequency, 7-bit mode
ratio = 0;
frequency = 8;
power = true;

// Result: Continuous metallic buzz
```

### Example 3: White Noise Background

```c
// NR42: Volume 4, increase slowly
envelope.initialVolume = 4;
envelope.direction = true;
envelope.stepTime = 5;

// NR43: Low frequency, 15-bit mode
ratio = 6;
frequency = 12;
power = false;

// Result: Smooth white noise fade-in
```

---

## Key Differences from Square Channels

| Feature | Square Channels (1, 2) | Noise Channel (4) |
|---------|------------------------|-------------------|
| Waveform | Duty cycle pattern | Pseudo-random (LFSR) |
| Pitch control | Frequency register | ratio + frequency |
| Tonal | Yes (musical notes) | No (percussive) |
| Envelope | Yes ([envelope.md](envelope.md)) | Yes ([envelope.md](envelope.md)) |
| Sweep | Ch1 only | No |
| Width modes | No | Yes (7-bit / 15-bit) |

---

## Related Documentation

- **[sound.md](sound.md)**: Overview of all 4 audio channels and `GBAudioRun` function
- **[envelope.md](envelope.md)**: Detailed envelope system analysis

---

## Summary

The `GBAudioNoiseChannel` structure elegantly combines:

1. **LFSR**: Generates pseudo-random bits using feedback (7-bit or 15-bit)
2. **Envelope**: Controls volume over time with linear ramping
3. **Sample accumulation**: Averages multiple LFSR updates for cleaner output
4. **Optimization**: Batch processing for performance

The final output is computed as:

```
output = (LFSR_bit) × envelope.currentVolume
```

This simple formula produces a wide variety of noise effects essential to Game Boy audio, from drums and explosions to ambient sounds and percussive elements.
