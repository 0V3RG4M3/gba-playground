import socket
import time


class ISimpleStream:
    def push(self, data: bytes) -> None:
        raise NotImplementedError()

    def read(self) -> bytes:
        raise NotImplementedError()

    def __enter__(self):
        raise NotImplementedError()

    def __exit__(self, exc_type, exc_value, traceback):
        raise NotImplementedError()


class UDPSimpleStream(ISimpleStream):
    def __init__(self, host, port):
        self.host = host
        self.port = port
        self._udp_socket: socket.socket
        self._is_opened = False

    def __enter__(self):
        self._is_opened = True
        self._udp_socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        try:
            self._udp_socket.bind((self.host, self.port))
        except Exception as e:
            print(f"Failed to bind UDP socket on {self.host}:{self.port}: {e}")
            self._udp_socket.close()
            raise
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        self._udp_socket.close()
        self._is_opened = False

    def push(self, data: bytes) -> None:
        # TODO: test this function
        assert self._is_opened, "Stream is not opened. Use 'with' statement to open the stream."
        self._udp_socket.sendto(data, (self.host, self.port))

    def read(self) -> bytes:
        assert self._is_opened, "Stream is not opened. Use 'with' statement to open the stream."
        data, addr = self._udp_socket.recvfrom(1024)
        return data


class TCPSimpleStream(ISimpleStream):
    def __init__(self, host, port):
        self.host = host
        self.port = port
        self._tcp_socket: socket.socket
        self._is_opened = False

    def __enter__(self):
        self._is_opened = True
        self._tcp_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        try:
            self._tcp_socket.connect((self.host, self.port))
        except Exception as e:
            print(f"Failed to connect TCP socket to {self.host}:{self.port}: {e}")
            self._tcp_socket.close()
            raise
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        self._tcp_socket.close()
        self._is_opened = False

    def push(self, data: bytes) -> None:
        assert self._is_opened, "Stream is not opened. Use 'with' statement to open the stream."
        self._tcp_socket.sendall(data)

    def read(self) -> bytes:
        # TODO: test this function
        assert self._is_opened, "Stream is not opened. Use 'with' statement to open the stream."
        data = self._tcp_socket.recv(1024)
        return data


# TODO: test this class
class NullSimpleStream(ISimpleStream):
    def __init__(self, rate_hz: float = 10.0):
        self.rate_hz = rate_hz
        self._is_opened = False

    def __enter__(self):
        self._is_opened = True
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        self._is_opened = False

    def push(self, data: bytes) -> None:
        assert self._is_opened, "Stream is not opened. Use 'with' statement to open the stream."

    def read(self) -> bytes:
        assert self._is_opened, "Stream is not opened. Use 'with' statement to open the stream."
        time.sleep(1.0 / self.rate_hz)
        return b''
