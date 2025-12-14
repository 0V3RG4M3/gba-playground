import enum
import json
from typing import Callable


class FuncType(enum.IntEnum):
    FILTER = 0
    EDIT = 1

class MaxEditor:


    def __init__(self, maxfile: str):
        self.maxfile = maxfile
        with open(maxfile, 'r', encoding='utf-8') as f:
            data = f.read()
        # crop header until first "{" and footer after last "}"
        json_start = data.find('{')
        json_end = data.rfind('}') + 1
        self.header = data[:json_start]
        self.footer = data[json_end:]

        self.content = json.loads(data[json_start:json_end])

        self.functions: list[tuple[FuncType, Callable]] = []

    def filter(self, filter_func):
        self.functions.append((FuncType.FILTER, filter_func))
        return self

    def edit(self, modify_func):
        self.functions.append((FuncType.EDIT, modify_func))
        return self

    def apply(self):
        boxes = [elem["box"] for elem in self.content["patcher"]["boxes"]]
        for box in boxes:
            for func_type, func in self.functions:
                if func_type == FuncType.FILTER:
                    if not func(box):
                        break
                    continue
                elif func_type == FuncType.EDIT:
                    func(box)
                    continue

        self.functions = []
        return self


    def save(self, output_file: str):
        with open(output_file, 'w', encoding='utf-8') as f:
            f.write(self.header)
            json.dump(self.content, f, indent=4)
            f.write(self.footer)

def resize_rect_to_20x20(box):
    if not 'presentation_rect' in box:
        return False
    rect = box.get('presentation_rect')
    x, y, w, h = rect
    x += (w - 20) // 2
    y += (h - 20) // 2
    box['presentation_rect'] = [x, y, 20, 20]
    return True

def main():

    target_folder = '../../maxGBA'
    # recuresively list all .amxd and maxpat files in the target_folder
    import os
    maxfiles = []
    for root, dirs, files in os.walk(target_folder):
        for file in files:
            if file.endswith('.amxd') or file.endswith('.maxpat'):
                maxfiles.append(os.path.join(root, file))

    print(f"Found {len(maxfiles)} max files to process.")
    for maxfile in maxfiles:
        editor = MaxEditor(maxfile)
        editor.save(maxfile)

if __name__ == "__main__":
    main()