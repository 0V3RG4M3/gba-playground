"""
Simple test for MaxEditor class to ensure .amxd files can be loaded.
"""
import sys
import unittest
from pathlib import Path

# Add the parent directory to path to import max_editor
sys.path.insert(0, str(Path(__file__).parent))

from max_editor import MaxEditor


class TestMaxEditor(unittest.TestCase):
    """Test cases for MaxEditor class."""

    @classmethod
    def setUpClass(cls):
        """Set up test fixtures."""
        cls.test_file = Path(__file__).parent / "example.amxd"
        cls.output_file = Path(__file__).parent / "test_output.amxd"

    def setUp(self):
        """Set up each test."""
        # Ensure test file exists
        self.assertTrue(self.test_file.exists(), f"Test file not found: {self.test_file}")

        # Clean up any leftover output files
        if self.output_file.exists():
            self.output_file.unlink()

    def tearDown(self):
        """Clean up after each test."""
        # Remove output file if it exists
        if self.output_file.exists():
            self.output_file.unlink()

    def test_load_amxd_file(self):
        """Test that MaxEditor can load an .amxd file."""
        # Load the .amxd file
        MaxEditor(str(self.test_file))


    def test_save_amxd_file(self):
        """Test that MaxEditor can save an .amxd file."""
        # Load the .amxd file
        editor = MaxEditor(str(self.test_file))

        # Save to a new file
        editor.save(str(self.output_file))

        # Verify the output file exists
        self.assertTrue(self.output_file.exists(), "Output file should exist")

        # Load the output file to verify it's valid
        editor2 = MaxEditor(str(self.output_file))

        # Verify content is the same
        self.assertEqual(editor.content, editor2.content, "Content should be identical after save/load")

        import json
        self.assertEqual(json.dumps(editor.content), json.dumps(editor2.content), "JSON content should match after save/load")

        print(f"\n   ✓ Output file created: {self.output_file}")
        print(f"   ✓ Content verified identical after save/load")

    def test_file_not_found(self):
        """Test that MaxEditor raises an error for non-existent files."""
        non_existent_file = Path(__file__).parent / "non_existent.amxd"

        with self.assertRaises(FileNotFoundError):
            MaxEditor(str(non_existent_file))

    def test_edit_field(self):
        editor = MaxEditor(str(self.test_file))

        new_id = b"TEST_ID_123"
        # ensure the new_id does not already exist in the file
        with open(self.test_file, 'rb') as f:
            content = f.read()
            self.assertNotIn(new_id, content, f"Test ID '{new_id}' should not already exist in the test file")

        # Modify all id of toggles boxes
        is_toggle = lambda box: box.get('maxclass') == 'toggle'
        has_id = lambda box: 'id' in box
        edit_id = lambda box: box.update({'id': new_id.decode('utf-8')})

        editor\
            .filter(is_toggle)\
            .filter(has_id)\
            .edit(edit_id)\
            .apply()

        editor.save(str(self.output_file))

        # Verify some lines where the id was changed
        with open(str(self.test_file), 'rb') as f:
            content_in = f.read().splitlines()
        with open(str(self.output_file), 'rb') as f:
            content_out = f.read().splitlines()

        new_id_found = False
        for line in content_out:
            if new_id in line:
                new_id_found = True
                break
        self.assertTrue(new_id_found, f"Modified ID '{new_id}' should be found in the output file")

        for line_in, line_out in zip(content_in, content_out):
            if new_id in line_out:
                self.assertNotEqual(line_in, line_out, "Lines with modified ID should differ between input and output files")
            else:
                self.assertEqual(line_in, line_out, "Lines without modified ID should be identical between input and output files")


if __name__ == "__main__":
    # Run the tests with verbosity
    unittest.main(verbosity=2)

