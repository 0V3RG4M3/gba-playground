import socket
import time
import mmio
import generate_reg_tune_rs


class BridgeInTheMiddle:
    FILENAME = "reg_tune.csv"
    HEADER = "time,cmd,address,value"

    def __init__(self, udp_port, tcp_host, tcp_port):
        self.udp_port = udp_port
        self.tcp_host = tcp_host
        self.tcp_port = tcp_port
        self.t0: float = 0

        self.is_recording = False

    def newlogfile(self):
        with open(self.FILENAME, "w") as file:
            file.write(self.HEADER + "\n")

    def log(self, command_line):
        cmd, address, value = command_line.split(" ")
        ts = str(time.time() - self.t0)
        line = ",".join([ts, cmd, address, value])
        info = mmio.addr2reg_map(address)
        info = info.NAME if info is not None else ""
        print(f"{line} ({info})")
        if self.is_recording:
            with open(self.FILENAME, "a") as file:
                file.write(line + "\n")

    def run(self):
        # Create a UDP socket
        with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as udp_socket:
            udp_socket.bind(('0.0.0.0', self.udp_port))

            while True:
                try:
                    print("Create a TCP socket")
                    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as tcp_socket:
                        tcp_socket.connect((self.tcp_host, self.tcp_port))

                        while True:
                            # Receive data from UDP socket
                            data, addr = udp_socket.recvfrom(128)
                            data = data.rstrip(b'\x00').rstrip(b',').rstrip(b'\x00')
                            command = data.decode("utf-8")
                            if command.startswith("WRITE"):
                                # Send data to TCP socket
                                tcp_socket.sendall(data)
                                self.log(command)
                            elif command.startswith("REC"):
                                self.newlogfile()
                                self.t0 = time.time()
                                self.is_recording = True
                            elif command.startswith("STOP"):
                                self.log("STOP -1 -1")
                                self.is_recording = False
                                print("STOP")
                                return

                except socket.error as e:
                    print(f"Socket error: {e}")
                    print("Reconnecting TCP socket...")
                    time.sleep(1)
                    continue


def main():
    BridgeInTheMiddle(9999, 'localhost', 8888).run()
    time.sleep(0.1)
    generate_reg_tune_rs.main_tune()

if __name__ == "__main__":
    main()
