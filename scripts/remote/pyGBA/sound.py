
import dataclasses
import abc


class RegData(abc.ABC):
    @abc.abstractmethod
    def value(self) -> int:
        """Return the integer value of the register data."""
        raise NotImplementedError


@dataclasses.dataclass
class SweepControl(RegData):
    sweep_num: int = 0  # Sweep number in [0, 7]
    sweep_increasing: bool = False  # True if the sweep is increasing
    sweep_time: int = 0  # Sweep time in [0, 7]

    def __post_init__(self):
        if not (0 <= self.sweep_num <= 7):
            raise ValueError("Sweep number must be between 0 and 7")
        if not (0 <= self.sweep_time <= 7):
            raise ValueError("Sweep time must be between 0 and 7")
    
    def value(self) -> int:
        return (self.sweep_num << 0) | (self.sweep_increasing << 3) | (self.sweep_time << 4)

@dataclasses.dataclass
class TonePattern(RegData):
    length: int = 0  # L in [0, 63]. Resulting length is: (64−val)/256 second. So L=0 -> 250 ms, and L=63 -> 3.9 ms
    duty: int = 0  # Duty cycle    0: 12.5%, 1: 25%, 2: 50%, 3: 75%
    step_time: int = 0  # envelope decay time in [0, 7]. 0: inf, 1: shortest 7: long
    step_increasing: bool = False
    volume: int = 0  # Volume value in [0, 15]

    def __post_init__(self):
        if not (0 <= self.length <= 63):
            raise ValueError("Length must be between 0 and 63")
        if not (0 <= self.duty <= 3):
            raise ValueError("Duty must be between 0 and 3")
        if not (0 <= self.step_time <= 7):
            raise ValueError("Step time must be between 0 and 7")
        if not (0 <= self.volume <= 15):
            raise ValueError("Volume must be between 0 and 15")
        
    def value(self) -> int:
        return (self.length << 0) | (self.duty << 6) | (self.step_time << 8) | (self.step_increasing << 11) | (self.volume << 12)
    
@dataclasses.dataclass
class ToneFrequency(RegData):
    frequency_rate: int = 0  # Frequency in [0, 2047]. The frequency is calculated as: 131072 / (2048 - frequency_rate)
    stop_when_expired: bool = False
    enabled: bool = False
    def __post_init__(self):
        if not (0 <= self.frequency_rate <= 2047):
            raise ValueError("Frequency must be between 0 and 2047")
        
    def value(self) -> int:
        return (self.frequency_rate << 0) | (self.stop_when_expired << 14) | (self.enabled << 15)


@dataclasses.dataclass
class LeftRightVolume(RegData):
    right_volume: int = 0  # Right volume in [0, 7]
    left_volume: int = 0  # Left volume in [0, 7]

    tone1_right: bool = False  # True if Tone 1 is enabled on the right channel
    tone2_right: bool = False  # True if Tone 2 is enabled on the right channel
    wave_right: bool = False  # True if Wave is enabled on the right channel
    noise_right: bool = False  # True if Noise is enabled on the right channel
    
    tone1_left: bool = False  # True if Tone 1 is enabled on the left channel
    tone2_left: bool = False  # True if Tone 2 is enabled on the left channel
    wave_left: bool = False  # True if Wave is enabled on the left channel  
    noise_left: bool = False

    def __post_init__(self):
        if not (0 <= self.right_volume <= 7):
            # bizarre comme dans sound.rs le volume n'utilise que 3 bits sur 4
            raise ValueError("Right volume must be between 0 and 7")
        if not (0 <= self.left_volume <= 7):
            raise ValueError("Left volume must be between 0 and 7")
    
    def value(self) -> int:
        return (self.right_volume << 0) | (self.left_volume << 4) | \
               (self.tone1_right << 8) | (self.tone2_right << 9) | \
               (self.wave_right << 10) | (self.noise_right << 11) | \
               (self.tone1_left << 12) | (self.tone2_left << 13) | \
               (self.wave_left << 14) | (self.noise_left << 15)
    

import enum
class PsgMix(enum.Enum):
    _25 = 0
    _50 = 1
    _100 = 2

    def value(self) -> int:
        return self.value

@dataclasses.dataclass
class SoundMix(RegData):
    psg: PsgMix = PsgMix._25
    sound_a_full: bool = False  # True if Sound A buffer is full
    sound_b_full: bool = False  # True if Sound B buffer is full
    
    sound_a_right: bool = False  # True if Sound A is enabled on the right channel
    sound_a_left: bool = False  # True if Sound A is enabled on the left channel
    sound_a_timer: bool = False  # True if Sound A timer is enabled
    sound_a_reset: bool = False  # True if Sound A is reset 

    sound_b_right: bool = False  # True if Sound B is enabled on the right channel
    sound_b_left: bool = False  # True if Sound B is enabled on the left channel
    sound_b_timer: bool = False  # True if Sound B timer is enabled
    sound_b_reset: bool = False  # True if Sound B is reset

    def value(self) -> int:
        return (self.psg.value() << 0) | (self.sound_a_full << 2) | (self.sound_b_full << 3) | \
               (self.sound_a_right << 4) | (self.sound_a_left << 5) | (self.sound_a_timer << 6) | \
               (self.sound_a_reset << 7) | (self.sound_b_right << 8) | (self.sound_b_left << 9) | \
               (self.sound_b_timer << 10) | (self.sound_b_reset << 11)


@dataclasses.dataclass
class SoundEnable(RegData):
    tone1_playing: bool = False
    tone2_playing: bool = False
    wave_playing: bool = False
    noise_playing: bool = False

    enabled: bool = False

    def value(self) -> int:
        return (self.tone1_playing << 0) | (self.tone2_playing << 1) | \
               (self.wave_playing << 2) | (self.noise_playing << 3) | \
               (self.enabled << 7)


class SampleCycle(enum.IntEnum):
    _9bit = 0
    _8bit = 1
    _7bit = 2
    _6bit = 3

@dataclasses.dataclass
class SoundBias(RegData):  # u16
    bias_level: int = 0  # Bias level in [0, 511] (9bits).
    sample_cycle: SampleCycle = 0 # Sample cycle in [0, 3] (2bits)

    def __post_init__(self):
        if not (1 <= self.bias_level <= 9):
            raise ValueError("Bias level must be between 1 and 9")
        if not (0 <= self.sample_cycle <= 3):
            raise ValueError("Sample cycle must be between 0 and 3")
        
    def value(self) -> int:
        return (self.bias_level << 1) | (self.sample_cycle << 14)

