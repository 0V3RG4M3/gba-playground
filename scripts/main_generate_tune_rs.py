import numpy as np
import os
from miditoolkit.midi import parser as mid_parser
from miditoolkit.midi import containers as ct
import subprocess

BAR_COUNT = 8
BEAT_PER_BAR = 4
STEPS_PER_BEAT = 32

STEP_COUNT = STEPS_PER_BEAT * BEAT_PER_BAR * BAR_COUNT


def parse_file(midi_path):
    # load midi file
    mido_obj = mid_parser.MidiFile(midi_path)
    tick2frame = STEPS_PER_BEAT / mido_obj.ticks_per_beat

    assert len(mido_obj.instruments) == 1
    instrument = mido_obj.instruments[0]

    # fill track
    track = [(0, 0)] * STEP_COUNT
    for note in instrument.notes:
        frame_id = int(round(note.start * tick2frame))
        track[frame_id] = (note.pitch, note.velocity)

    return track


def write_tune_rs_file(filename, track1, track2, drums):
    txt = f"""// This file has been automatically generated
    
pub const TUNE_STEP_COUNT: u16 = {STEP_COUNT};
pub const TUNE_TRACK1: [(u16, u16); TUNE_STEP_COUNT as usize] = {track1};
pub const TUNE_TRACK2: [(u16, u16); TUNE_STEP_COUNT as usize] = {track2};
pub const TUNE_DRUMS: [(u16, u16); TUNE_STEP_COUNT as usize] = {drums};
"""

    print(txt)
    with open(filename, "w") as fio:
        fio.write(txt)


def format_rust_file(filename):
    subprocess.run(['rustfmt', filename])


def main():
    track1 = parse_file('../src/assets/tune/tune_lead.mid')
    track2 = parse_file('../src/assets/tune/tune_bass.mid')
    drums = parse_file('../src/assets/tune/tune_bass.mid')

    write_tune_rs_file('../src/tune.rs', track1, track2, drums)

    format_rust_file('../src/tune.rs')


if __name__ == '__main__':
    main()
