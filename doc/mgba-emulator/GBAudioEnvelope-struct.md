# Game Boy Audio Envelope System

## Overview

The Game Boy audio system uses a **simplified envelope generator** that is **NOT** a full ADSR (Attack-Decay-Sustain-Release) envelope like in modern synthesizers. Instead, it implements a much simpler **single-stage linear volume ramping** system that can either increase or decrease volume over time.

### Key Difference from ADSR

| Feature | Game Boy Envelope | Traditional ADSR |
|---------|-------------------|------------------|
| **Stages** | 1 (single ramp) | 4 (Attack, Decay, Sustain, Release) |
| **Complexity** | Simple linear ramp | Multi-stage with curves |
| **Control** | Direction + step time + initial volume | Multiple time/level parameters |
| **Sustain** | No sustain stage | Has sustain level |
| **Release** | No release stage | Triggered on note off |

---

## Structure Definition

```c
struct GBAudioEnvelope {
    int length;           // Sound length (used for note duration, not envelope)
    int duty;             // Duty cycle for square waves (12.5%, 25%, 50%, 75%)
    int stepTime;         // Time between volume changes (in 64Hz steps)
    int initialVolume;    // Starting volume (0-15)
    int currentVolume;    // Current volume (0-15)
    bool direction;       // true = increase volume, false = decrease volume
    int dead;             // Status flag: 0 = active, 1 = max volume, 2 = silent
    int nextStep;         // Countdown to next volume change
};
```

---

## How the Envelope Works

### 1. Initialization (`_resetEnvelope`)

When a sound is triggered, the envelope is reset:

```c
bool _resetEnvelope(struct GBAudioEnvelope* envelope, enum GBAudioStyle style) {
    envelope->currentVolume = envelope->initialVolume;
    envelope->nextStep = envelope->stepTime;
    _updateEnvelopeDead(envelope, style);
    return envelope->initialVolume || envelope->direction;
}
```

**Key points:**
- Current volume starts at `initialVolume` (0-15)
- `nextStep` is set to `stepTime` (countdown timer)
- Returns `false` if the envelope won't produce sound (volume 0 and decreasing)

---

### 2. Volume Update (`_updateEnvelope`)

Every frame (at 64Hz), if the envelope is active and `nextStep` reaches 0, the volume is updated:

```c
static void _updateEnvelope(struct GBAudioEnvelope* envelope) {
    if (envelope->direction) {
        ++envelope->currentVolume;      // Increase volume
    } else {
        --envelope->currentVolume;      // Decrease volume
    }
    
    if (envelope->currentVolume >= 15) {
        envelope->currentVolume = 15;
        envelope->dead = 1;             // Reached maximum
    } else if (envelope->currentVolume <= 0) {
        envelope->currentVolume = 0;
        envelope->dead = 2;             // Reached silence
    } else {
        envelope->nextStep = envelope->stepTime;  // Reset timer
    }
}
```

**Behavior:**
- **Direction = true** (increase): Volume goes from `initialVolume` → 15
- **Direction = false** (decrease): Volume goes from `initialVolume` → 0
- **Linear ramping**: +1 or -1 per step
- **Clamping**: Volume stays at 0 or 15 once reached (no wraparound)

---

### 3. Frame Sequencer Integration

The envelope is updated by the frame sequencer at **64Hz** (every 8 frames at ~512Hz base rate):

```c
case 7:  // Frame 7 of 8 - envelope update
    if (audio->playingCh1 && !audio->ch1.envelope.dead) {
        --audio->ch1.envelope.nextStep;
        if (audio->ch1.envelope.nextStep == 0) {
            _updateEnvelope(&audio->ch1.envelope);
            _updateSquareSample(&audio->ch1);
        }
    }
    
    // Similar for ch2 and ch4...
```

**Update frequency:**
- Base rate: 64Hz (once per frame sequencer cycle)
- Actual rate: `64Hz / stepTime`
- If `stepTime = 0`, no updates occur (envelope disabled)

---

## Envelope Patterns and Use Cases

### Pattern 1: Fade In (Attack-like)

```c
initialVolume = 0;
direction = true;   // Increase
stepTime = 3;       // Update every 3/64 seconds
```

**Result:** Volume ramps from 0 → 15 over ~0.7 seconds
- Used for: Smooth sound entry

---

### Pattern 2: Fade Out (Release-like)

```c
initialVolume = 15;
direction = false;  // Decrease
stepTime = 1;       // Update every 1/64 seconds
```

**Result:** Volume ramps from 15 → 0 over ~0.23 seconds
- Used for: Percussion, short sound effects

---

### Pattern 3: Sustain (No Envelope)

```c
initialVolume = 12;
direction = true;   // Increase (but...)
stepTime = 0;       // No updates!
```

**Result:** Volume stays at 12 indefinitely
- Used for: Constant-volume sounds, drones

---

