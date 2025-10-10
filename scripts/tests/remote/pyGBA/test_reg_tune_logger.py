import unittest
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

# Now import normally (as if we're in the same directory)
from reg_tune_logger import (
    extract_frame_id,
    IRegTuneLogWriter,
    RegTuneLogWriter,
    RegTuneLogReader,
    RegTuneLogReaderAsync
)


class TestExtractFrameId(unittest.TestCase):
    """Test the extract_frame_id function."""

    @patch('reg_tune_logger.max4live_udp_cleaner')
    def test_extract_frame_id_valid_data(self, mock_cleaner):
        """Test extracting frame ID from valid data."""
        # Mock the cleaner to return a message with frame ID
        mock_cleaner.clean_udp_message.return_value = b"123 REC some data"

        result = extract_frame_id(b"raw_data")

        self.assertEqual(result, 123)
        mock_cleaner.clean_udp_message.assert_called_once_with(b"raw_data")

    @patch('reg_tune_logger.max4live_udp_cleaner')
    def test_extract_frame_id_empty_data(self, mock_cleaner):
        """Test extracting frame ID from empty data."""
        mock_cleaner.clean_udp_message.return_value = b""

        result = extract_frame_id(b"raw_data")

        self.assertIsNone(result)

    @patch('reg_tune_logger.max4live_udp_cleaner')
    def test_extract_frame_id_whitespace_only(self, mock_cleaner):
        """Test extracting frame ID from whitespace-only data."""
        mock_cleaner.clean_udp_message.return_value = b"   \n\t  "

        result = extract_frame_id(b"raw_data")

        self.assertIsNone(result)

    @patch('reg_tune_logger.max4live_udp_cleaner')
    def test_extract_frame_id_invalid_number(self, mock_cleaner):
        """Test extracting frame ID when first item is not a number."""
        mock_cleaner.clean_udp_message.return_value = b"not_a_number REC data"

        with self.assertRaises(ValueError):
            extract_frame_id(b"raw_data")


class TestIRegTuneLogWriter(unittest.TestCase):
    """Test the IRegTuneLogWriter interface."""

    def test_interface_methods_not_implemented(self):
        """Test that interface methods raise NotImplementedError."""
        writer = IRegTuneLogWriter()

        with self.assertRaises(NotImplementedError):
            writer.newlogfile()

        with self.assertRaises(NotImplementedError):
            writer.log(b"test")


