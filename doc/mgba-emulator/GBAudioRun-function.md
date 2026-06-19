# Game Boy Audio Emulation Report: `GBAudioRun` Function Analysis

## Overview

The following document provides an analysis of the `GBAudioRun` function from the mGBA emulator, focusing on its implementation.

The source code can be found in the official mGBA repository:
- [src/gb/audio.c](https://github.com/mgba-emu/mgba/blob/master/src/gb/audio.c)  
  

The `GBAudioRun` function is the core audio processing loop in mGBA's Game Boy emulation. It advances the audio state by processing all enabled sound channels up to a given timestamp. The function handles 4 PSG (Programmable Sound Generator) channels:

1. **Channel 1**: Square wave with sweep
2. **Channel 2**: Square wave
3. **Channel 3**: Wavetable/PCM
4. **Channel 4**: Noise (using LFSR)

---

## Function Signature

```c
void GBAudioRun(struct GBAudio* audio, int32_t timestamp, int channels)
```

**Parameters:**
- `audio`: Pointer to the audio state structure
- `timestamp`: Target timestamp to advance audio to
- `channels`: Bitmask of which channels to process (bits 0-3 for channels 1-4)

---

## Channel 1: Square Wave with Sweep

Channel 1 generates a square wave and supports frequency sweep functionality.

### Code

```c
if (audio->ch1Left || audio->ch1Right) {
    while (audio->ch1.nextEvent <= timestamp) {
        audio->ch1.nextEvent += audio->ch1.control.frequency * 4 + 4;
        audio->ch1.sample = _squareChannelDuty[audio->ch1.envelope.duty][audio->ch1.control.hi];
        audio->ch1.control.hi = (audio->ch1.control.hi + 1) & 7;
    }
}
```

### Explanation

1. **Frequency calculation**: The channel advances when `nextEvent` reaches the current timestamp
2. **Event spacing**: `audio->ch1.control.frequency * 4 + 4` determines how many cycles until the next update
3. **Duty cycle lookup**: `_squareChannelDuty[duty][hi]` provides the waveform shape (12.5%, 25%, 50%, or 75% duty)
4. **Phase increment**: `hi` is incremented and wraps at 8, cycling through the duty pattern

---

## Channel 2: Square Wave

Channel 2 is similar to Channel 1 but without sweep functionality.

### Code

```c
if (audio->ch2Left || audio->ch2Right) {
    while (audio->ch2.nextEvent <= timestamp) {
        audio->ch2.nextEvent += audio->ch2.control.frequency * 4 + 4;
        audio->ch2.sample = _squareChannelDuty[audio->ch2.envelope.duty][audio->ch2.control.hi];
        audio->ch2.control.hi = (audio->ch2.control.hi + 1) & 7;
    }
}
```

### Explanation

The logic is identical to Channel 1:
- Uses the same frequency calculation formula
- Accesses the same duty cycle lookup table
- 8-step phase counter with wraparound

---

## Channel 3: Wavetable/PCM

Channel 3 plays custom waveforms stored in wave RAM (32 4-bit samples).

### Code

```c
if (audio->ch3Left || audio->ch3Right) {
    while (audio->ch3.nextEvent <= timestamp) {
        audio->ch3.nextEvent += (2048 - audio->ch3.rate) * 2;
        if (audio->playingCh3) {
            audio->ch3.sample = audio->ch3.wavedata8[audio->ch3.window >> 1];
            if (audio->ch3.window & 1) {
                audio->ch3.sample &= 0xF;
            } else {
                audio->ch3.sample >>= 4;
            }
            audio->ch3.window = (audio->ch3.window + 1) & 0x1F;
        }
    }
}
```

### Explanation

1. **Frequency**: `(2048 - rate) * 2` determines the sample playback rate
2. **Wave RAM access**: `wavedata8[window >> 1]` reads from the 16-byte wave RAM
3. **Nibble selection**: The `window & 1` check determines whether to use the upper or lower 4 bits
4. **Position increment**: `window` advances through 32 positions (0-31), wrapping around

---

## Channel 4: Noise (LFSR Implementation) ⭐

Channel 4 generates pseudo-random noise using a **Linear Feedback Shift Register (LFSR)**. 

> **📖 For detailed analysis of the noise channel, see [GBAudioNoiseChannel-struct.md](GBAudioNoiseChannel-struct.md)**

### Key Characteristics

- **Waveform**: Pseudo-random noise (not tonal)
- **LFSR modes**: 7-bit (metallic) or 15-bit (white noise)
- **Volume control**: Uses [envelope system](GBAudioEnvelope-struct.md)
- **Applications**: Percussion, explosions, ambient effects

### Simplified Code (from `GBAudioRun`)

The actual implementation in `GBAudioRun` is more complex and includes optimizations like batch processing. See [GBAudioNoiseChannel-struct.md](GBAudioNoiseChannel-struct.md) for the complete analysis.

**Basic LFSR update**:
```c
// Extract LSB and shift
int lsb = (audio->ch4.lfsr ^ (audio->ch4.lfsr >> 1) ^ 1) & 1;
audio->ch4.lfsr >>= 1;

// Apply feedback (tap pattern depends on width mode)
if (lsb) {
    audio->ch4.lfsr |= audio->ch4.power ? 0x4040 : 0x4000;
}

// Generate sample with envelope volume
audio->ch4.sample = lsb * audio->ch4.envelope.currentVolume;
```

### LFSR Modes

| Mode | Bits | Period | Sound Quality |
|------|------|--------|---------------|
| 15-bit | 15 | 32767 | White noise (smooth) |
| 7-bit | 7 | 127 | Metallic/buzzy |

---

## Summary

The `GBAudioRun` function processes each channel independently:

- **Channels 1 & 2**: Generate square waves using duty cycle lookup tables
- **Channel 3**: Plays custom waveforms from wave RAM
- **Channel 4**: Uses an LFSR to generate pseudo-random noise (see [GBAudioNoiseChannel-struct.md](GBAudioNoiseChannel-struct.md))

All channels (except Channel 3) use the [envelope system](GBAudioEnvelope-struct.md) for volume control.heard in Game Boy games.

---

## Related Documentation

- **[GBAudioEnvelope-struct.md](GBAudioEnvelope-struct.md)**: Detailed envelope system analysis
- **[GBAudioNoiseChannel-struct.md](GBAudioNoiseChannel-struct.md)**: Complete noise channel (LFSR) documentation

---

## Source Code Reference

The complete source code can be found in the official mGBA repository:
- [`src/gb/audio.c`](https://github.com/mgba-emu/mgba/blob/master/src/gb/audio.c) - Main audio implementation
- [`include/mgba/internal/gb/audio.h`](https://github.com/mgba-emu/mgba/blob/master/include/mgba/internal/gb/audio.h) - Structure definitions
