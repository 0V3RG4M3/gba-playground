import unittest
import socket
import threading
import time
import tempfile
import os
import sys
from pathlib import Path
from unittest.mock import Mock, patch

# Add the remote/pyGBA directory to sys.path for testing
test_dir = Path(__file__).parent
remote_pygba_dir = test_dir.parent.parent.parent / "remote" / "pyGBA"
if str(remote_pygba_dir) not in sys.path:
    sys.path.insert(0, str(remote_pygba_dir))

# Now import normally (as if we're in the same directory)
from simple_stream import (
    ISimpleStream,
    UDPSimpleStream,
    TCPSimpleStream,
    NullSimpleStream,
    FileSimpleStream
)


class TestISimpleStream(unittest.TestCase):
    """Test the ISimpleStream interface."""

    def test_interface_methods_not_implemented(self):
        """Test that interface methods raise NotImplementedError."""
        stream = ISimpleStream()

        with self.assertRaises(NotImplementedError):
            stream.push(b"test")

        with self.assertRaises(NotImplementedError):
            stream.read()

        with self.assertRaises(NotImplementedError):
            stream.__enter__()

        with self.assertRaises(NotImplementedError):
            stream.__exit__(None, None, None)


class TestUDPSimpleStream(unittest.TestCase):
    """Test the UDPSimpleStream class."""

    def setUp(self):
        self.host = "127.0.0.1"
        self.port = 0  # Let OS choose available port

    def test_init(self):
        """Test UDP stream initialization."""
        stream = UDPSimpleStream(self.host, self.port)
        self.assertEqual(stream.host, self.host)
        self.assertEqual(stream.port, self.port)
        self.assertFalse(stream._is_opened)

    def test_context_manager(self):
        """Test UDP stream context manager functionality."""
        stream = UDPSimpleStream(self.host, self.port)

        with stream as s:
            self.assertTrue(s._is_opened)
            self.assertIsInstance(s._udp_socket, socket.socket)

        self.assertFalse(stream._is_opened)

    def test_context_manager_bind_failure(self):
        """Test UDP stream context manager with bind failure."""
        # First, bind a socket to a specific port to ensure it's in use
        blocking_socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        blocking_socket.bind((self.host, 0))  # Let OS choose port
        used_port = blocking_socket.getsockname()[1]

        try:
            # Now try to bind to the same port with UDPSimpleStream
            stream = UDPSimpleStream(self.host, used_port)

            with self.assertRaises(Exception):
                with stream:
                    pass
        finally:
            blocking_socket.close()

    def test_read_write_cycle(self):
        """Test UDP stream read/write functionality."""
        # This test demonstrates UDP communication but is simplified
        # because UDP doesn't have built-in coordination like TCP

        # Create a UDP stream that will listen on a specific port
        with UDPSimpleStream(self.host, 0) as stream:
            # Get the actual port the stream is bound to
            actual_port = stream._udp_socket.getsockname()[1]

            # Set a timeout to prevent hanging
            stream._udp_socket.settimeout(1.0)

            test_data = b"test_message"

            # Create a separate sender socket
            sender_socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

            def send_data():
                time.sleep(0.1)  # Small delay to ensure receiver is ready
                sender_socket.sendto(test_data, (self.host, actual_port))
                sender_socket.close()

            # Start sender in background
            sender_thread = threading.Thread(target=send_data)
            sender_thread.daemon = True
            sender_thread.start()

            try:
                # Read the data
                response = stream.read()
                self.assertEqual(response, test_data)
            except socket.timeout:
                self.fail("UDP read operation timed out")

            sender_thread.join(timeout=1)

    def test_operations_without_context_manager(self):
        """Test that operations fail when stream is not opened."""
        stream = UDPSimpleStream(self.host, self.port)

        with self.assertRaises(AssertionError):
            stream.read()

        with self.assertRaises(AssertionError):
            stream.push(b"test")


class TestTCPSimpleStream(unittest.TestCase):
    """Test the TCPSimpleStream class."""

    def setUp(self):
        self.host = "127.0.0.1"

    def test_init(self):
        """Test TCP stream initialization."""
        port = 8080
        stream = TCPSimpleStream(self.host, port)
        self.assertEqual(stream.host, self.host)
        self.assertEqual(stream.port, port)
        self.assertFalse(stream._is_opened)

    def test_context_manager_connection_failure(self):
        """Test TCP stream context manager with connection failure."""
        # Try to connect to a port that's not listening
        stream = TCPSimpleStream(self.host, 9999)

        with self.assertRaises(Exception):
            with stream:
                pass

    def test_read_write_with_mock_server(self):
        """Test TCP stream read/write with a mock server."""
        # Create a simple TCP server
        server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        server_socket.bind((self.host, 0))
        server_port = server_socket.getsockname()[1]
        server_socket.listen(1)

        test_data = b"test_tcp_message"

        def server_thread():
            conn, addr = server_socket.accept()
            data = conn.recv(1024)
            conn.sendall(data)  # Echo back
            conn.close()
            server_socket.close()

        # Start server in background
        thread = threading.Thread(target=server_thread)
        thread.daemon = True
        thread.start()

        # Give server time to start
        time.sleep(0.1)

        # Test client
        with TCPSimpleStream(self.host, server_port) as stream:
            stream.push(test_data)
            response = stream.read()
            self.assertEqual(response, test_data)

        thread.join(timeout=1)

    def test_operations_without_context_manager(self):
        """Test that operations fail when stream is not opened."""
        stream = TCPSimpleStream(self.host, 8080)

        with self.assertRaises(AssertionError):
            stream.read()

        with self.assertRaises(AssertionError):
            stream.push(b"test")