### Pattern 4: Quick Attack

```c
initialVolume = 8;
direction = true;
stepTime = 1;       // Fast updates
```

**Result:** Volume ramps from 8 → 15 over ~0.11 seconds
- Used for: Punchy sounds with quick start

---

## Channels Using Envelopes

### Channels 1, 2, and 4 (Square + Noise)

All three use the full envelope system. See also:
- [GBAudioNoiseChannel-struct.md](GBAudioNoiseChannel-struct.md) for detailed noise channel analysis
- [GBAudioRun-function.md](GBAudioRun-function.md) for channel processing overview

```c
struct GBAudioSquareChannel {
    struct GBAudioSweep sweep;
    struct GBAudioEnvelope envelope;  // <-- Has envelope
    struct GBAudioSquareControl control;
    // ...
};

struct GBAudioNoiseChannel {
    struct GBAudioEnvelope envelope;  // <-- Has envelope
    // ...
};
```

**Volume application:**
```c
// For square waves (Channels 1 & 2)
ch->sample = _squareChannelDuty[ch->envelope.duty][ch->index] * ch->envelope.currentVolume;

// For noise (Channel 4)
audio->ch4.sample = (audio->ch4.lfsr & 1) * audio->ch4.envelope.currentVolume;
```

---

### Channel 3 (Wave)

Channel 3 does **NOT** use the envelope system. Instead, it has a simple 2-bit volume control:

```c
struct GBAudioWaveChannel {
    // No envelope member!
    int volume;  // Simple volume control (0-3)
    // ...
};
```

**Volume levels:**
- 0: Muted (sample >> 4)
- 1: Full volume (sample >> 0)
- 2: Half volume (sample >> 1)
- 3: Quarter volume (sample >> 2)

---

## Dead State Management

The `dead` flag prevents unnecessary processing:

```c
static void _updateEnvelopeDead(struct GBAudioEnvelope* envelope, enum GBAudioStyle style) {
    if (!envelope->stepTime) {
        envelope->dead = envelope->currentVolume ? 1 : 2;
    } else if (!envelope->direction && !envelope->currentVolume) {
        envelope->dead = 2;  // Silent and decreasing
    } else if (envelope->direction && envelope->currentVolume == 0xF) {
        envelope->dead = 1;  // Max volume and increasing
    } else if (envelope->dead) {
        // TODO: Figure out if this happens on DMG/CGB or just AGB
```

**Dead states:**
- `0`: Active (envelope can still change)
- `1`: Stuck at maximum (volume = 15, direction = increase)
- `2`: Silent (volume = 0, direction = decrease)

When `dead != 0`, the frame sequencer skips envelope updates:
```c
if (audio->playingCh1 && !audio->ch1.envelope.dead) {
    // Only update if not dead
}
```

---

## Comparison: Game Boy vs. Modern ADSR

### Game Boy Envelope
```
Volume
  15 ┤     ╱─────  (direction=true, reaches max and stops)
     │    ╱
     │   ╱
   8 ┤  ╱
     │ ╱
   0 ┴──────────
     Time
```

### Traditional ADSR
```
Volume
  15 ┤   ╱╲
     │  ╱  ╲___   (separate Attack, Decay, Sustain, Release)
     │ ╱       ╲__
   8 ┤╱          ╲
     │            ╲
   0 ┴──────────────╲
     A  D  S      R
     Time
```

---

## Register Mapping

The envelope parameters are set via NRx2 registers:

```c
bool _writeEnvelope(struct GBAudioEnvelope* envelope, uint8_t value, enum GBAudioStyle style) {
    envelope->stepTime = GBAudioRegisterSweepGetStepTime(value);        // Bits 0-2
    envelope->direction = GBAudioRegisterSweepGetDirection(value);      // Bit 3
    envelope->initialVolume = GBAudioRegisterSweepGetInitialVolume(value); // Bits 4-7
    // ...
}
```

**NRx2 Register format:**
```
Bit 7-4: Initial Volume (0-15)
Bit 3:   Direction (0=decrease, 1=increase)
Bit 2-0: Step Time (0-7, in 1/64 second units, 0=disabled)
```

---

## Summary

The Game Boy envelope system is **NOT an ADSR**. It's a **single-stage linear volume ramp** that can either:
- **Increase** volume from initial → 15 (attack-like)
- **Decrease** volume from initial → 0 (release-like)
- **Hold** constant volume when stepTime = 0 (sustain-like)

This simplicity is due to the hardware limitations of the Game Boy's audio chip, but it's sufficient for creating a wide variety of sound effects and musical tones characteristic of 8-bit games.

**Key characteristics:**
- ✅ Simple linear ramping
- ✅ One direction per note
- ✅ Updates at 64Hz
- ✅ Volume range 0-15
- ❌ No multi-stage envelope
- ❌ No exponential curves
- ❌ No sustain level control
- ❌ No release trigger