class TestRegTuneLogWriter(unittest.TestCase):
    """Test the RegTuneLogWriter class."""

    def setUp(self):
        self.temp_file = tempfile.NamedTemporaryFile(delete=False)
        self.temp_filename = self.temp_file.name
        self.temp_file.close()

    def tearDown(self):
        if os.path.exists(self.temp_filename):
            os.unlink(self.temp_filename)

    def test_init(self):
        """Test RegTuneLogWriter initialization."""
        writer = RegTuneLogWriter(self.temp_filename)
        self.assertEqual(writer.filename, self.temp_filename)
        self.assertFalse(writer.is_recording)

    def test_newlogfile(self):
        """Test creating a new log file."""
        writer = RegTuneLogWriter(self.temp_filename)

        # Write some content first
        with open(self.temp_filename, "w") as f:
            f.write("existing content")

        # Verify file has content
        with open(self.temp_filename, "r") as f:
            self.assertEqual(f.read(), "existing content")

        # Call newlogfile to clear it
        writer.newlogfile()

        # Verify file is now empty
        with open(self.temp_filename, "r") as f:
            self.assertEqual(f.read(), "")

    @patch('reg_tune_logger.max4live_udp_cleaner')
    def test_log_empty_data(self, mock_cleaner):
        """Test logging empty data."""
        mock_cleaner.clean_udp_message.return_value = b""

        writer = RegTuneLogWriter(self.temp_filename)
        writer.log(b"empty_data")

        # File should not be created or modified
        self.assertFalse(os.path.exists(self.temp_filename) and os.path.getsize(self.temp_filename) > 0)

    @patch('reg_tune_logger.max4live_udp_cleaner')
    def test_log_rec_command(self, mock_cleaner):
        """Test logging REC command starts recording."""
        mock_cleaner.clean_udp_message.return_value = b"123 REC start"

        writer = RegTuneLogWriter(self.temp_filename)
        writer.log(b"123 REC start")

        self.assertTrue(writer.is_recording)
        # Should create empty file
        self.assertTrue(os.path.exists(self.temp_filename))

    @patch('reg_tune_logger.max4live_udp_cleaner')
    def test_log_stop_command(self, mock_cleaner):
        """Test logging STOP command stops recording."""
        mock_cleaner.clean_udp_message.return_value = b"123 STOP end"

        writer = RegTuneLogWriter(self.temp_filename)
        writer.is_recording = True  # Start recording

        with patch('builtins.print') as mock_print:
            writer.log(b"123 STOP end")

        self.assertFalse(writer.is_recording)
        mock_print.assert_called_once_with("STOP")

    @patch('reg_tune_logger.max4live_udp_cleaner')
    def test_log_data_while_recording(self, mock_cleaner):
        """Test logging data while recording is active."""
        mock_cleaner.clean_udp_message.return_value = b"123 DATA some_data"

        writer = RegTuneLogWriter(self.temp_filename)
        writer.is_recording = True

        test_data = b"test_log_data"
        writer.log(test_data)

        # Check that data was written to file
        with open(self.temp_filename, "r", encoding="utf-8") as f:
            content = f.read().strip()
            # Should contain the repr of the decoded data
            expected = repr(test_data.decode("latin1"))
            self.assertEqual(content, expected)

    @patch('reg_tune_logger.max4live_udp_cleaner')
    def test_log_data_while_not_recording(self, mock_cleaner):
        """Test logging data while recording is not active."""
        mock_cleaner.clean_udp_message.return_value = b"123 DATA some_data"

        writer = RegTuneLogWriter(self.temp_filename)
        writer.is_recording = False

        writer.log(b"test_data")

        # File should not be created or should be empty
        if os.path.exists(self.temp_filename):
            with open(self.temp_filename, "r") as f:
                self.assertEqual(f.read(), "")

    @patch('reg_tune_logger.max4live_udp_cleaner')
    def test_log_multiple_entries(self, mock_cleaner):
        """Test logging multiple entries while recording."""
        mock_cleaner.clean_udp_message.side_effect = [
            b"123 REC start",
            b"124 DATA entry1",
            b"125 DATA entry2",
            b"126 STOP end"
        ]

        writer = RegTuneLogWriter(self.temp_filename)

        # Start recording
        writer.log(b"123 REC start")
        self.assertTrue(writer.is_recording)

        # Log some data
        writer.log(b"entry1_data")
        writer.log(b"entry2_data")

        # Stop recording
        writer.log(b"126 STOP end")
        self.assertFalse(writer.is_recording)

        # Check file content - should have REC command + 2 data entries + STOP command = 4 lines
        with open(self.temp_filename, "r", encoding="utf-8") as f:
            lines = f.readlines()
            self.assertEqual(len(lines), 4)  # REC command + 2 data entries + STOP command


