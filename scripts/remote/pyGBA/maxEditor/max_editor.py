import enum
import json
from typing import Callable, Any


class FuncType(enum.IntEnum):
    FILTER = 0
    EDIT = 1


class MaxJSONEncoder:
    """Encoder that produces Max/MSP-style JSON formatting."""

    def __init__(self, indent: str = "\t", line_endings: str = "\r\n"):
        self.indent_str = indent
        self.line_endings = line_endings

    def encode(self, obj: Any) -> str:
        """Encode a Python object to Max-style JSON string."""
        return self._encode_value(obj, 0)

    def _encode_value(self, obj: Any, level: int) -> str:
        """Encode a value at a specific indentation level."""
        if obj is None:
            return "null"
        elif isinstance(obj, bool):
            return "true" if obj else "false"
        elif isinstance(obj, int):
            return str(obj)
        elif isinstance(obj, float):
            # up to 15 decimal places, remove trailing zeros except for one zero after decimal point
            # example: 1.230000000000000 -> 1.23
            # example: 1.000000000000000 -> 1.0,
            s15 = f"{obj:.15f}"
            s = s15.rstrip('0')
            if s.endswith("."):
                s += "0"
            return s
        elif isinstance(obj, str):
            return json.dumps(obj)  # Use standard JSON string escaping
        elif isinstance(obj, list):
            return self._encode_array(obj, level)
        elif isinstance(obj, dict):
            return self._encode_object(obj, level)
        else:
            raise TypeError(f"Object of type {type(obj).__name__} is not JSON serializable")

    def _encode_array(self, arr: list, level: int) -> str:
        """Encode an array with Max-style formatting."""
        if not arr:
            return "[ ]"

        # Check if it's a simple array (all primitives)
        is_simple = all(isinstance(item, (int, float, str, bool, type(None))) for item in arr)

        if is_simple:
            # Simple array on one line
            items = [self._encode_value(item, level) for item in arr]
            return "[ " + ", ".join(items) + " ]"
        else:
            # Complex array with line breaks
            indent = self.indent_str * (level + 1)
            items = []
            for item in arr:
                encoded = self._encode_value(item, level + 1)
                items.append(encoded)

            result = "[ " + indent + items[0] + self.line_endings
            for item in items[1:]:
                result += ", " + indent + item + self.line_endings
            result += " ]"
            return result

    def _encode_object(self, obj: dict, level: int) -> str:
        """Encode an object with Max-style formatting."""
        if not obj:
            return "{" + 2 * self.line_endings + self.indent_str * level + "}"

        indent = self.indent_str * (level + 1)
        lines = []
        items = list(obj.items())

        for i, (key, value) in enumerate(items):
            encoded_value = self._encode_value(value, level + 1)
            key_str = json.dumps(key)
            is_last = (i == len(items) - 1)

            # Max format uses: "key" : \t\t\t{...} for objects (tabs match indentation level)
            # For other values: "key" : value
            if isinstance(value, dict):
                # Number of tabs after : should match indentation level (level + 1)
                tabs_after_colon = self.indent_str * (level + 1)
                line = f'{indent}{key_str} : {tabs_after_colon}{encoded_value}'
            else:
                line = f'{indent}{key_str} : {encoded_value}'

            # For objects (not arrays) that are not last, add }\r\n,\r\n pattern
            if isinstance(value, dict) and not is_last:
                lines.append(line + self.line_endings + ",")
            else:
                lines.append(line + ("," if not is_last else ""))

        result = "{" + self.line_endings
        for line in lines:
            result += line + self.line_endings

        # Add empty line before closing brace if last item is an object
        if items and isinstance(items[-1][1], dict):
            result += self.line_endings

        result += self.indent_str * level + "}"
        return result


class MaxEditor:
    def __init__(self, maxfile: str):
        self.maxfile = maxfile
        with open(maxfile, 'rb') as f:
            data = f.read()
        # crop header until first "{" and footer after last "}"
        json_start = data.find(b'{')
        json_end = data.rfind(b'}') + 1
        self.header = data[:json_start]
        self.footer = data[json_end:]

        self.content = json.loads( data[json_start:json_end].decode('utf-8'))
            # parse_float=lambda s: s + " as float",
            # parse_int=lambda s: s + " as int")

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

    def format_content(self) -> str:
        encoder = MaxJSONEncoder(indent="\t")
        return encoder.encode(self.content)

    def save(self, output_file: str):
        with open(output_file, 'wb') as f:
            f.write(self.header)
            json_content = self.format_content()
            f.write(json_content.encode('utf-8'))
            f.write(self.footer)



def resize_toggle_rect(maxfile: str):
    def resize_rect(box, new_w:int, new_h: int):
        if not 'presentation_rect' in box:
            return False
        new_w = int(new_w)
        new_h = int(new_h)

        rect = box.get('presentation_rect')
        x, y, w, h = rect
        x += (w - new_w) // 2
        y += (h - new_h) // 2
        box['presentation_rect'] = [float(val) for val in [x, y, new_w, new_h]]
        return True


    is_toggle = lambda box: box.get('maxclass') == 'toggle'
    is_autogenerated = lambda box: box.get('id', '').startswith('AUTOGENERATED')
    has_presentation_rect = lambda box: 'presentation_rect' in box
    resize_rect_to_20x20 = lambda box: resize_rect(box, 20, 20)

    editor = MaxEditor(maxfile)
    editor.filter(is_toggle)\
      .filter(is_autogenerated)\
      .filter(has_presentation_rect)\
      .edit(resize_rect_to_20x20)\
      .apply()
    editor.save(maxfile)

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
        #format_file(maxfile)
        resize_toggle_rect(maxfile)

def format_file(maxfile: str, output_file: str = None):
    if output_file is None:
        output_file = maxfile

    editor = MaxEditor(maxfile)
    editor.save(output_file)


if __name__ == "__main__":
    main()
