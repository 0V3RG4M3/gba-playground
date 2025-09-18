import max4live_udp_cleaner

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
            with open(self.filename, "ab") as file:
                file.writelines([data])

class RegTuneLogReader:
    def __init__(self, filename: str):
        raise NotImplementedError("yet to be implemented")