class TestRegTuneLogReader(unittest.TestCase):
    """Test the RegTuneLogReader class."""

    def setUp(self):
        self.temp_file = tempfile.NamedTemporaryFile(delete=False, mode='w', encoding='utf-8')
        self.temp_filename = self.temp_file.name

    def tearDown(self):
        self.temp_file.close()
        if os.path.exists(self.temp_filename):
            os.unlink(self.temp_filename)

    def test_init_default_time_scale(self):
        """Test RegTuneLogReader initialization with default time scale."""
        reader = RegTuneLogReader(self.temp_filename)
        self.assertEqual(reader.filename, self.temp_filename)
        self.assertIsNone(reader.time_scale)

    def test_init_custom_time_scale(self):
        """Test RegTuneLogReader initialization with custom time scale."""
        time_scale = 2.0
        reader = RegTuneLogReader(self.temp_filename, time_scale)
        self.assertEqual(reader.time_scale, time_scale)

    def test_init_invalid_time_scale(self):
        """Test RegTuneLogReader initialization with invalid time scale."""
        with self.assertRaises(AssertionError):
            RegTuneLogReader(self.temp_filename, -1.0)

        with self.assertRaises(AssertionError):
            RegTuneLogReader(self.temp_filename, 0.0)

    @patch('reg_tune_logger.extract_frame_id')
    def test_read_no_time_scale(self, mock_extract):
        """Test reading log without time scaling."""
        # Prepare test data
        test_lines = [
            "'123 DATA test1'",
            "'124 DATA test2'",
            "'125 DATA test3'"
        ]

        self.temp_file.write('\n'.join(test_lines))
        self.temp_file.close()

        # Mock frame ID extraction
        mock_extract.side_effect = [123, 124, 125]

        reader = RegTuneLogReader(self.temp_filename, time_scale=None)
        results = list(reader.read())

        self.assertEqual(len(results), 3)
        self.assertEqual(results[0], b"123 DATA test1")
        self.assertEqual(results[1], b"124 DATA test2")
        self.assertEqual(results[2], b"125 DATA test3")

    @patch('reg_tune_logger.extract_frame_id')
    @patch('reg_tune_logger.time')
    def test_read_with_time_scale(self, mock_time, mock_extract):
        """Test reading log with time scaling."""
        # Prepare test data
        test_lines = [
            "'123 DATA test1'",
            "'124 DATA test2'"
        ]

        self.temp_file.write('\n'.join(test_lines))
        self.temp_file.close()

        # Mock time and frame ID extraction
        mock_time.time.side_effect = [100.0, 100.0, 100.5, 101.0]  # Initial time, then current times
        mock_time.sleep = Mock()
        mock_extract.side_effect = [123, 124]

        reader = RegTuneLogReader(self.temp_filename, time_scale=60.0)  # 1 second per frame
        results = list(reader.read())

        self.assertEqual(len(results), 2)
        # Should have called sleep with appropriate delays
        self.assertTrue(mock_time.sleep.called)

    @patch('reg_tune_logger.extract_frame_id')
    def test_read_skip_invalid_frame_ids(self, mock_extract):
        """Test reading log skips entries with invalid frame IDs."""
        test_lines = [
            "'123 DATA test1'",
            "'invalid_entry'",
            "'125 DATA test3'"
        ]

        self.temp_file.write('\n'.join(test_lines))
        self.temp_file.close()

        # Mock frame ID extraction - None for invalid entry
        mock_extract.side_effect = [123, None, 125]

        reader = RegTuneLogReader(self.temp_filename)
        results = list(reader.read())

        self.assertEqual(len(results), 2)
        self.assertEqual(results[0], b"123 DATA test1")
        self.assertEqual(results[1], b"125 DATA test3")

    def test_read_empty_file(self):
        """Test reading an empty log file."""
        self.temp_file.close()

        reader = RegTuneLogReader(self.temp_filename)
        results = list(reader.read())

        self.assertEqual(len(results), 0)


class TestRegTuneLogReaderAsync(unittest.TestCase):
    """Test the RegTuneLogReaderAsync class."""

    def setUp(self):
        self.temp_file = tempfile.NamedTemporaryFile(delete=False, mode='w', encoding='utf-8')
        self.temp_filename = self.temp_file.name

    def tearDown(self):
        self.temp_file.close()
        if os.path.exists(self.temp_filename):
            os.unlink(self.temp_filename)

    def test_init_default_time_scale(self):
        """Test RegTuneLogReaderAsync initialization with default time scale."""
        reader = RegTuneLogReaderAsync(self.temp_filename)
        self.assertEqual(reader.filename, self.temp_filename)
        self.assertIsNone(reader.time_scale)

    def test_init_custom_time_scale(self):
        """Test RegTuneLogReaderAsync initialization with custom time scale."""
        time_scale = 2.0
        reader = RegTuneLogReaderAsync(self.temp_filename, time_scale=time_scale)
        self.assertEqual(reader.time_scale, time_scale)

    def test_init_invalid_time_scale(self):
        """Test RegTuneLogReaderAsync initialization with invalid time scale."""
        with self.assertRaises(AssertionError):
            RegTuneLogReaderAsync(self.temp_filename, time_scale=-1.0)

        with self.assertRaises(AssertionError):
            RegTuneLogReaderAsync(self.temp_filename, time_scale=0.0)


# Pytest-style async tests for RegTuneLogReaderAsync
# These replace the unittest async methods to avoid coroutine warnings
@pytest.mark.asyncio
@patch('reg_tune_logger.extract_frame_id')
async def test_async_reader_no_time_scale(mock_extract):
    """Test async reading log without time scaling."""
    temp_file = tempfile.NamedTemporaryFile(delete=False, mode='w', encoding='utf-8')
    temp_filename = temp_file.name

    try:
        test_lines = [
            "'123 DATA test1'",
            "'124 DATA test2'",
            "'125 DATA test3'"
        ]

        temp_file.write('\n'.join(test_lines))
        temp_file.close()

        # Need 4 calls: first for frame_id0, then for each of the 3 items
        mock_extract.side_effect = [123, 123, 124, 125]

        reader = RegTuneLogReaderAsync(temp_filename, time_scale=None)
        results = []
        async for data in reader.read():
            results.append(data)

        assert len(results) == 3
        assert results[0] == b"123 DATA test1"
        assert results[1] == b"124 DATA test2"
        assert results[2] == b"125 DATA test3"
    finally:
        if os.path.exists(temp_filename):
            os.unlink(temp_filename)


