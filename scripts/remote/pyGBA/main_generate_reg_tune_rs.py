"""
Converts recorded register tunes from Max4Live into Rust code for embedding into GBA ROMs,
handling timing and register data extraction.
"""

import max4live_udp_cleaner
import utils
from reg_tune_logger import RegTuneLogReader
from pathlib import Path


def extract_data(reg_tune_file: str):
    result: list[tuple[int, int, int, int]] = []
    loop_size: int = 0
    frame_id0 = -1

    tune_reader = RegTuneLogReader(reg_tune_file, time_scale=None)


    for data in tune_reader.read():
        print(data)

        if len(data) < 2:
            continue

        items = max4live_udp_cleaner.clean_udp_message(data).decode("utf-8").split()
        frame_id, command = int(items[0]), items[1:]

        if "REC" in command[0]:
            frame_id0 = frame_id
            continue

        if "STOP" in command[0]:
            loop_size = frame_id - frame_id0
            break

        assert len(command) == 3, f"Unexpected command length: {command}"

        cmd, addr, value = command

        frame_id_k = frame_id - frame_id0
        size = int(cmd[len('WRITE'):]) // 8
        addr = int(addr, 16)
        value = int(value, 16)
        result.append((frame_id_k, size, addr, value))

    return result, loop_size


def parse_file(reg_tune_file: Path, bpm_gain: float = 1) -> tuple[list[tuple[int, int, int, int]], int]:
    """
    Parse a text file containing register writes and return a list of tuples (frame_id, size, address, value) as well as the loop size.

    Expected file format:
    b"'4258 REC\x00\x00\x00\x00,\x00\x00\x00'"
    b"'4268 WRITE16 0x4000062 0xf143\x00\x00\x00,\x00\x00\x00'"
    b"'4268 WRITE16 0x4000064 0x8689\x00\x00\x00,\x00\x00\x00'"
    b"'4274 WRITE16 0x4000062 0x143\x00\x00\x00\x00,\x00\x00\x00'"
    b"'4274 WRITE16 0x4000064 0x8689\x00\x00\x00,\x00\x00\x00'"
    b"'4282 WRITE16 0x4000062 0xf143\x00\x00\x00,\x00\x00\x00'"
    b"'4282 WRITE16 0x4000064 0x8689\x00\x00\x00,\x00\x00\x00'"
    b"'4289 WRITE16 0x4000062 0xf143\x00\x00\x00,\x00\x00\x00'"
    b"'4289 WRITE16 0x4000064 0x86b2\x00\x00\x00,\x00\x00\x00'"
    b"'4296 WRITE16 0x4000062 0x143\x00\x00\x00\x00,\x00\x00\x00'"
    ...
    b"'5698 STOP\x00\x00\x00,\x00\x00\x00'"

    """
    result: list[tuple[int, int, int, int]]
    loop_size: int  # important to know when the loop must start over

    result, loop_size = extract_data(reg_tune_file)

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

    here = Path(__file__).parent
    reg_tune_src_folder = here / "reg_tunes"
    src_ext = ".bin.txt"

    reg_tune_dst_folder = here / '../../../'
    dst_ext = ".rs"

    reg_tune_file_subpath =  Path("src/egj2025/reg_tune")

    src_file = (reg_tune_src_folder / reg_tune_file_subpath).with_suffix(src_ext)
    tune, frame_count = parse_file(src_file, bpm_gain=1)

    dst_rsfile = (reg_tune_dst_folder / reg_tune_file_subpath).with_suffix(dst_ext)
    write_reg_tune_rs_file(dst_rsfile, tune, frame_count)
    utils.format_rust_file(dst_rsfile)


if __name__ == '__main__':
    main_tune()
