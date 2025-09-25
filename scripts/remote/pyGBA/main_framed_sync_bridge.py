import asyncio
import time
from typing import Optional

# Local imports - back to absolute imports for direct execution
from reg_tune_csv import RegTuneCsvWriter
from reg_tune_logger import IRegTuneLogWriter, RegTuneLogWriter
from simple_stream_async import ISimpleStreamAsync, UDPSimpleStreamAsync, FileSimpleStreamAsync, TCPSimpleStreamAsync
import max4live_udp_cleaner


async def start_socket_consumer_async(queue: asyncio.Queue[str], stop_event: asyncio.Event, simple_stream: ISimpleStreamAsync):
    print(f"CONSUMER: 🔌⏳ Connecting to simple stream...")
    async with simple_stream as sstream:
        while not stop_event.is_set():
            # Wait for trigger (e.g., a line with 'TRIGGER\n')
            trigger = await sstream.read()
            if not trigger:
                print("CONSUMER: 🔌❌ Connection closed by peer.")
                return

            batch = []
            while not queue.empty():
                batch.append(queue.get_nowait())

            if batch:
                commands = ("\n".join(batch) + "\n").encode()
                if len(commands) > 1024:
                    print(f"CONSUMER: ⚠️ Warning: Sending a large batch of {len(commands)} bytes")
                    continue
                print(f"CONSUMER: sending {len(commands)} bytes")
                print(f"CONSUMER: commands: {commands}")
                await sstream.push(commands)

                print(f"CONSUMER: 📤 Sent {len(commands)} bytes after trigger")


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
        print(f"PRODUCER: 🔌✅ Input stream opened!")

        while not stop_event.is_set():
            # print(f"PRODUCER: ⏳ Waiting for data...")
            data = await sstream.read()
            if data is None:
                print(f"PRODUCER: 🔌❌ Connection closed by peer.")
                break
            # print(f"PRODUCER: 📥 Received {len(data)} bytes")
            if reg_tune_logger:
                reg_tune_logger.log(data)

            command_line = max4live_udp_cleaner.clean_udp_message(data).decode("utf-8")

            await queue.put(command_line)
            # print(f"PRODUCER: 📝 Queued: {command_line}")

    stop_event.set()

async def run_producer_consumer_tasks(producer_task, consumer_task):
    await asyncio.gather(producer_task, consumer_task)


def main():
    lua_tcp_port = 8888
    max4live_udp_port = 9999


    command_queue: asyncio.Queue[str] = asyncio.Queue(maxsize=1000)
    stop_event = asyncio.Event()

    asyncio.run(
        run_producer_consumer_tasks(
            start_socket_bridge_async(
                command_queue, stop_event,
                simple_stream=UDPSimpleStreamAsync(host="127.0.0.1", port=max4live_udp_port),
                #simple_stream=FileSimpleStreamAsync("reg_tune6.bin.txt", loop=True, time_scale=1.0),
                #reg_tune_logger=RegTuneLogWriter("reg_tune6.bin.txt")
            ),

            start_socket_consumer_async(
                command_queue, stop_event,
                simple_stream=TCPSimpleStreamAsync(host="localhost", port=lua_tcp_port),
            )
            #start_null_consumer_async(command_queue, stop_event)
        )
    )
    try:
        pass
    except KeyboardInterrupt:
        print("\n👋 Interrupted by user")
    except Exception as e:
        print(f"Fatal error: {e}")
    finally:
        print("👋 Exiting...")


if __name__ == "__main__":
    main()
