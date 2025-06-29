import socket
import time
from remote.pyGBA import mmio, sound

# convert midi pitch in[0, 127] to rates used in gba tone1 and tone2. lowest available pitch is 36
PITCH2RATE = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 45, 156, 262, 362, 458, 547, 632, 712, 786, 856, 923, 986, 1046, 1102, 1155, 1205,
    1253, 1297, 1340, 1380, 1417, 1452, 1486, 1517, 1547, 1575, 1602, 1627, 1650, 1673, 1694, 1714,
    1732, 1750, 1767, 1783, 1798, 1812, 1825, 1837, 1849, 1860, 1871, 1881, 1890, 1899, 1907, 1915,
    1923, 1930, 1936, 1943, 1949, 1954, 1959, 1964, 1969, 1974, 1978, 1982, 1985, 1989, 1992, 1995,
    1998, 2001, 2004, 2006, 2009, 2011, 2013, 2015, 2017, 2018, 2020, 2022, 2023, 2025, 2026, 2027,
    2028, 2029, 2030, 2031, 2032, 2033, 2034, 2035, 2036, 2036, 2037, 2038,
]


def frequency_to_rate(frequency: int) -> int:
    """
    frequency = 131072 / (2048 - rate)
    rate = 2048 - (131072 / frequency)
    """
    if frequency <= 0:
        return 0
    rate = 2048 - (131072 / frequency)
    rate = int(round(rate))
    return max(0, min(2047, rate))


def log_cmd(frame_id: int, commands: list):
    for cmd in commands:
        print(frame_id, cmd)


def send_cmd(s: socket.socket, commands: list):
    msg = "\n".join(commands).encode('utf-8')
    s.sendall(msg + b'\n')


def initilize_synth(s: socket.socket):
    master_sound_on = mmio.SOUND_ENABLED.write_cmd(sound.SoundEnable(enabled=True))
    all_tone_on = mmio.LEFT_RIGHT_VOLUME.write_cmd(sound.LeftRightVolume(
        right_volume=7, left_volume=7,
        tone1_right=True, tone2_right=True, wave_right=True, noise_right=True,
        tone1_left=True, tone2_left=True, wave_left=True, noise_left=True
    ))

    commands = [master_sound_on, all_tone_on]
    log_cmd(0, commands)
    send_cmd(s, commands)


def main():
    with socket.socket() as s:
        s.connect(('localhost', 8888))
        print("Connected to synth server on port 8888")
        initilize_synth(s)

        no_sweep = sound.SweepControl()
        glockenspiel_tone_pattern = sound.TonePattern(volume=15, length=0, duty=2, step_time=7, step_increasing=False)
        tone_frequency = sound.ToneFrequency(frequency_rate=frequency_to_rate(440), stop_when_expired=False, enabled=True)

        for frame_id in range(60):
            commands = [
                mmio.TONE1_SWEEP.write_cmd(no_sweep),
                mmio.TONE1_PATTERN.write_cmd(glockenspiel_tone_pattern),
                mmio.TONE1_FREQUENCY.write_cmd(sound.ToneFrequency(frequency_rate=frequency_to_rate(440 + frame_id), stop_when_expired=False, enabled=True)),
            ]
            log_cmd(frame_id, commands)
            send_cmd(s, commands)

            time.sleep(1/60)


if __name__ == "__main__":
    main()
