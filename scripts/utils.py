import subprocess
import os

def format_rust_file(filename):
    subprocess.run(['rustfmt', filename])


def find_all_by_extension(folder, ext):
    folder = os.path.normpath(folder)
    file_list = []
    for parent_path, folders, files in os.walk(os.path.normpath(folder)):
        file_list += [os.path.join(parent_path, file) for file in files if file.endswith(ext)]

    file_list = [os.path.normpath(file) for file in file_list]
    return file_list
