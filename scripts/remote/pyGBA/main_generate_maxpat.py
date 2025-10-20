
import gba_sound

class MaxPatGenerator:
    GridSizeX = 90
    GridSizeY = 120
    Margin = 10

    @staticmethod
    def compute_coordinates_xy(i: int, j: int) -> tuple[float, float]:
        x = i * MaxPatGenerator.GridSizeX + MaxPatGenerator.Margin
        y = j * MaxPatGenerator.GridSizeY + MaxPatGenerator.Margin
        y += (i%2) * MaxPatGenerator.GridSizeY // 3
        return (float(x), float(y))
    
    @staticmethod
    def compute_coordinates_ij(x: float, y: float) -> tuple[int, int]:
        i = int((x - MaxPatGenerator.Margin) // MaxPatGenerator.GridSizeX)
        j = int((y - MaxPatGenerator.Margin) // MaxPatGenerator.GridSizeY)
        return (i, j)

    @staticmethod
    def compute_id(i: int, j: int) -> str:
        return f"obj-{i}-{j}"

    def __init__(self, regdata_cls: type[gba_sound.RegData]) -> None:
        self.regdata_cls = regdata_cls
        self.type_name = regdata_cls.__name__
        self.boxes: list[dict] = []
        self.lines: list[dict] = []

    def generate_inlet_box(self, hint: str, comment: str, index: int) -> dict:
        i, j = index-1, 0
        x, y = self.compute_coordinates_xy(i, j)
        id = self.compute_id(i, j)
        return {
            "box": {
                "annotation": hint,
                "comment": comment,
                "hint": hint,
                "id": id,
                "index": index,
                "maxclass": "inlet",
                "numinlets": 0,
                "numoutlets": 1,
                "outlettype": [""],
                "patching_rect": [x, y, 30.0, 30.0]
            }
        }

    def generate_outlet_box(self, comment: str, index: int, i: int, j: int) -> dict:
        x, y = self.compute_coordinates_xy(i, j)
        id = self.compute_id(i, j)
        return {
            "box" : {
                "comment" : comment,
                "id" : id,
                "index" : index,
                "maxclass" : "outlet",
                "numinlets" : 1,
                "numoutlets" : 0,
                "patching_rect" : [ x, y, 30.0, 30.0 ]
            }
        }

    def generate_prepend_box(self, message: str, i: int, j: int) -> dict:
        x, y = self.compute_coordinates_xy(i, j)
        id = self.compute_id(i, j)
        text = f"prepend {message}"
        width = float(MaxPatGenerator.GridSizeX - MaxPatGenerator.Margin)
        return {
            "box" : {
                "id" : id,
                "maxclass" : "newobj",
                "numinlets" : 1,
                "numoutlets" : 1,
                "outlettype" : [ "" ],
                "patching_rect" : [ x, y, width, 22.0 ],
                "text" : text
            }
        }

    def generate_line(self, source_id: str, source_outlet: int, dest_id: str, dest_inlet: int) -> dict:
        return {
            "patchline" : {
                "source" : [ source_id, source_outlet ],
                "destination" : [ dest_id, dest_inlet ]
            }
        }

    def generate_js_box(self, filename: str, args: list[str], i: int, j: int) -> dict:
        color = [1.0, 0.588235294117647, 0.317647058823529, 1.0]
        x, y = self.compute_coordinates_xy(i, j)
        id = self.compute_id(i, j)
        text = f"js {filename} {' '.join(args)}"
        width = MaxPatGenerator.GridSizeX - MaxPatGenerator.Margin
        return {
            "box" : 				{
                "color" : color,
                "id" : id,
                "maxclass" : "newobj",
                "numinlets" : 1,
                "numoutlets" : 2,
                "outlettype" : [ "", "" ],
                "patching_rect" : [ x, y, width, 22.0 ],
                "saved_object_attributes" : 					{
                    "filename" : filename,
                    "parameter_enable" : 0
                },
                "text" : text,
                "textcolor" : color
            }

        }
    
    def max_ij(self) -> tuple[int, int]:
        max_i = 0
        max_j = 0
        for box in self.boxes:
            x, y = box["box"]["patching_rect"][:2]
            i, j = self.compute_coordinates_ij(x, y)
            if i > max_i:
                max_i = i
            if j > max_j:
                max_j = j
        return (max_i, max_j)

    def generate_maxpat(self) -> dict:
        
        inlet1 = self.generate_inlet_box(hint="commands", comment="js script commands", index=1)
        self.boxes.append(inlet1)

        field_inlets = []
        field_prepends = []
        for n, field in enumerate(self.regdata_cls.empty_fields()):
            if isinstance(field, gba_sound.FieldUnused):
                continue
            ind = -n + len(self.regdata_cls.empty_fields()) + 1
            comment = f"{field.name} in [0, {(1 << field.size) - 1}]"
            field_inlet = self.generate_inlet_box(hint=field.name, comment=comment, index=ind)
            field_inlets.append(field_inlet)

            field_prepend = self.generate_prepend_box(message=field.name, i=ind-1, j=1)
            field_prepends.append(field_prepend)

            line = self.generate_line(field_inlet["box"]["id"], 0, field_prepend["box"]["id"], 0)
            self.lines.append(line)

        self.boxes.extend(field_inlets)
        self.boxes.extend(field_prepends)

        i_center = len(field_inlets) // 2

        set_value_prepend = self.generate_prepend_box(message="set_value", i=i_center, j=2)
        self.boxes.append(set_value_prepend)
        for field_prepend in field_prepends:
            line = self.generate_line(field_prepend["box"]["id"], 0, set_value_prepend["box"]["id"], 0)
            self.lines.append(line)

        js_box = self.generate_js_box(filename="gba_sound.js", args=[self.type_name], i=i_center, j=3)
        self.boxes.append(js_box)
        for box in [inlet1, set_value_prepend]:
            line = self.generate_line(box["box"]["id"], 0, js_box["box"]["id"], 0)
            self.lines.append(line)

        outlet1 = self.generate_outlet_box(comment="current register value", index=1, i=i_center, j=4)
        self.boxes.append(outlet1)
        line = self.generate_line(js_box["box"]["id"], 0, outlet1["box"]["id"], 0)
        self.lines.append(line)

        return {
            "patcher" : 	{
                "fileversion" : 1,
                "appversion" : 		{
                    "major" : 8,
                    "minor" : 6,
                    "revision" : 0,
                    "architecture" : "x64",
                    "modernui" : 1
                }
        ,
                "classnamespace" : "box",
                "rect" : [ 50.0, 50.0, 900.0, 600.0 ],
                "bglocked" : 0,
                "openinpresentation" : 0,
                "default_fontsize" : 12.0,
                "default_fontface" : 0,
                "default_fontname" : "Arial",
                "gridonopen" : 1,
                "gridsize" : [ 15.0, 15.0 ],
                "gridsnaponopen" : 1,
                "objectsnaponopen" : 1,
                "statusbarvisible" : 2,
                "toolbarvisible" : 1,
                "lefttoolbarpinned" : 0,
                "toptoolbarpinned" : 0,
                "righttoolbarpinned" : 0,
                "bottomtoolbarpinned" : 0,
                "toolbars_unpinned_last_save" : 0,
                "tallnewobj" : 0,
                "boxanimatetime" : 200,
                "enablehscroll" : 1,
                "enablevscroll" : 1,
                "devicewidth" : 0.0,
                "description" : "",
                "digest" : "",
                "tags" : "",
                "style" : "",
                "subpatcher_template" : "",
                "assistshowspatchername" : 0,
                "boxes" : self.boxes,
                "lines" : self.lines,
                "saved_attribute_attributes" : 		{
                    "default_plcolor" : 			{
                        "expression" : ""
                    }
                }
            }
        }


def main() -> None:

    sound_types = [
        gba_sound.SweepControl,
        gba_sound.TonePattern,
        gba_sound.ToneFrequency,
        gba_sound.NoiseLenEnvelope,
        gba_sound.NoiseFrequency,
        gba_sound.LeftRightVolume,
        gba_sound.SoundBias,
        gba_sound.SoundEnable,
        gba_sound.SoundMix,
    ]
    for sound_type in sound_types:
        type_name = sound_type.__name__
        print(f"Generating maxpat for {type_name}...")
        generator = MaxPatGenerator(sound_type)
        maxpat_dict = generator.generate_maxpat()
        import json
        with open(f"..\\maxGBA\\mg0.gba_{type_name}.maxpat", "w", encoding="utf-8") as f:
            json.dump(maxpat_dict, f, indent=4)

if __name__ == "__main__":
    main()