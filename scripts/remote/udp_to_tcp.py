import socket
import time

FILENAME="chiptune.txt"

def newfile():
    with open(FILENAME, "w") as file:
        pass

def log_to_file(cmd):
    with open(FILENAME, "a") as file:
        file.write(cmd + "\n")

def udp_to_tcp(udp_port, tcp_host, tcp_port):
    # Create a UDP socket
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as udp_socket:
        udp_socket.bind(('0.0.0.0', udp_port))

        while True:
            try:
                print("Create a TCP socket")
                with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as tcp_socket:
                    tcp_socket.connect((tcp_host, tcp_port))

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
                                log_to_file(command)
                        elif command.startswith("REC"):
                            newfile()
                            is_recording = True
                        elif command.startswith("STOP"):
                            is_recording = False

                break
            except socket.error as e:
                print(f"Socket error: {e}")
                print("Reconnecting TCP socket...")
                time.sleep(1)
                continue


if __name__ == "__main__":
    udp_to_tcp(9999, 'localhost', 8888)