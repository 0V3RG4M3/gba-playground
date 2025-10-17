"""
Defines data structures and types for GBA sound registers as in gba::sound.rs
used to validate and format register values before sending them to the emulator.
"""

import dataclasses


@dataclasses.dataclass
class Field:
    name: str
    size: int  # in bits
    value: int = 0

    def __str__(self):
        return f"{self.value}/{(1 << self.size) - 1}"

    def __repr__(self):
        return self.to_repr(self.value)
    
class FieldUnused(Field):
    def __init__(self, size: int):
        super().__init__(name="", size=size, value=0)


class RegData:
    @classmethod
    def empty_fields(cls) -> list[Field]:
        """Return a list of Field(name, size, value) representing the structure of the register."""
        raise NotImplementedError

    @staticmethod
    def create_from_value(cls, val: int):
        fields = []
        bit_pos = 0
        for field in cls.empty_fields():
            field_value = (val >> bit_pos) & ((1 << field.size) - 1)
            fields.append(field_value)
            bit_pos += field.size

        return cls(*fields)

    def fields(self) -> list[Field]:
        filled_fields = []
        for field in self.empty_fields():
            field.value = self.__dict__[field.name]
            filled_fields.append(field)
        return filled_fields

    def value(self) -> int:
        res = 0
        bit_pos = 0
        for field in self.fields():
            res |= (field.value << bit_pos)
            bit_pos += field.size
        return res

    def __post_init__(self):
        for field in self.fields():
            if not (0 <= field.value < (1 << field.size)):
                raise ValueError(f"{field.name} must be between 0 and {(1 << field.size) - 1}")

    def to_string(self) -> tuple[str, str]:
        def render0(size, txt) -> str:
            w = len(txt)
            txt = txt[:min(w, size)]
            padright = max(0, (size - w) // 2)
            padleft = max(0, size - w - padright)
            padchar = "."
            return f"{padchar * padleft}{txt}{padchar * padright}"

        # print values chained with | with "-" filling the gaps to get bit size right

        rendered_headers = "|"
        rendered_values = "|"
        for field in self.fields():
            size = field.size*2
            rendered_headers += render0(size, field.name) + "|"

            #max_val = (1 << field.size) - 1
            #rendered_values += render0(size, f"{field.value}/{max_val}") + "|"
            rendered_values += render0(size, f"{field.value}") + "|"

        return rendered_headers, rendered_values


@dataclasses.dataclass
class SweepControl(RegData):
    sweep_num: int = 0  # Sweep number in [0, 7]
    sweep_increasing: bool = False  # True if the sweep is increasing
    sweep_time: int = 0  # Sweep time in [0, 7]

    @classmethod
    def empty_fields(cls) -> list[Field]:
        return [
            Field("sweep_num", 3),
            Field("sweep_increasing", 1),
            Field("sweep_time", 3),
        ]

@dataclasses.dataclass
class TonePattern(RegData):
    length: int = 0  # L in [0, 63]. Resulting length is: (64−val)/256 second. So L=0 -> 250 ms, and L=63 -> 3.9 ms
    duty: int = 0  # Duty cycle    0: 12.5%, 1: 25%, 2: 50%, 3: 75%
    step_time: int = 0  # envelope decay time in [0, 7]. 0: inf, 1: shortest 7: long
    step_increasing: int = 0  # True if the envelope is increasing
    volume: int = 0  # Volume value in [0, 15]

    @classmethod
    def empty_fields(cls) -> list[Field]:
        return [
            Field("length", 6),
            Field("duty", 2),
            Field("step_time", 3),
            Field("step_increasing", 1),
            Field("volume", 4),
        ]

@dataclasses.dataclass
class ToneFrequency(RegData):
    frequency_rate: int = 0  # Frequency in [0, 2047]. The frequency is calculated as: 131072 / (2048 - frequency_rate)
    stop_when_expired: int = 0
    enabled: int = 0

    @classmethod
    def empty_fields(cls) -> list[Field]:
        return [
            Field("frequency_rate", 11),
            Field("stop_when_expired", 4),
            Field("enabled", 1),
        ]

@dataclasses.dataclass
class LeftRightVolume(RegData):
    right_volume: int = 0  # Right volume in [0, 7]
    left_volume: int = 0  # Left volume in [0, 7]

    tone1_right: int = 0  # True if Tone 1 is enabled on the right channel
    tone2_right: int = 0  # True if Tone 2 is enabled on the right channel
    wave_right: int = 0  # True if Wave is enabled on the right channel
    noise_right: int = 0  # True if Noise is enabled on the right channel

    tone1_left: int = 0  # True if Tone 1 is enabled on the left channel
    tone2_left: int = 0  # True if Tone 2 is enabled on the left channel
    wave_left: int = 0  # True if Wave is enabled on the left channel
    noise_left: int = 0

    @classmethod
    def empty_fields(cls) -> list[Field]:
        return [
            Field("right_volume", 3),
            Field("", 1),  # unused bit
            Field("left_volume", 3),
            Field("", 1),  # unused bit
            Field("tone1_right", 1),
            Field("tone2_right", 1),
            Field("wave_right", 1),
            Field("noise_right", 1),
            Field("tone1_left", 1),
            Field("tone2_left", 1),
            Field("wave_left", 1),
            Field("noise_left", 1),
        ]


import enum


@dataclasses.dataclass
class SoundMix(RegData):
    psg: int = 0  # PSG output level in (2bits). 0: 25%, 1: 50%, 2: 100%, 3: not used
    sound_a_full: int = 0  # True if Sound A buffer is full
    sound_b_full: int = 0  # True if Sound B buffer is full

    sound_a_right: int = 0  # True if Sound A is enabled on the right channel
    sound_a_left: int = 0  # True if Sound A is enabled on the left channel
    sound_a_timer: int = 0  # True if Sound A timer is enabled
    sound_a_reset: int = 0  # True if Sound A is reset

    sound_b_right: int = 0  # True if Sound B is enabled on the right channel
    sound_b_left: int = 0  # True if Sound B is enabled on the left channel
    sound_b_timer: int = 0  # True if Sound B timer is enabled
    sound_b_reset: int = 0  # True if Sound B is reset

    def empty_fields(cls) -> list[Field]:
        return [
            Field("psg", 2),
            Field("sound_a_full", 1),
            Field("sound_b_full", 1),
            Field("sound_a_right", 1),
            Field("sound_a_left", 1),
            Field("sound_a_timer", 1),
            Field("sound_a_reset", 1),
            Field("sound_b_right", 1),
            Field("sound_b_left", 1),
            Field("sound_b_timer", 1),
            Field("sound_b_reset", 1),
        ]


@dataclasses.dataclass
class SoundEnable(RegData):
    tone1_playing: int = 0
    tone2_playing: int = 0
    wave_playing: int = 0
    noise_playing: int = 0

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
    sample_cycle: SampleCycle = 0  # Sample cycle in [0, 3] (2bits)

    def __post_init__(self):
        if not (1 <= self.bias_level <= 9):
            raise ValueError("Bias level must be between 1 and 9")
        if not (0 <= self.sample_cycle <= 3):
            raise ValueError("Sample cycle must be between 0 and 3")

    def value(self) -> int:
        return (self.bias_level << 1) | (self.sample_cycle << 14)


def create_reg_data_from_value(cls: RegData, val: int) -> RegData:
    fields = []
    bit_pos = 0
    for field in cls.empty_fields():
        field_value = (val >> bit_pos) & ((1 << field.size) - 1)
        fields.append(field_value)
        bit_pos += field.size

    return cls(*fields)

""" 
Rust code to translate to Python dataclass:
pub struct NoiseLenEnvelope(u16);
impl NoiseLenEnvelope {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 5, length, with_length);
  u16_int_field!(8 - 10, step_time, with_step_time);
  u16_bool_field!(11, step_increasing, with_step_increasing);
  u16_int_field!(12 - 15, volume, with_volume);
}
"""
@dataclasses.dataclass
class NoiseLenEnvelope(RegData):
    length: int = 0  # Length in [0, 63]. Resulting length is: (64−val)/256 second. So L=0 -> 250 ms, and L=63 -> 3.9 ms
    step_time: int = 0  # envelope decay time in [0, 7]. 0: inf, 1: shortest 7: long
    step_increasing: int = 0  # True if the envelope is increasing
    volume: int = 0  # Volume value in [0, 15]

    @classmethod
    def empty_fields(cls) -> list[Field]:
        return [
            Field("length", 6),
            FieldUnused(2),  # unused bits
            Field("step_time", 3),
            Field("step_increasing", 1),
            Field("volume", 4),
        ]
    
"""
Rust code to translate to Python dataclass:
pub struct NoiseFrequency(u16);
impl NoiseFrequency {
  pub_const_fn_new_zeroed!();
  u16_int_field!(0 - 2, r, with_r);
  u16_bool_field!(3, counter7, with_counter7);
  u16_int_field!(4 - 7, s, with_s);
  u16_bool_field!(14, stop_when_expired, with_stop_when_expired);
  u16_bool_field!(15, enabled, with_enabled);
}
"""
@dataclasses.dataclass
class NoiseFrequency(RegData):
    rate: int = 0 # r in [0, 7] divisor code
    counter7: int = 0 # 
    shift: int = 0 # s in [0, 15] clock shift
    stop_when_expired: int = 0 # True if the sound should stop when the length expires
    enabled: int = 0 # True if the sound is enabled

    @classmethod
    def empty_fields(cls) -> list[Field]:
        return [
            Field("rate", 3),
            Field("counter7", 1),
            Field("shift", 4),
            FieldUnused(6),
            Field("stop_when_expired", 1),
            Field("enabled", 1),
        ]