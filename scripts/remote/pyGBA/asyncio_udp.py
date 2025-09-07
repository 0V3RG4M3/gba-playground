import asyncio
from typing import Optional, Tuple

class UDPConnection(asyncio.DatagramProtocol):
    def __init__(self):
        self.queue = asyncio.Queue()
        self.transport: Optional[asyncio.DatagramTransport] = None

    def connection_made(self, transport: asyncio.DatagramTransport):
        self.transport = transport
        print("🔌 UDP server ready and listening...")

    def datagram_received(self, data: bytes, addr: Tuple[str, int]):
        print(f"📩 reçu {data.decode()} de {addr}")
        self.queue.put_nowait((data, addr))

    async def recv(self) -> Tuple[bytes, Tuple[str, int]]:
        data, addr = await self.queue.get()
        return data, addr

    def send(self, data: bytes, addr: Tuple[str, int]):
        if self.transport:
            print(f"📤 envoi '{data.decode()}' vers {addr}")
            self.transport.sendto(data, addr)

    def close(self):
        if self.transport:
            self.transport.close()
            self.transport = None


async def demo():
    """ """
    loop = asyncio.get_running_loop()
    conn = UDPConnection()

    transport, _ = await loop.create_datagram_endpoint(
        lambda: conn,
        local_addr=("127.0.0.1", 9999),
    )

    try:
        print("UDP server listening on port 9999...")
        while True:
            data, addr = await conn.recv()
            # Automatically sending an acknowledgment back to the sender
            conn.send(f"ACK: {data.decode()}".encode(), addr)
    except asyncio.CancelledError:
        pass
    finally:
        conn.close()


if __name__ == "__main__":
    asyncio.run(demo())
