import asyncio
import time
from typing import Optional

# Local imports
from reg_tune_csv import RegTuneCsvWriter
from reg_tune_logger import IRegTuneLogWriter, RegTuneLogWriter
from simple_stream_async import ISimpleStreamAsync, UDPSimpleStreamAsync, FileSimpleStreamAsync
import max4live_udp_cleaner
MAX_BUFFER_SIZE = 1024


async def cleanup_socket(reader: Optional[asyncio.StreamReader], writer: Optional[asyncio.StreamWriter]):
    if reader:
        try:
            reader.feed_eof()
        except Exception as e:
            print(f"Error during reader cleanup: {e}")

    if writer:
        try:
            writer.close()
            await writer.wait_closed()
        except Exception as e:
            print(f"Error during writer cleanup: {e}")


async def triggered_consumer_loop_async(queue: asyncio.Queue[str], stop_event: asyncio.Event, reader: asyncio.StreamReader,
                                        writer: asyncio.StreamWriter):
    """Consomme par batch à 60 fps"""
    while reader and writer and not stop_event.is_set():
        # Wait for trigger (e.g., a line with 'TRIGGER\n')
        trigger = await reader.readline()
        if not trigger:
            print("CONSUMER: 🔌❌ Connection closed by peer.")
            return

        batch = []
        while not queue.empty():
            batch += queue.get_nowait()

        if batch:
            commands = ("\n".join(batch) + "\n").encode()
            if len(commands) > 1024:
                print(f"CONSUMER: ⚠️ Warning: Sending a large batch of {len(commands)} bytes")

            writer.write(commands)
            await writer.drain()

            print(f"CONSUMER: 📤 Sent {len(commands)} bytes after trigger")


async def start_socket_consumer_async(queue: asyncio.Queue[str], stop_event: asyncio.Event, host: str, port: int):
    reader: Optional[asyncio.StreamReader] = None
    writer: Optional[asyncio.StreamWriter] = None

    print(f"CONSUMER: 🔌⏳ Connecting with TCP on {host}:{port}...")
    while not stop_event.is_set():
        try:
            reader, writer = await asyncio.open_connection(host, port)
            print("CONSUMER: 🔌✅ TCP socket connected!")
            break
        except ConnectionRefusedError:
            await asyncio.sleep(1)

    try:
        await triggered_consumer_loop_async(queue, stop_event, reader, writer)

    except Exception as e:
        print(f"CONSUMER: Unexpected error: {e}")
    finally:
        await cleanup_socket(reader, writer)
        stop_event.set()


async def start_null_consumer_async(queue: asyncio.Queue[str], stop_event: asyncio.Event):
    try:
        print("CONSUMER: Null consumer ready to discard all data...")
        while not stop_event.is_set():

            batch = []
            while not queue.empty():
                batch += queue.get_nowait()

            if batch:
                print(f"NULL CONSUMER: 📤 Discarded batch of {len(batch)} items")

            await asyncio.sleep(1 / 60)  # Simulate 60 fps

    except Exception as e:
        print(f"NULL CONSUMER: Unexpected error: {e}")
    finally:
        stop_event.set()


async def start_socket_bridge_async(
        queue: asyncio.Queue[str],
        stop_event: asyncio.Event,
        simple_stream: ISimpleStreamAsync,
        reg_tune_logger: Optional[IRegTuneLogWriter] = None,
):

    print(f"PRODUCER: 🔌⏳ Opening input stream...")
    async with simple_stream as sstream:
        print("PRODUCER: 🔌✅ Input stream opened!")

        while not stop_event.is_set():
            print("PRODUCER: ⏳ Waiting for data...")
            data = await sstream.read()
            if data is None:
                print("PRODUCER: 🔌❌ Connection closed by peer.")
                break
            print(f"PRODUCER: 📥 Received {len(data)} bytes")
            reg_tune_logger.log(data)

            command_line = max4live_udp_cleaner.clean_udp_message(data).decode("utf-8")

            await queue.put(command_line)
            print(f"PRODUCER: 📝 Queued: {command_line}")

    stop_event.set()

async def run_producer_consumer_tasks(producer_task, consumer_task):
    await asyncio.gather(producer_task, consumer_task)


def main():
    lua_tcp_port = 8888
    max4live_udp_port = 9999

    try:
        command_queue: asyncio.Queue[str] = asyncio.Queue(maxsize=1000)
        stop_event = asyncio.Event()

        asyncio.run(
            run_producer_consumer_tasks(
                start_socket_bridge_async(
                    command_queue, stop_event,
                    #UDPSimpleStreamAsync(host="127.0.0.1", port=max4live_udp_port),
                    FileSimpleStreamAsync("reg_tune4.bin.txt"),
                    reg_tune_logger=RegTuneLogWriter("reg_tune5.bin.txt")),
                #start_socket_consumer_async(command_queue, stop_event, host="localhost", port=lua_tcp_port),
                start_null_consumer_async(command_queue, stop_event)
            )
        )

    except KeyboardInterrupt:
        print("\n👋 Interrupted by user")
    except Exception as e:
        print(f"Fatal error: {e}")
    finally:
        print("👋 Exiting...")


if __name__ == "__main__":
    main()
