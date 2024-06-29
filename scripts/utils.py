import subprocess


def format_rust_file(filename):
    subprocess.run(['rustfmt', filename])