class TestNullSimpleStream(unittest.TestCase):
    """Test the NullSimpleStream class."""

    def test_init_default_rate(self):
        """Test NullSimpleStream initialization with default rate."""
        stream = NullSimpleStream()
        self.assertEqual(stream.rate_hz, 10.0)
        self.assertFalse(stream._is_opened)

    def test_init_custom_rate(self):
        """Test NullSimpleStream initialization with custom rate."""
        rate = 5.0
        stream = NullSimpleStream(rate)
        self.assertEqual(stream.rate_hz, rate)
        self.assertFalse(stream._is_opened)

    def test_context_manager(self):
        """Test NullSimpleStream context manager functionality."""
        stream = NullSimpleStream()

        with stream as s:
            self.assertTrue(s._is_opened)

        self.assertFalse(stream._is_opened)

    def test_push_does_nothing(self):
        """Test that push operation does nothing but requires open stream."""
        stream = NullSimpleStream()

        with stream:
            # Should not raise any exception
            stream.push(b"test_data")

    def test_read_returns_empty_bytes_with_delay(self):
        """Test that read returns empty bytes after a delay."""
        rate = 100.0  # High rate for faster test
        stream = NullSimpleStream(rate)

        with stream:
            start_time = time.time()
            result = stream.read()
            end_time = time.time()

            self.assertEqual(result, b'')
            # Check that some delay occurred (allowing for timing variations)
            self.assertGreaterEqual(end_time - start_time, 0.008)  # Slightly less than 1/100

    def test_operations_without_context_manager(self):
        """Test that operations fail when stream is not opened."""
        stream = NullSimpleStream()

        with self.assertRaises(AssertionError):
            stream.read()

        with self.assertRaises(AssertionError):
            stream.push(b"test")


class TestFileSimpleStream(unittest.TestCase):
    """Test the FileSimpleStream class."""

    def setUp(self):
        self.temp_file = tempfile.NamedTemporaryFile(delete=False)
        self.temp_filename = self.temp_file.name
        self.temp_file.close()

    def tearDown(self):
        if os.path.exists(self.temp_filename):
            os.unlink(self.temp_filename)

    @patch('simple_stream.RegTuneLogReader')
    def test_init(self, mock_reader_class):
        """Test FileSimpleStream initialization."""
        mock_reader = Mock()
        mock_reader_class.return_value = mock_reader

        stream = FileSimpleStream(self.temp_filename)

        self.assertEqual(stream.reader, mock_reader)
        self.assertFalse(stream._is_opened)
        self.assertIsNone(stream._iterator)
        mock_reader_class.assert_called_once_with(self.temp_filename, time_scale=1.0)

    @patch('simple_stream.RegTuneLogReader')
    def test_context_manager(self, mock_reader_class):
        """Test FileSimpleStream context manager functionality."""
        mock_reader = Mock()
        mock_iterator = Mock()
        mock_reader.read.return_value = mock_iterator
        mock_reader_class.return_value = mock_reader

        stream = FileSimpleStream(self.temp_filename)

        with stream as s:
            self.assertTrue(s._is_opened)
            self.assertEqual(s._iterator, mock_iterator)
            mock_reader.read.assert_called_once()

        self.assertFalse(stream._is_opened)
        self.assertIsNone(stream._iterator)

    @patch('simple_stream.RegTuneLogReader')
    def test_push_not_supported(self, mock_reader_class):
        """Test that push operation is not supported."""
        mock_reader_class.return_value = Mock()
        stream = FileSimpleStream(self.temp_filename)

        with self.assertRaises(NotImplementedError) as cm:
            stream.push(b"test")

        self.assertIn("does not support push operation", str(cm.exception))

    @patch('simple_stream.RegTuneLogReader')
    def test_read_success(self, mock_reader_class):
        """Test successful read operation."""
        mock_reader = Mock()
        mock_iterator = Mock()
        test_data = b"test_file_data"
        mock_iterator.__next__ = Mock(return_value=test_data)
        mock_reader.read.return_value = mock_iterator
        mock_reader_class.return_value = mock_reader

        stream = FileSimpleStream(self.temp_filename)

        with stream:
            result = stream.read()
            self.assertEqual(result, test_data)

    @patch('simple_stream.RegTuneLogReader')
    def test_read_stop_iteration(self, mock_reader_class):
        """Test read operation when iterator is exhausted."""
        mock_reader = Mock()
        mock_iterator = Mock()
        mock_iterator.__next__ = Mock(side_effect=StopIteration)
        mock_reader.read.return_value = mock_iterator
        mock_reader_class.return_value = mock_reader

        stream = FileSimpleStream(self.temp_filename)

        with stream:
            result = stream.read()
            self.assertEqual(result, b'')

    @patch('simple_stream.RegTuneLogReader')
    def test_read_without_context_manager(self, mock_reader_class):
        """Test that read fails when stream is not opened."""
        mock_reader_class.return_value = Mock()
        stream = FileSimpleStream(self.temp_filename)

        with self.assertRaises(AssertionError):
            stream.read()


if __name__ == '__main__':
    unittest.main()
