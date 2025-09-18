import socket
import time
import mmio
from reg_tune_logger import IRegTuneLogWriter, RegTuneLogWriter
from typing import Optional
from simple_stream import ISimpleStream, UDPSimpleStream, NullSimpleStream, TCPSimpleStream

class BridgeInTheMiddle:
    FILENAME = "reg_tune.csv"
    HEADER = "time,cmd,address,value"

    def __init__(self, input_stream: ISimpleStream, output_stream: ISimpleStream, reg_tune_logger:Optional[IRegTuneLogWriter] = None):
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

                        output_stream.push(data)



def main():
    BridgeInTheMiddle(
        input_stream=UDPSimpleStream("127.0.0.1", 9999),
        output_stream=NullSimpleStream(),
        reg_tune_logger=RegTuneLogWriter("reg_tune4.hex")
    ).run()
    return
    time.sleep(0.1)
    generate_reg_tune_rs.main_tune()

if __name__ == "__main__":
    main()
