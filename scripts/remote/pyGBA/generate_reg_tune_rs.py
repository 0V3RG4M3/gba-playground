import numpy as np
import os
import utils

def parse_file(csv_path: str, bpm_gain: float = 1) -> tuple[list[tuple[int, int, int, int]], int]:
    """Parse a CSV file containing MIDI data and convert it to a list of tuples representing the track. """    
    result = []
    loop_size: int = 0  # important to know when the loop must start over
    with open(csv_path, 'r') as csvfile:

        header = csvfile.readline()
        assert header =='time,cmd,address,value\n', "CSV file must start with 'time,cmd,addr,value' header"
        fps = 60
        for line in csvfile:
            if len(line) < 2:
                continue

            time, cmd, addr, value = line.strip().split(',')
            frame = int(round(float(time) * fps / bpm_gain))

            if cmd == "STOP":
                loop_size = frame
                break

            size = int(cmd[len('WRITE'):]) // 8
            addr = int(addr, 16)
            value = int(value, 16)
            result.append((frame, size, addr, value))

    # Sort by frame (just in case)
    result.sort(key=lambda x: x[0])

    if result[-1][0] >= loop_size:
        loop_size = result[-1][0] + 1

    return result, loop_size


def write_reg_tune_rs_file(filename, regs, frame_count):
    txt = f"""// This file has been automatically generated
    

pub const TUNE_LOOP_SIZE: u16 = {frame_count};
pub const TUNE_SIZE: u16 = {len(regs)};
#[link_section =".rodata"]
pub static TUNE_TRACK1: [(u16, u8, u32, u32); TUNE_SIZE as usize] = {regs};
"""
    print(txt)
    with open(filename, "w") as fio:
        fio.write(txt)


def main_tune():
    # tune, frame_count = parse_file('../src/assets/reg_tunes/reg_tune1.csv')
    tune, frame_count = parse_file('reg_tune.csv', bpm_gain=1)
    write_reg_tune_rs_file('../../../src/reg_tune.rs', tune, frame_count)

    utils.format_rust_file('../../../src/reg_tune.rs')


if __name__ == '__main__':
    main_tune()
