# Testing Configuration for Remote Module

This directory contains unit tests for the remote module components.

## Directory Structure

```
tests/
├── __init__.py                 # Main tests package
└── remote/                     # Tests for remote/ module
    ├── __init__.py            # Remote tests package
    └── pyGBA/                 # Tests for remote/pyGBA/ module
        ├── __init__.py        # pyGBA tests package
        └── test_simple_stream.py  # Tests for simple_stream.py
```

## Running Tests

### From the project root directory:

```bash
# Run all tests
python -m pytest tests/

# Run specific test module
python -m pytest tests/remote/pyGBA/test_simple_stream.py

# Run with verbose output
python -m pytest tests/remote/pyGBA/test_simple_stream.py -v

# Run using uv (if available)
uv run pytest tests/remote/pyGBA/test_simple_stream.py -v
```

### From the tests directory:

```bash
cd tests
python -m pytest remote/pyGBA/test_simple_stream.py -v
```

## Test Coverage

### simple_stream.py Tests
- **ISimpleStream**: Interface implementation tests
- **UDPSimpleStream**: UDP socket communication tests
- **TCPSimpleStream**: TCP socket communication tests  
- **NullSimpleStream**: Mock stream functionality tests
- **FileSimpleStream**: File-based stream tests with mocked dependencies

## Adding New Tests

When adding tests for new remote module files:

1. Create test files in the corresponding directory structure under `tests/`
2. Use the naming convention `test_<module_name>.py`
3. Import the modules being tested using the path resolution pattern shown in existing tests
4. Follow the existing test class structure and naming conventions

## Notes

- Tests use dynamic path resolution to import modules from the remote directory
- Mock objects are used to isolate dependencies (e.g., RegTuneLogReader)
- Socket tests include proper cleanup and timeout handling
- All tests are designed to be run independently and in any order
