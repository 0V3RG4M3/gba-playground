import max4live_udp_cleaner
from typing import Optional
import time


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
        items = max4live_udp_cleaner.clean_udp_message(data).decode("utf-8").splitlines()

        if len(items) == 0:
            return

        if items[0] == "REC":  # empty line
            self.newlogfile()
            self.is_recording = True
            return

        if items[0] == "STOP":
            self.is_recording = False
            print("STOP")
            return

        if self.is_recording:
            line = repr(data.decode("latin1"))  # latin1 = preserves bytes 0-255

            with open(self.filename, "a", encoding="utf-8") as f:
                f.write(line + "\n")


class RegTuneLogReader:
    def __init__(self, filename: str, time_scale: Optional[float] = None):
        self.filename = filename
        assert time_scale is None or time_scale > 0.0, "time_scale must be positive"
        self.time_scale: Optional[float] = time_scale

    def read(self):
        lines: list[bytes] = []

        with open(self.filename, encoding="utf-8") as file:
            t0 = time.time()
            for line in file:
                line = line.strip()
                data = eval(line).encode()
                lines.append(data)

        for data in lines:
            items = data.decode("utf-8").splitlines()

            if len(items) == 0:
                continue

            if self.time_scale is None:
                yield data
                continue

            frame_id = int(items[0])
            t1 = t0 + (frame_id * self.time_scale / 60.0)
            to_sleep = max(0.0, t1 - time.time())
            time.sleep(to_sleep)
            yield data
