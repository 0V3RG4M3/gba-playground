import numpy as np
import os
import utils

def parse_file(csv_path: str, bpm_gain: float = 1) -> list[tuple[int, int, int, int, int]]:
    """Parse a CSV file containing MIDI data and convert it to a list of tuples representing the track. """    
    result = []
    with open(csv_path, 'r') as csvfile:

        header = csvfile.readline()
        assert header =='time,cmd,address,value\n', "CSV file must start with 'time,cmd,addr,value' header"
        fps = 60
        for line in csvfile:
            if len(line) < 2:
                continue

            time, cmd, addr, value = line.strip().split(',')
            frame = int(round(float(time) * fps / bpm_gain))
            size = int(cmd[len('WRITE'):]) // 8
            addr = int(addr, 16)
            value = int(value, 16)
            result.append((frame, size, addr, value))

    # Sort by frame
    result.sort(key=lambda x: x[0])

    return result


def write_reg_tune_rs_file(filename, regs):
    txt = f"""// This file has been automatically generated
    

pub const TUNE_STEP_COUNT: u16 = {len(regs)};
pub const TUNE_TRACK1: [(u16, u8, u32, u32); TUNE_STEP_COUNT as usize] = {regs};
"""
    print(txt)
    with open(filename, "w") as fio:
        fio.write(txt)


def main_tune():
    tune = parse_file('../src/assets/reg_tunes/reg_tune1.csv')
    write_reg_tune_rs_file('../src/reg_tune.rs', tune)

    utils.format_rust_file('../src/reg_tune.rs')


if __name__ == '__main__':
    main_tune()
