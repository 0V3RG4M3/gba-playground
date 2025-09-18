import socket
import time
import mmio
from reg_tune_logger import IRegTuneLogWriter, RegTuneLogWriter
from typing import Optional
from simple_stream import ISimpleStream, UDPSimpleStream, NullSimpleStream, TCPSimpleStream, FileSimpleStream
import max4live_udp_cleaner

class BridgeInTheMiddle:

    def __init__(self, input_stream: ISimpleStream, output_stream: ISimpleStream,
                 reg_tune_logger: Optional[IRegTuneLogWriter] = None):
        self.input_stream: ISimpleStream = input_stream
        self.output_stream: ISimpleStream = output_stream
        self.reg_tune_logger: Optional[IRegTuneLogWriter] = reg_tune_logger

    def run(self):
        print("Create a Input stream")
        with self.input_stream as input_stream:
            while True:

                print("Create Output stream")
                with self.output_stream as output_stream:

                    while True:
                        # Receive data from UDP socket
                        data = input_stream.read()
                        print(data)

                        if self.reg_tune_logger:
                            self.reg_tune_logger.log(data)

                        data = max4live_udp_cleaner.clean_udp_message(data)
                        output_stream.push(data)


def main():
    if False:
        BridgeInTheMiddle(
            input_stream=UDPSimpleStream("127.0.0.1", 9999),
            output_stream=NullSimpleStream(),
            reg_tune_logger=RegTuneLogWriter("reg_tune.bin.txt")
        ).run()
    else:
        BridgeInTheMiddle(
            input_stream=FileSimpleStream("reg_tune.csv.bin.txt"),
            output_stream=TCPSimpleStream("localhost", 8888),
        ).run()

    return
    time.sleep(0.1)
    generate_reg_tune_rs.main_tune()  # TODO: update generator to new files


if __name__ == "__main__":
    main()
