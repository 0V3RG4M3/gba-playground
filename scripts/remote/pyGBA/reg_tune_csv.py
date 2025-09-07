import time
import mmio

HEADER = "time,cmd,address,value"

class RegTuneCsvWriter:
    def __init__(self, filename: str):
        self.filename = filename
        self.t0: float = 0

    def reset_timer(self):
        self.t0 = time.time()

    def newlogfile(self):
        with open(self.filename, "w") as file:
            file.write(HEADER + "\n")

    def log(self, command_line, verbose=False):
        cmd, address, value = command_line.split(" ")
        ts = str(time.time() - self.t0)
        line = ",".join([ts, cmd, address, value])

        if verbose:
            info = mmio.addr2reg_map(address)
            info = info.NAME if info is not None else ""
            print(f"{line} ({info})")

        with open(self.filename, "a") as file:
            file.write(line + "\n")

class RegTuneCsvReader:
    def __init__(self, filename: str):
        raise NotImplementedError("yet to be implemented")