@pytest.mark.asyncio
@patch('reg_tune_logger.extract_frame_id')
@patch('reg_tune_logger.time')
async def test_async_reader_with_time_scale(mock_time, mock_extract):
    """Test async reading log with time scaling."""
    temp_file = tempfile.NamedTemporaryFile(delete=False, mode='w', encoding='utf-8')
    temp_filename = temp_file.name

    try:
        test_lines = [
            "'100 DATA test1'",
            "'102 DATA test2'"
        ]

        temp_file.write('\n'.join(test_lines))
        temp_file.close()

        # Mock time and frame ID extraction
        mock_time.time.side_effect = [100.0, 100.0, 100.5]
        # Need 3 calls: first for frame_id0, then for each of the 2 items
        mock_extract.side_effect = [100, 100, 102]

        # Create an async mock for asyncio.sleep
        async def mock_sleep(duration):
            pass

        with patch('reg_tune_logger.asyncio.sleep', side_effect=mock_sleep) as mock_async_sleep:
            reader = RegTuneLogReaderAsync(temp_filename, time_scale=60.0)
            results = []
            async for data in reader.read():
                results.append(data)

            assert len(results) == 2
            # Should have called asyncio.sleep
            assert mock_async_sleep.called
    finally:
        if os.path.exists(temp_filename):
            os.unlink(temp_filename)


@pytest.mark.asyncio
@patch('reg_tune_logger.extract_frame_id')
async def test_async_reader_runtime_error_no_frame_id(mock_extract):
    """Test async reading raises RuntimeError when first line has no frame ID."""
    temp_file = tempfile.NamedTemporaryFile(delete=False, mode='w', encoding='utf-8')
    temp_filename = temp_file.name

    try:
        test_lines = ["'invalid_data'"]

        temp_file.write('\n'.join(test_lines))
        temp_file.close()

        mock_extract.return_value = None

        reader = RegTuneLogReaderAsync(temp_filename, time_scale=1.0)

        with pytest.raises(RuntimeError):
            async for data in reader.read():
                pass
    finally:
        if os.path.exists(temp_filename):
            os.unlink(temp_filename)


@pytest.mark.asyncio
@patch('reg_tune_logger.extract_frame_id')
async def test_async_reader_skip_invalid_frame_ids(mock_extract):
    """Test async reading skips entries with invalid frame IDs."""
    temp_file = tempfile.NamedTemporaryFile(delete=False, mode='w', encoding='utf-8')
    temp_filename = temp_file.name

    try:
        test_lines = [
            "'100 DATA test1'",
            "'invalid_entry'",
            "'102 DATA test3'"
        ]

        temp_file.write('\n'.join(test_lines))
        temp_file.close()

        # Mock frame ID extraction - first call for frame_id0, then for each entry
        mock_extract.side_effect = [100, 100, None, 102]

        reader = RegTuneLogReaderAsync(temp_filename)
        results = []
        async for data in reader.read():
            results.append(data)

        assert len(results) == 2
        assert results[0] == b"100 DATA test1"
        assert results[1] == b"102 DATA test3"
    finally:
        if os.path.exists(temp_filename):
            os.unlink(temp_filename)


@pytest.mark.asyncio
async def test_async_reader_empty_file():
    """Test async reading an empty log file."""
    temp_file = tempfile.NamedTemporaryFile(delete=False, mode='w', encoding='utf-8')
    temp_filename = temp_file.name
    temp_file.close()

    try:
        reader = RegTuneLogReaderAsync(temp_filename)
        results = []
        async for data in reader.read():
            results.append(data)

        assert len(results) == 0
    finally:
        if os.path.exists(temp_filename):
            os.unlink(temp_filename)


# Helper to run async tests
def run_async_test(coro):
    """Helper function to run async tests."""
    loop = asyncio.new_event_loop()
    try:
        return loop.run_until_complete(coro)
    finally:
        loop.close()



if __name__ == '__main__':
    unittest.main()
