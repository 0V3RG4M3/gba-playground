import asyncio
import tempfile
import os
import sys
from pathlib import Path
from unittest.mock import Mock, patch
import pytest

# Add the remote/pyGBA directory to sys.path for testing
test_dir = Path(__file__).parent
remote_pygba_dir = test_dir.parent.parent.parent / "remote" / "pyGBA"
if str(remote_pygba_dir) not in sys.path:
    sys.path.insert(0, str(remote_pygba_dir))

from simple_stream_async import (
    ISimpleStreamAsync,
    TCPSimpleStreamAsync,
    UDPSimpleStreamAsync,
    UDPProtocol,
    FileSimpleStreamAsync
)


@pytest.mark.asyncio
async def test_interface_methods_not_implemented():
    """Test that interface methods raise NotImplementedError."""
    stream = ISimpleStreamAsync()

    with pytest.raises(NotImplementedError):
        await stream.push(b"test")

    with pytest.raises(NotImplementedError):
        await stream.read()

    with pytest.raises(NotImplementedError):
        async with stream:
            pass


class TestUDPProtocol:
    """Test the UDPProtocol helper class."""

    def test_init(self):
        """Test protocol initialization."""
        queue = asyncio.Queue()
        protocol = UDPProtocol(queue)
        assert protocol.queue == queue

    def test_datagram_received(self):
        """Test datagram reception."""
        queue = asyncio.Queue()
        protocol = UDPProtocol(queue)

        test_data = b"test_data"
        test_addr = ("127.0.0.1", 8888)

        protocol.datagram_received(test_data, test_addr)
        assert not queue.empty()
        data, addr = queue.get_nowait()
        assert data == test_data
        assert addr == test_addr

    def test_error_received(self, capsys):
        """Test error handling with output capture."""
        queue = asyncio.Queue()
        protocol = UDPProtocol(queue)

        test_error = Exception("Test error")
        protocol.error_received(test_error)

        captured = capsys.readouterr()
        assert "Error received: Test error" in captured.out


class TestTCPSimpleStreamAsync:
    """Test the TCPSimpleStreamAsync class."""

    def test_init(self):
        """Test TCP stream initialization."""
        host = "localhost"
        port = 8888
        stream = TCPSimpleStreamAsync(host, port)

        assert stream.host == host
        assert stream.port == port
        assert stream.reader is None
        assert stream.writer is None
        assert not stream._is_opened

    def test_sync_context_manager_not_supported(self):
        """Test that sync context manager is not supported."""
        stream = TCPSimpleStreamAsync("localhost", 8888)

        with pytest.raises(NotImplementedError):
            with stream:
                pass

    @pytest.mark.asyncio
    async def test_connection_lifecycle(self):
        """Test TCP connection lifecycle with mock server."""
        # Start a mock TCP server
        server = await asyncio.start_server(
            self._handle_server_client,
            host="127.0.0.1",
            port=0  # Let OS choose port
        )
        server_port = server.sockets[0].getsockname()[1]

        try:
            stream = TCPSimpleStreamAsync("127.0.0.1", server_port)
            async with stream:
                assert stream._is_opened
                assert stream.reader is not None
                assert stream.writer is not None

                # Test data exchange
                test_data = b"hello"
                await stream.push(test_data)
                response = await stream.read()
                assert response == test_data  # Echo server should return same data
        finally:
            server.close()
            await server.wait_closed()

    @pytest.mark.asyncio
    async def _handle_server_client(self, reader, writer):
        """Mock TCP server handler that echoes data back."""
        try:
            while True:
                data = await reader.read(1024)
                if not data:
                    break
                writer.write(data)
                await writer.drain()
        finally:
            writer.close()
            await writer.wait_closed()

    @pytest.mark.asyncio
    async def test_operations_without_context(self):
        """Test that operations fail when stream is not opened."""
        stream = TCPSimpleStreamAsync("localhost", 8888)

        with pytest.raises(AssertionError):
            await stream.push(b"test")

        with pytest.raises(AssertionError):
            await stream.read()


