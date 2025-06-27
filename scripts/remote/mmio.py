"""
def_mmio!(0x0400_0060 = TONE1_SWEEP/["SOUND1CNT_L","NR10"]: VolAddress<SweepControl, Safe, Safe>; "Tone 1 Sweep");
def_mmio!(0x0400_0062 = TONE1_PATTERN/["SOUND1CNT_H","NR11","NR12"]: VolAddress<TonePattern, Safe, Safe>; "Tone 1 Duty/Len/Envelope");
def_mmio!(0x0400_0064 = TONE1_FREQUENCY/["SOUND1CNT_X","NR13","NR14"]: VolAddress<ToneFrequency, Safe, Safe>; "Tone 1 Frequency/Control");

def_mmio!(0x0400_0068 = TONE2_PATTERN/["SOUND2CNT_L","NR21","NR22"]: VolAddress<TonePattern, Safe, Safe>; "Tone 2 Duty/Len/Envelope");
def_mmio!(0x0400_006C = TONE2_FREQUENCY/["SOUND2CNT_H","NR23","NR24"]: VolAddress<ToneFrequency, Safe, Safe>; "Tone 2 Frequency/Control");

def_mmio!(0x0400_0070 = WAVE_BANK/["SOUND3CNT_L","NR30"]: VolAddress<WaveBank, Safe, Safe>; "Wave banking controls");
def_mmio!(0x0400_0072 = WAVE_LEN_VOLUME/["SOUND3CNT_H","NR31","NR32"]: VolAddress<WaveLenVolume, Safe, Safe>; "Wave Length/Volume");
def_mmio!(0x0400_0074 = WAVE_FREQ/["SOUND3CNT_X","NR33","NR34"]: VolAddress<WaveFrequency, Safe, Safe>; "Wave Frequency/Control");

def_mmio!(0x0400_0078 = NOISE_LEN_ENV/["SOUND4CNT_L","NR41","NR42"]: VolAddress<NoiseLenEnvelope, Safe, Safe>; "Noise Length/Envelope");
def_mmio!(0x0400_007C = NOISE_FREQ/["SOUND4CNT_H","NR43","NR44"]: VolAddress<NoiseFrequency, Safe, Safe>; "Noise Frequency/Control");

def_mmio!(0x0400_0080 = LEFT_RIGHT_VOLUME/["SOUNDCNT_L","NR50","NR51"]: VolAddress<LeftRightVolume, Safe, Safe>;"Left/Right sound control (but GBAs only have one speaker each).");
def_mmio!(0x0400_0082 = SOUND_MIX/["SOUNDCNT_H"]: VolAddress<SoundMix, Safe, Safe>;"Mixes sound sources out to the left and right");
def_mmio!(0x0400_0084 = SOUND_ENABLED/["SOUNDCNT_X"]: VolAddress<SoundEnable, Safe, Safe>;"Sound active flags (r), as well as the sound primary enable (rw).");
def_mmio!(0x0400_0088 = SOUNDBIAS: VolAddress<SoundBias, Safe, Safe>;"Provides a bias to set the 'middle point' of sound output.");

def_mmio!(0x0400_0090 = WAVE_RAM/["WAVE_RAM0_L","WAVE_RAM0_H","WAVE_RAM1_L","WAVE_RAM1_H","WAVE_RAM2_L","WAVE_RAM2_H","WAVE_RAM3_L","WAVE_RAM3_H"]: VolBlock<u32, Safe, Safe, 4>; "Wave memory, `u4`, plays MSB/LSB per byte.");
def_mmio!(0x0400_00A0 = FIFO_A/["FIFO_A_L", "FIFO_A_H"]: VolAddress<u32, (), Safe>; "Pushes 4 `i8` samples into the Sound A buffer.\n\nThe buffer is 32 bytes max, playback is LSB first.");
def_mmio!(0x0400_00A4 = FIFO_B/["FIFO_B_L", "FIFO_B_H"]: VolAddress<u32, (), Safe>; "Pushes 4 `i8` samples into the Sound B buffer.\n\nThe buffer is 32 bytes max, playback is LSB first.");
"""
import dataclasses
import sound

@dataclasses.dataclass
class Register:
    ADDRESS: int
    SIZE: int
    DATA_TYPE: type[sound.RegData]
    CNAME: str
    DESCRIPTION: str

    def write_cmd(self, data: sound.RegData) -> str:

        return f"WRITE{self.SIZE*8} {hex(self.ADDRESS)} {hex(data.value())}"

TONE1_SWEEP = Register(ADDRESS=0x04000060, SIZE=1, CNAME="SOUND1CNT_L", DATA_TYPE=sound.SweepControl, DESCRIPTION="Tone 1 Sweep control")
TONE1_PATTERN = Register(ADDRESS=0x04000062, SIZE=2, CNAME="SOUND1CNT_H", DATA_TYPE=sound.TonePattern, DESCRIPTION="Tone 1 Duty/Length/Envelope")
TONE1_FREQUENCY = Register(ADDRESS=0x04000064, SIZE=2, CNAME="SOUND1CNT_X", DATA_TYPE=sound.ToneFrequency, DESCRIPTION="Tone 1 Frequency/Control")

LEFT_RIGHT_VOLUME = Register(ADDRESS=0x04000080, SIZE=2, CNAME="SOUNDCNT_L", DATA_TYPE=sound.LeftRightVolume, DESCRIPTION="Left/Right sound control")
SOUND_MIX = Register(ADDRESS=0x04000082, SIZE=2, CNAME="SOUNDCNT_H", DATA_TYPE=sound.SoundMix, DESCRIPTION="Sound mix control")
SOUND_ENABLED = Register(ADDRESS=0x04000084, SIZE=1, CNAME="SOUNDCNT_X", DATA_TYPE=sound.SoundEnable, DESCRIPTION="Sound enable control")
SOUNDBIAS = Register(ADDRESS=0x04000088, SIZE=2, CNAME="SOUNDBIAS", DATA_TYPE=sound.SoundBias, DESCRIPTION="Sound bias control")