import asyncio
from typing import Optional


class ISimpleStreamAsync:
    async def push(self, data: bytes) -> None:
        raise NotImplementedError()

    async def read(self) -> bytes:
        raise NotImplementedError()

    def __aenter__(self):
        raise NotImplementedError()

    def __aexit__(self, exc_type, exc_value, traceback):
        raise NotImplementedError()


# TODO: test this class
class TCPSimpleStreamAsync(ISimpleStreamAsync):
    def __init__(self, host: str, port: int):
        self.host = host
        self.port = port
        self.reader = None
        self.writer = None
        self._is_opened = False

    def __enter__(self):
        raise NotImplementedError("Use 'async with' statement to open/close the stream.")

    async def __aenter__(self):
        self.reader, self.writer = await asyncio.open_connection(self.host, self.port)
        self._is_opened = True
        return self

    async def __aexit__(self, exc_type, exc_value, traceback):
        if self.writer:
            self.writer.close()
            await self.writer.wait_closed()
        self._is_opened = False

    async def push(self, data: bytes) -> None:
        assert self._is_opened, "Stream is not opened. Use 'async with' statement to open the stream."
        if self.writer:
            self.writer.write(data)
            await self.writer.drain()

    async def read(self) -> bytes:
        assert self._is_opened, "Stream is not opened. Use 'async with' statement to open the stream."
        if self.reader:
            data = await self.reader.read(1024)
            return data
        return b''

class UDPProtocol(asyncio.DatagramProtocol):
    def __init__(self, queue: asyncio.Queue):
        self.queue = queue

    def datagram_received(self, data: bytes, addr):
        self.queue.put_nowait((data, addr))

    def error_received(self, exc):
        print(f"Error received: {exc}")

# TODO: test this class
class UDPSimpleStreamAsync(ISimpleStreamAsync):
    def __init__(self, host: str, port: int):
        self.host = host
        self.port = port


        self._transport: Optional[asyncio.DatagramTransport] = None
        self._is_opened = False
        self._protocol: UDPProtocol = UDPProtocol(queue=asyncio.Queue())


    def __enter__(self):
        raise NotImplementedError("Use 'async with' statement to open/close the stream.")

    async def __aenter__(self):
        loop = asyncio.get_running_loop()
        self._transport, protocol = await loop.create_datagram_endpoint(
            lambda: self._protocol,
            local_addr=(self.host, self.port)
        )
        self._is_opened = True
        return self

    async def __aexit__(self, exc_type, exc_value, traceback):
        if self._transport:
            self._transport.close()
        self._is_opened = False

    async def push(self, data: bytes) -> None:
        assert self._is_opened, "Stream is not opened. Use 'async with' statement to open the stream."
        if self._transport:
            self._transport.sendto(data)

    async def read(self) -> bytes:
        assert self._is_opened, "Stream is not opened. Use 'async with' statement to open the stream."
        data, addr = await self._protocol.queue.get()
        return data

