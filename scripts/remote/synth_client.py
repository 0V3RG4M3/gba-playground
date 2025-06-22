import socket
import time

def send_frame(s: socket.socket, commands: list):
    msg = "\n".join(commands).encode('utf-8')
    print("send")
    s.sendall(msg + b'\n')


def main():
    with socket.socket() as s:
        s.connect(('localhost', 8888))
        print("Connected to synth server on port 8888")

        for frame in range(3):
            send_frame(s, ['WRITE8 0x04000001 0x80'])
            time.sleep(1/60)

if __name__ == "__main__":
    main()

