import socket
import time


class BridgeInTheMiddle:
    FILENAME = "chiptune.csv"
    HEADER = "time,size,address,value"

    def __init__(self, udp_port, tcp_host, tcp_port):
        self.udp_port = udp_port
        self.tcp_host = tcp_host
        self.tcp_port = tcp_port
        self.t0: float = 0

    def newfile(self):
        with open(self.FILENAME, "w") as file:
            file.write(self.HEADER + "\n")

    def log_to_file(self, cmd):
        func, address, value = cmd.split(" ")
        ts = str(time.time() - self.t0)
        line = ",".join([ts, func, address, value])
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

                        is_recording = False
                        while True:
                            # Receive data from UDP socket
                            data, addr = udp_socket.recvfrom(1024)
                            data = data.rstrip(b'\x00').rstrip(b',').rstrip(b'\x00')
                            print(f"Received UDP data from {addr}: {data}")
                            command = data.decode("utf-8")
                            if command.startswith("WRITE"):
                                # Send data to TCP socket
                                tcp_socket.sendall(data)
                                if is_recording:
                                    self.log_to_file(command)
                            elif command.startswith("REC"):
                                self.newfile()
                                is_recording = True
                            elif command.startswith("STOP"):
                                is_recording = False

                    break
                except socket.error as e:
                    print(f"Socket error: {e}")
                    print("Reconnecting TCP socket...")
                    time.sleep(1)
                    continue


def main():
    BridgeInTheMiddle(9999, 'localhost', 8888).run()


if __name__ == "__main__":
    main()
