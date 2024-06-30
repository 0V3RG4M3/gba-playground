import numpy as np
import os
from miditoolkit.midi import parser as mid_parser
from miditoolkit.midi import containers as ct
import utils

BAR_COUNT = 8
BEAT_PER_BAR = 4
STEPS_PER_BEAT = 32

STEP_COUNT = STEPS_PER_BEAT * BEAT_PER_BAR * BAR_COUNT


def parse_file(midi_path: str, bpm_gain: float = 1) -> list[tuple[int, int]]:
    # load midi file
    mido_obj = mid_parser.MidiFile(midi_path)
    tick2frame = STEPS_PER_BEAT / mido_obj.ticks_per_beat

    assert len(mido_obj.instruments) == 1
    instrument = mido_obj.instruments[0]

    # fill track
    track = [(0, 0)] * STEP_COUNT
    for note in instrument.notes:
        frame_id = int(round(note.start * tick2frame / bpm_gain))
        track[frame_id] = (note.pitch, note.velocity)

    return track


def trim_00(midi_array: list[tuple[int, int]], margin:int)->list[tuple[int, int]]:
    last_occurence = 0
    for i, note in enumerate(midi_array):
        if note != (0, 0):
            last_occurence = i
    midi_array = midi_array[:last_occurence+margin]
    return midi_array


def write_tune_rs_file(filename, track1, track2, drums):
    txt = f"""// This file has been automatically generated
    
pub const TUNE_STEP_COUNT: u16 = {STEP_COUNT};
pub const TUNE_TRACK1: [(u8, u8); TUNE_STEP_COUNT as usize] = {track1};
pub const TUNE_TRACK2: [(u8, u8); TUNE_STEP_COUNT as usize] = {track2};
pub const TUNE_DRUMS: [(u8, u8); TUNE_STEP_COUNT as usize] = {drums};
"""

    print(txt)
    with open(filename, "w") as fio:
        fio.write(txt)


def main_tune():
    track1 = parse_file('../src/assets/tune/tune_lead.mid')
    track2 = parse_file('../src/assets/tune/tune_bass.mid')
    drums = parse_file('../src/assets/tune/tune_drum.mid')

    write_tune_rs_file('../src/tune.rs', track1, track2, drums)

    utils.format_rust_file('../src/tune.rs')


def main_sfx():
    dst_filename = "../src/sfx.rs"
    txt = f"// This file has been automatically generated\n\n"

    midi_files = utils.find_all_by_extension('../src/assets/sfx_225bpm', ".mid")
    for filename in midi_files:
        name = os.path.split(filename)[-1]
        assert name.endswith(".mid")
        name = name[:-4].upper().replace(" ", "_").replace("-", "_")

        midi_array = parse_file(filename, bpm_gain=2)
        # midi_array = trim_00(midi_array, margin=16)
        midi_array = midi_array[:30]  # force length to 30 because gba_synth doesn't support multiple length yet
        txt += f"\n"
        txt += f"pub const {name}_STEP_COUNT: u16 = {len(midi_array)};\n"
        txt += f"pub const {name}: [(u8, u8); {name}_STEP_COUNT as usize] = {midi_array};\n"

    print(txt)
    with open(dst_filename, "w") as fio:
        fio.write(txt)

    utils.format_rust_file(dst_filename)

if __name__ == '__main__':
    main_tune()
    main_sfx()
