"""
Provides a human-readable visualization of GBA sound register states,
used for debugging and monitoring register changes.
"""

import enum

import gba_mmio
import gba_sound


class Color(enum.StrEnum):
    BLACK = "\033[0m"
    GRAY = "\033[90m"
    RED = "\033[91m"


def color_diff_values(old: str, new: str, sep:str, base_color: Color, diff_color: Color) -> str:
    new_spl = new.split(sep)[1:-1]
    old_spl = old.split(sep)[1:-1]
    res = sep
    for new_val, old_val in zip(new_spl, old_spl):
        current_color = base_color if new_val == old_val else diff_color
        res += current_color + new_val + base_color
        res += sep
    return res

class RegistryState:
    def __init__(self):
        self.registries: dict[int, int] = {
            gba_mmio.TONE1_SWEEP.ADDRESS: 0,
            gba_mmio.TONE1_PATTERN.ADDRESS: 0,
            gba_mmio.TONE1_FREQUENCY.ADDRESS: 0,
            gba_mmio.TONE2_PATTERN.ADDRESS: 0,
            gba_mmio.TONE2_FREQUENCY.ADDRESS: 0
        }
        self.registries_old = self.registries.copy()
        self.updated_registries: set[int] = set(self.registries.keys())

    def set_value(self, address: int, value: int):
        if address not in self.registries:
            return

        self.registries[address] = value
        self.updated_registries.add(address)


    def to_string(self):
        reg_sep = "   "
        headers_str = reg_sep
        reg_data_str = reg_sep

        for addr, val in self.registries.items():
            reg = gba_mmio.addr2reg_map(hex(addr))

            if reg is not None and addr in self.registries:
                reg_data_old = gba_sound.create_reg_data_from_value(reg.DATA_TYPE, self.registries_old[addr])
                reg_data = gba_sound.create_reg_data_from_value(reg.DATA_TYPE, val)

                base_color = Color.GRAY if addr not in self.updated_registries else Color.BLACK
                _, old_values_str = reg_data_old.to_string()
                header_str, values_str = reg_data.to_string()

                headers_str += f"{header_str}" + reg_sep

                reg_data_str += color_diff_values(old_values_str, values_str, "|", base_color, Color.RED)
                reg_data_str += reg_sep
                # values_str += color + v_str + Color.BLACK + sep

        reg_data_str += Color.BLACK

        self.updated_registries.clear()
        self.registries_old = self.registries.copy()
        return headers_str, reg_data_str


def main():
    regstate = RegistryState()

    commands = b'5325 WRITE16 0x400006c 0x563\n5325 WRITE16 0x4000068 0xf000\n5325 WRITE16 0x4000064 0x721\n5325 WRITE16 0x4000062 0x20c\n'.splitlines()
    frame_id = ""
    for cmd in commands:
        frame_id, cmd_type, addr_str, value_str = cmd.decode().split()
        regstate.set_value(int(addr_str[2:], 16), int(value_str[2:], 16))

    print(frame_id, regstate.to_string()[1])


if __name__ == "__main__":
    main()
