"""
Test script to verify MaxEditor properly formats Max/MSP JSON files.
"""

import json
import os
from max_editor import MaxEditor


def test_format_content():
    """Test that format_content produces Max-style JSON."""
    # Path to the minimal example
    minimal_example = os.path.join(
        os.path.dirname(__file__),
        "..", "json_dump", "minimal_example.json"
    )

    if not os.path.exists(minimal_example):
        print("⚠ minimal_example.json not found, skipping test")
        return

    # Load the example file
    editor = MaxEditor(minimal_example)

    # Format the content
    formatted = editor.format_content()

    # Verify formatting characteristics
    assert '\t' in formatted, "Output should use tabs"
    assert ' : ' in formatted, "Output should have spaces around colons"
    assert '[ ' in formatted, "Arrays should have space after bracket"

    # Verify it's valid JSON
    parsed = json.loads(formatted)
    assert parsed == editor.content, "Parsed JSON should match original content"

    print("✅ format_content test passed!")
    print("\nSample output (first 500 chars):")
    print(formatted[:500])


if __name__ == "__main__":
    test_format_content()

