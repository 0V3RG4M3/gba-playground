"""
Main bridge module that handles real-time communication between Max4Live (UDP)
and mGBA (TCP), synchronizing register writes via frames.
"""

import asyncio
from typing import Optional

import max4live_udp_cleaner
from reg_tune_logger import IRegTuneLogWriter
from simple_stream_async import ISimpleStreamAsync, UDPSimpleStreamAsync, TCPSimpleStreamAsync, FileSimpleStreamAsync


def format_pretty_regstate(regstate, batch):
    frame_id = -1
    for cmd in batch:
        items = cmd.split()
        if len(items) != 4:
            print(cmd)
            continue
        if len(items) == 4:
            frame_id, cmd_type, addr_str, value_str = items
            regstate.set_value(int(addr_str[2:], 16), int(value_str[2:], 16))
    return f"{frame_id} {regstate.to_string()[1]}"


async def start_socket_consumer_async(queue: asyncio.Queue[str], stop_event: asyncio.Event, simple_stream: ISimpleStreamAsync):
    print("CONSUMER: 🔌⏳ Connecting to simple stream...")

    import pretty_registry
    regstate = pretty_registry.RegistryState()
    print("XXXX", regstate.to_string()[0])

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

            if not batch:
                continue


            commands = ("\n".join(batch) + "\n").encode()
            if len(commands) > 1024:
                print(f"CONSUMER: ⚠️ Warning: Sending a large batch of {len(commands)} bytes")
                continue
            # print(f"CONSUMER: sending {len(commands)} bytes")
            # print(f"CONSUMER: commands: {commands}")
            print(f"CONSUMER: {format_pretty_regstate(regstate, batch)}")
            await sstream.push(commands)

            # print(f"CONSUMER: 📤 Sent {len(commands)} bytes after trigger")


async def start_null_consumer_async(queue: asyncio.Queue[str], stop_event: asyncio.Event):
    try:
        import pretty_registry
        regstate = pretty_registry.RegistryState()

        print("CONSUMER: Null consumer ready to discard all data...")
        print("XXXX", regstate.to_string()[0])
        while not stop_event.is_set():
            await asyncio.sleep(1 / 60)

            batch = []
            while not queue.empty():
                batch.append(queue.get_nowait())

            if not batch:
                continue

            print(format_pretty_regstate(regstate, batch))


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

    print("PRODUCER: 🔌⏳ Opening input stream...")
    async with simple_stream as sstream:
        print("PRODUCER: 🔌✅ Input stream opened!")

        while not stop_event.is_set():
            # print(f"PRODUCER: ⏳ Waiting for data...")
            data = await sstream.read()
            if data is None:
                print("PRODUCER: 🔌❌ Connection closed by peer.")
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
                # simple_stream=FileSimpleStreamAsync("reg_tune.bin.txt", loop=True, time_scale=1.0),
                #reg_tune_logger=RegTuneLogWriter("reg_tune.bin.txt")
            ),

            start_socket_consumer_async(
                command_queue, stop_event,
                simple_stream=TCPSimpleStreamAsync(host="localhost", port=lua_tcp_port),
            )
            # start_null_consumer_async(command_queue, stop_event)
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
