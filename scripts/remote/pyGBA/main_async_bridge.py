import asyncio
from collections import deque
import time
import sys
from typing import Optional

MAX_BUFFER_SIZE = 1000

class LogPlayer:
    def __init__(self):
        self.buffer = deque(maxlen=MAX_BUFFER_SIZE)
        self.running = True
        self.writer: Optional[asyncio.StreamWriter] = None

    def log_to_command(self, log_line: str):
        """Parse a log line into timestamp and command."""
        try:
            log_line = log_line[:-1]  # remove newline
            ts, cmd, address, value = log_line.split(",")
            command = " ".join([cmd, address, value])
            return float(ts), command
        except ValueError as e:
            print(f"Error parsing log line: {log_line}")
            print(f"Error details: {e}")
            return None

    async def read_log_file(self, log_file: str, timescale: float=1.0):
        """Read and replay log file with original timing."""
        try:
            with open(log_file, "r") as f:
                # Verify header
                header = f.readline()
                if header != "time,cmd,address,value\n":
                    raise ValueError("Invalid CSV header")

                t0 = time.time()
                for line in f:
                    if not self.running:
                        break

                    result = self.log_to_command(line)
                    if result is None:
                        continue

                    ts, command = result

                    # Adjust timestamp according to timescale
                    ts = ts / timescale

                    # Calculate and apply delay
                    delay = float(ts) - (time.time() - t0)
                    if delay > 0:
                        await asyncio.sleep(delay)

                    self.buffer.append(command)
                    print(f"📝 Queued: {command}")

        except FileNotFoundError:
            print(f"Error: Could not find log file: {log_file}")
            self.running = False
        except Exception as e:
            print(f"Error reading log file: {e}")
            self.running = False

    async def send_commands(self):
        """Send buffered commands at 60Hz."""
        t_next = time.time()

        while self.running and self.writer:
            try:
                t_next += 1 / 60
                t_now = time.time()
                delay = max(0.0, t_next - t_now)
                await asyncio.sleep(delay)

                if self.buffer:
                    commands = "\n".join(self.buffer) + "\n"
                    self.buffer.clear()

                    self.writer.write(commands.encode())
                    await self.writer.drain()

                    print(f"📤 Sent {len(commands)} bytes, delay:{int(delay * 1000)}ms")

            except ConnectionError as e:
                print(f"Connection error: {e}")
                self.running = False
                break
            except Exception as e:
                print(f"Error in send_commands: {e}")
                self.running = False
                break

    async def cleanup(self):
        """Clean up resources."""
        if self.writer:
            try:
                self.writer.close()
                await self.writer.wait_closed()
            except Exception as e:
                print(f"Error during cleanup: {e}")

    async def run(self, log_file: str, host: str, port: int, timescale: float = 1.0):
        """Main entry point."""
        try:
            print(f"🔌 Connecting to {host}:{port}...")
            reader, self.writer = await asyncio.open_connection(host, port)
            print("✅ Connected!")

            # Start both tasks
            await asyncio.gather(
                self.read_log_file(log_file, timescale),
                self.send_commands()
            )

        except ConnectionRefusedError:
            print(f"Could not connect to {host}:{port}")
        except Exception as e:
            print(f"Unexpected error: {e}")
        finally:
            self.running = False
            await self.cleanup()

def main():
    # if len(sys.argv) != 4:
    #     print("Usage: python main_async_log_player.py <logfile> <host> <port>")
    #     sys.exit(1)

    log_file = "reg_tune.csv"  # sys.argv[1]
    host = "localhost"  # sys.argv[2]
    port = 8888  # int(sys.argv[3])
    timescale = 1  # float(sys.argv[4]) if len(sys.argv) > 4 else 1.0

    player = LogPlayer()

    try:
        asyncio.run(player.run(log_file, host, port, timescale))
    except KeyboardInterrupt:
        print("\n👋 Interrupted by user")
    except Exception as e:
        print(f"Fatal error: {e}")
    finally:
        player.running = False

if __name__ == "__main__":
    main()
