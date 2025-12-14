import json
import os.path

import gba_mmio
import gba_sound
from py2maxGBA import DialBox, MessageBox, NumberBox, JsBox, PrependBox, InletBox, OutletBox, SubPatchBox, ToggleBox, TriggerBox
from py2maxGBA import IGridSpace, MaxLine, Patch


class GridSpace(IGridSpace):
    def __init__(self, x0: float, y0: float, width: float, height: float) -> None:
        self.x0 = x0
        self.y0 = y0
        self.width = width
        self.height = height

    def compute_xy(self, X, Y) -> tuple[float, float]:
        x = X * self.width + self.x0
        y = Y * self.height + self.y0
        return float(round(x)), float(round(y))

    def compute_wh(self, W, H) -> tuple[float, float]:
        w = W * self.width
        h = H * self.height
        return float(round(w)), float(round(h))


class SkewedGridSpace(IGridSpace):
    def __init__(self, x0: float, y0: float, width: float, height: float) -> None:
        self.x0 = x0
        self.y0 = y0
        self.width = width
        self.height = height

    def compute_xy(self, X, Y) -> tuple[float, float]:
        x = X * self.width + self.x0
        y = (Y + X / 3) * self.height + self.y0
        return float(round(x)), float(round(y))

    def compute_wh(self, W, H) -> tuple[float, float]:
        w = W * self.width
        h = H * self.height
        return float(round(w)), float(round(h))


def generate_sound_type_subpatch(regdata_cls: type[gba_sound.RegData]) -> SubPatchBox:
    fields: list[gba_sound.Field] = [field for field in regdata_cls.empty_fields() if
                                     not isinstance(field, gba_sound.FieldUnused)]
    type_name = regdata_cls.__name__

    gridspace = SkewedGridSpace(x0=10.0, y0=10.0, width=80.0, height=80.0)

    maxpatch = Patch(gridspace)

    inlet1 = InletBox(hint="commands", comment="js script commands", index=1)
    maxpatch.add_box(inlet1)

    field_inlets = []
    field_prepends = []
    N = len(fields)
    for n, field in enumerate(fields):
        ind = -n + N + 1
        comment = f"{field.name} in [0, {(1 << field.size) - 1}]"
        field_inlet = InletBox(hint=field.name, comment=comment, index=ind)
        X, Y = ind - 1, 0
        maxpatch.set_XY(X, Y)
        maxpatch.add_box(field_inlet)
        field_inlets.append(field_inlet)

        maxpatch.Y += 1
        field_prepend = PrependBox(message=field.name)
        maxpatch.add_box(field_prepend)
        field_prepends.append(field_prepend)

        maxpatch.add_line(MaxLine(field_inlet, field_prepend))

    X_center = len(field_inlets) / 2 + 1

    maxpatch.X = X_center
    maxpatch.Y += 1
    set_value_prepend = PrependBox(message="set_value")
    maxpatch.add_box(set_value_prepend)
    for field_prepend in field_prepends:
        maxpatch.add_line(MaxLine(field_prepend, set_value_prepend))

    maxpatch.Y += 1
    js_box = JsBox(filename="gba_sound.js", args=[type_name])
    maxpatch.add_box(js_box)
    for box in [inlet1, set_value_prepend]:
        maxpatch.add_line(MaxLine(box, js_box))

    maxpatch.Y += 1
    outlet1 = OutletBox(comment="current register value", index=1)
    maxpatch.add_box(outlet1)
    maxpatch.add_line(MaxLine(js_box, outlet1))

    return SubPatchBox(type_name, maxpatch)


def generate_mmio_patch(reg: gba_mmio.Register, subpatch: SubPatchBox) -> Patch:
    gridspace = GridSpace(x0=10.0, y0=10.0, width=25.0, height=25.0)
    patch = Patch(gridspace)

    sound_type = reg.DATA_TYPE
    fields: list[gba_sound.Field] = [field for field in sound_type.empty_fields() if
                                     not isinstance(field, gba_sound.FieldUnused)]

    # Add field controllers and their labels on the left
    label_width = 5
    X0, Y0 = label_width, 2
    N = len(fields)
    toggle_boxes = []
    non_toggle_boxes = []
    controller_width = 0
    for k, field in enumerate(fields):
        patch.XY = X0, Y0 + k
        range_size = 2 ** field.size
        name = f"{reg.NAME}-{field.name}"
        if range_size < 2:
            raise ValueError(f"Field {field.name} has invalid range size {range_size}")
        elif range_size == 2:
            box = ToggleBox(name=name, shortname=field.name)
            toggle_boxes.append(box)
            controller_width = max(controller_width, 1)
        elif range_size <= 128:
            box = DialBox(name=name, shortname=field.name, n=range_size)
            non_toggle_boxes.append(box)
            controller_width = max(controller_width, 1)
        else:
            box = NumberBox(name=name, shortname=field.name, n=range_size)
            non_toggle_boxes.append(box)
            controller_width = max(controller_width, 3)
        box.presentation = True
        patch.add_box(box)
        patch.add_line(MaxLine(box, subpatch, 0, N - k))

        patch.X = X0 - label_width
        label_box = MessageBox(message=f"{field.name}")
        label_box.presentation = True
        patch.add_box(label_box, W=label_width)

        patch.XY = X0 + N, Y0 + k

    y_backup = patch.Y

    # Add title box
    patch.XY = 0, 1
    title_width = label_width + controller_width
    title_label_box = MessageBox(message=f"{reg.NAME}")
    title_label_box.presentation = True
    patch.add_box(title_label_box, W=title_width)

    # Add subpatch box, reading the controllers outputs
    patch.Y += y_backup + 1
    patch.add_box(subpatch)

    patch.X += label_width
    compile_box = MessageBox(message=f"compile")
    patch.add_box(compile_box)

    patch.X -= label_width
    patch.Y += 2
    js_box = JsBox(filename="gba_mmio.js", args=[reg.NAME])
    patch.add_box(js_box)

    patch.add_line(MaxLine(subpatch, js_box))
    patch.add_line(MaxLine(compile_box, subpatch))
    patch.add_line(MaxLine(compile_box, js_box))

    patch.XY = label_width, 0
    tbbox = TriggerBox("b")
    patch.add_box(tbbox)

    for box in non_toggle_boxes:
        patch.add_line(MaxLine(tbbox, box))

    patch.X += 1
    outputvalue_box = MessageBox(message="outputvalue")
    patch.add_box(outputvalue_box)
    for box in toggle_boxes:
        patch.add_line(MaxLine(outputvalue_box, box))

    patch.add_line(MaxLine(tbbox, compile_box))
    patch.add_line(MaxLine(tbbox, outputvalue_box))

    return patch


def generate_all_sound_maxpat() -> None:
    HERE = os.path.abspath(__file__)

    for register in gba_mmio.registers:
        register_name = register.NAME
        sound_type = register.DATA_TYPE
        type_name = sound_type.__name__
        print(f"Generating maxpat for {type_name}...")
        subpatch: SubPatchBox = generate_sound_type_subpatch(sound_type)
        patch: Patch = generate_mmio_patch(register, subpatch)

        target_file = os.path.normpath(os.path.join(HERE,f"..\\..\\maxGBA\\generated\\mg0.gba_{register_name}-{type_name}.maxpat"))
        with open(target_file, "w", encoding="utf-8") as f:
            json.dump(patch.dict, f, indent=4)

        print(json.dumps(patch.dict, indent=4))


if __name__ == "__main__":
    generate_all_sound_maxpat()

