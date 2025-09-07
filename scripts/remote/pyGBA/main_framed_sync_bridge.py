import asyncio
import threading
from collections import deque
import time
import sys
from typing import Optional

MAX_BUFFER_SIZE = 1000


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


async def triggered_consumer_loop_async(queue: asyncio.Queue, stop_event: asyncio.Event, reader: asyncio.StreamReader,
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
            batch.append(queue.get_nowait())

        if batch:
            commands = ("\n".join(batch) + "\n").encode()
            if len(commands) > 1024:
                print(f"CONSUMER: ⚠️ Warning: Sending a large batch of {len(commands)} bytes")

            writer.write(commands)
            await writer.drain()

            print(f"CONSUMER: 📤 Sent {len(commands)} bytes after trigger")


async def start_socket_consumer_async(queue: asyncio.Queue, stop_event: asyncio.Event, host: str, port: int):
    reader: Optional[asyncio.StreamReader] = None
    writer: Optional[asyncio.StreamWriter] = None
    try:
        print(f"🔌 Connecting to {host}:{port}...")
        reader, writer = await asyncio.open_connection(host, port)
        print("✅ Connected!")

        await triggered_consumer_loop_async(queue, stop_event, reader, writer)

    except ConnectionRefusedError:
        print(f"CONSUMER: Could not connect to {host}:{port}")
    except Exception as e:
        print(f"CONSUMER: Unexpected error: {e}")
    finally:
        await cleanup_socket(reader, writer)
        stop_event.set()


async def start_offline_producer_async(queue: asyncio.Queue, stop_event: asyncio.Event, log_file: str,
                                       timescale: float = 1.0):
    """Read and replay log file with original timing."""
    try:
        with open(log_file, "r") as f:
            # Verify header
            header = f.readline()
            if header != "time,cmd,address,value\n":
                raise ValueError("Invalid CSV header")

            # TODO: use an event to signal consumer is ready
            # Initial delay to allow consumer to connect
            await asyncio.sleep(3)

            t0 = time.time()
            for line in f:
                if stop_event.is_set():
                    break
                line = line[:-1]  # remove newline
                try:
                    # Parse log line
                    ts, cmd, address, value = line.split(",")
                    command = " ".join([cmd, address, value])
                    ts = float(ts) / timescale

                    # Calculate and apply delay
                    delay = float(ts) - (time.time() - t0)
                    if delay > 0:
                        await asyncio.sleep(delay)
                    else:
                        print(f"PRODUCER: ⚠️ Warning: behind schedule by {-delay:.3f} seconds ({-delay*60:.2f} frames)")

                    await queue.put(command)
                    print(f"PRODUCER: 📝 Queued: {command}")

                except ValueError as e:
                    print(f"PRODUCER: Error parsing log line: {line}")
                    print(f"PRODUCER: Error details: {e}")
                    continue

    except FileNotFoundError:
        print(f"PRODUCER: Error: Could not find log file: {log_file}")
    except Exception as e:
        print(f"PRODUCER: Error reading log file: {e}")
    finally:
        stop_event.set()


async def run_producer_consumer_tasks(producer_task, consumer_task):
    await asyncio.gather(producer_task, consumer_task)


def main():
    log_file = "reg_tune.csv"  # sys.argv[1]
    host = "localhost"  # sys.argv[2]
    port = 8888  # int(sys.argv[3])
    timescale = 1  # float(sys.argv[4]) if len(sys.argv) > 4 else 1.0

    try:
        command_queue = asyncio.Queue()
        stop_event = asyncio.Event()

        asyncio.run(
            run_producer_consumer_tasks(
                start_offline_producer_async(command_queue, stop_event, log_file, timescale),
                start_socket_consumer_async(command_queue, stop_event, host, port),
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
