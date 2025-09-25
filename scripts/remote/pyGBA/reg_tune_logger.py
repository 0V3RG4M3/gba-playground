import asyncio
import time
from typing import Optional

import max4live_udp_cleaner


def extract_frame_id(log_line: bytes) -> Optional[int]:
    items = max4live_udp_cleaner.clean_udp_message(log_line).decode("utf-8").strip().split()
    if len(items) == 0:
        return None
    return int(items[0])


class IRegTuneLogWriter:
    def newlogfile(self):
        raise NotImplementedError("newlogfile method not implemented")

    def log(self, data: bytes) -> None:
        raise NotImplementedError("log method not implemented")


class RegTuneLogWriter(IRegTuneLogWriter):
    def __init__(self, filename: str):
        self.filename = filename
        self.is_recording = False

    def newlogfile(self):
        with open(self.filename, "wb") as file:
            pass

    def log(self, data: bytes) -> None:
        items = max4live_udp_cleaner.clean_udp_message(data).decode("utf-8").strip().split()

        if len(items) == 0:
            return

        if items[1] == "REC":  # empty line
            self.newlogfile()
            self.is_recording = True

        if self.is_recording:
            line = repr(data.decode("latin1"))  # latin1 = preserves bytes 0-255

            with open(self.filename, "a", encoding="utf-8") as f:
                f.write(line + "\n")

        if items[1] == "STOP":
            self.is_recording = False
            print("STOP")
            return


class RegTuneLogReader:
    def __init__(self, filename: str, time_scale: Optional[float] = None):
        self.filename = filename
        assert time_scale is None or time_scale > 0.0, "time_scale must be positive"
        self.time_scale: Optional[float] = time_scale

    def read(self):
        lines: list[bytes] = []

        with open(self.filename, encoding="utf-8") as file:
            for line in file:
                line = line.strip()
                data = eval(line).encode()
                lines.append(data)

        t0 = time.time()
        for data in lines:
            frame_id = extract_frame_id(data)
            if frame_id is None:
                continue

            if self.time_scale is None:
                yield data
                continue

            t1 = t0 + (frame_id * self.time_scale / 60.0)
            to_sleep = max(0.0, t1 - time.time())
            time.sleep(to_sleep)
            yield data


class RegTuneLogReaderAsync:
    def __init__(self, filename: str, loop: bool = False, time_scale: Optional[float] = None):
        self.filename = filename
        assert time_scale is None or time_scale > 0.0, "time_scale must be positive"
        self.loop = loop
        self.time_scale: Optional[float] = time_scale

    async def read(self):
        lines: list[bytes] = []

        with open(self.filename, encoding="utf-8") as file:
            for line in file:
                line = line.strip()
                data = eval(line).encode()
                lines.append(data)

        if len(lines) == 0:
            return

        frame_id0 = extract_frame_id(lines[0])
        if frame_id0 is None:
            raise RuntimeError

        while True:
            t0 = time.time()
            for data in lines:
                frame_id = extract_frame_id(data)
                if frame_id is None:
                    continue

                if self.time_scale is None:
                    yield data
                    continue

                delta_frame_id = frame_id - frame_id0
                delta_time = delta_frame_id * self.time_scale / 60.0
                t1 = t0 + delta_time
                to_sleep = max(0.0, t1 - time.time())
                if to_sleep > 0.0:
                    await asyncio.sleep(to_sleep)

                yield data
            print("............................................................................................................")
            if not self.loop:
                break
