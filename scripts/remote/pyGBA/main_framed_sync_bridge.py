import asyncio
import time
from typing import Optional

# Local imports
from reg_tune_csv import RegTuneCsvWriter
from asyncio_udp import UDPConnection

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


async def start_socket_producer_async(queue: asyncio.Queue, stop_event: asyncio.Event, host: str, port: int,
                                      reg_tune_logger: Optional[RegTuneCsvWriter] = None):
    loop = asyncio.get_running_loop()
    conn = UDPConnection()

    print(f"PRODUCER: 🔌⏳ Binding UDP endpoint to {host}:{port}...")
    transport, _ = await loop.create_datagram_endpoint(
        protocol_factory=lambda: conn,
        local_addr=(host, port),
    )
    conn.connection_made(transport)
    print("PRODUCER: 🔌✅ UDP endpoint ready!")

    try:
        is_recording = False
        while not stop_event.is_set():
            data, addr = await conn.recv()
            command = data.decode().strip()

            if not command:
                continue

            if command.startswith("WRITE"):
                await queue.put(command)
                if is_recording and reg_tune_logger:
                    reg_tune_logger.log(command)
                print(f"PRODUCER: 📝 Queued: {command}")
                # self.log(command)
            elif command.startswith("REC"):
                reg_tune_logger.newlogfile()
                reg_tune_logger.reset_timer()
                is_recording = True
            elif command.startswith("STOP"):
                reg_tune_logger.log("STOP -1 -1")
                is_recording = False
                print("STOP")

    except Exception as e:
        print(f"PRODUCER: Unexpected error: {e}")
    finally:
        conn.close()
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
                        print(
                            f"PRODUCER: ⚠️ Warning: behind schedule by {-delay:.3f} seconds ({-delay * 60:.2f} frames)")

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
    lua_tcp_port = 8888
    max4live_udp_port = 9999

    try:
        command_queue = asyncio.Queue()
        stop_event = asyncio.Event()

        asyncio.run(
            run_producer_consumer_tasks(
                # start_offline_producer_async(command_queue, stop_event, log_file="reg_tune.csv", timescale=1.0),
                start_socket_producer_async(command_queue, stop_event, host="localhost", port=max4live_udp_port,
                                            reg_tune_logger=RegTuneCsvWriter("reg_tune2.csv")),
                start_socket_consumer_async(command_queue, stop_event, host="localhost", port=lua_tcp_port),
            )
        )

    except KeyboardInterrupt:
        print("\n👋 Interrupted by user")
    except Exception as e:
        print(f"Fatal error: {e}")
    finally:
        print("👋 Exiting...")


if __name__ == "__main__":
    # TODO: next step is the triple queue design:
    #   - Max4live device must include a frame_id in the command.
    #   - There should be 3 different queues acting as a triple buffer:
    #       - Producer fills queue frame_id % 3
    #       - Consumer reads from queue (frame_id - 1) % 3

    main()