class TestUDPSimpleStreamAsync:
    """Test the UDPSimpleStreamAsync class."""

    def test_init(self):
        """Test UDP stream initialization."""
        host = "localhost"
        port = 8888
        stream = UDPSimpleStreamAsync(host, port)

        assert stream.host == host
        assert stream.port == port
        assert stream._transport is None
        assert not stream._is_opened
        assert isinstance(stream._protocol, UDPProtocol)

    def test_sync_context_manager_not_supported(self):
        """Test that sync context manager is not supported."""
        stream = UDPSimpleStreamAsync("localhost", 8888)

        with pytest.raises(NotImplementedError):
            with stream:
                pass

    @pytest.mark.asyncio
    async def test_udp_communication(self):
        """Test UDP communication with actual socket."""
        # Create UDP stream bound to a random port
        stream = UDPSimpleStreamAsync("127.0.0.1", 0)

        async with stream:
            assert stream._is_opened
            assert stream._transport is not None

            # Get the actual bound port
            sock = stream._transport.get_extra_info('socket')
            bound_port = sock.getsockname()[1]

            # Create a separate UDP socket for testing
            test_socket = await asyncio.get_event_loop().create_datagram_endpoint(
                asyncio.DatagramProtocol,
                remote_addr=("127.0.0.1", bound_port)
            )

            try:
                # Send test data
                test_data = b"hello udp"
                test_socket[0].sendto(test_data, ("127.0.0.1", bound_port))

                # Receive response
                response = await stream.read()
                assert response == test_data
            finally:
                test_socket[0].close()

    @pytest.mark.asyncio
    async def test_operations_without_context(self):
        """Test that operations fail when stream is not opened."""
        stream = UDPSimpleStreamAsync("localhost", 8888)

        with pytest.raises(AssertionError):
            await stream.push(b"test")

        with pytest.raises(AssertionError):
            await stream.read()



class TestFileSimpleStreamAsync:
    """Test the FileSimpleStreamAsync class."""

    def setup_method(self):
        """Setup test file."""
        self.temp_file = tempfile.NamedTemporaryFile(delete=False)
        self.temp_filename = self.temp_file.name
        self.temp_file.close()

    def teardown_method(self):
        """Cleanup test file."""
        if os.path.exists(self.temp_filename):
            os.unlink(self.temp_filename)

    def test_init(self):
        """Test file stream initialization."""
        stream = FileSimpleStreamAsync(self.temp_filename)
        assert not stream._is_opened
        assert stream._iterator is None

    def test_sync_context_manager_not_supported(self):
        """Test that sync context manager is not supported."""
        stream = FileSimpleStreamAsync(self.temp_filename)

        with pytest.raises(NotImplementedError):
            with stream:
                pass

    @pytest.mark.asyncio
    async def test_push_not_supported(self):
        """Test that push operation is not supported."""
        stream = FileSimpleStreamAsync(self.temp_filename)

        with pytest.raises(NotImplementedError):
            await stream.push(b"test")

    @pytest.mark.asyncio
    @patch('reg_tune_logger.RegTuneLogReaderAsync')
    async def test_read_success(self, mock_reader_class):
        """Test successful async read operation."""
        mock_reader = Mock()
        test_data = b"test_file_data"

        # Create a proper async iterator mock
        class AsyncIteratorMock:
            async def __anext__(self):
                return test_data

        # Create the mock iterator instance
        mock_iterator = AsyncIteratorMock()
        mock_reader.read.return_value = mock_iterator
        mock_reader_class.return_value = mock_reader

        async with FileSimpleStreamAsync(self.temp_filename) as stream:
            assert stream._is_opened
            result = await stream.read()
            assert result == test_data

    @pytest.mark.asyncio
    @patch('reg_tune_logger.RegTuneLogReaderAsync')
    async def test_read_stop_iteration(self, mock_reader_class):
        """Test read operation when iterator is exhausted."""
        mock_reader = Mock()

        # Create a proper async iterator mock that raises StopAsyncIteration
        class AsyncIteratorMock:
            async def __anext__(self):
                raise StopAsyncIteration()

        # Create the mock iterator instance
        mock_iterator = AsyncIteratorMock()
        mock_reader.read.return_value = mock_iterator
        mock_reader_class.return_value = mock_reader

        stream = FileSimpleStreamAsync(self.temp_filename)
        async with stream:
            result = await stream.read()
            assert result == b''

    @pytest.mark.asyncio
    async def test_operations_without_context(self):
        """Test that operations fail when stream is not opened."""
        stream = FileSimpleStreamAsync(self.temp_filename)

        with pytest.raises(AssertionError):
            await stream.read()
