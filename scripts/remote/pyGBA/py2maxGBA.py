
class MaxBox:
    def __init__(self) -> None:
        self.box_dict: dict = {}

    @property
    def id(self) -> str:
        return self.box_dict["box"]["id"]

    @property
    def patching_rect_x(self) -> float:
        return self.box_dict["box"]["patching_rect"][0]

    @patching_rect_x.setter
    def patching_rect_x(self, value: float):
        self.box_dict["box"]["patching_rect"][0] = value

    @property
    def patching_rect_y(self) -> float:
        return self.box_dict["box"]["patching_rect"][1]

    @patching_rect_y.setter
    def patching_rect_y(self, value: float):
        self.box_dict["box"]["patching_rect"][1] = value

    @property
    def patching_rect_xy(self) -> list[float]:
        return self.box_dict["box"]["patching_rect"][:2]

    @patching_rect_xy.setter
    def patching_rect_xy(self, value: list[float]):
        self.box_dict["box"]["patching_rect"][0] = value[0]
        self.box_dict["box"]["patching_rect"][1] = value[1]

    @property
    def patching_rect_wh(self) -> list[float]:
        return self.box_dict["box"]["patching_rect"][2:]

    @patching_rect_wh.setter
    def patching_rect_wh(self, value: list[float]):
        self.box_dict["box"]["patching_rect"][2] = value[0]
        self.box_dict["box"]["patching_rect"][3] = value[1]

    @property
    def patching_rect(self) -> list[float]:
        return self.box_dict["box"]["patching_rect"]

    @patching_rect.setter
    def patching_rect(self, value: list[float]):
        self.box_dict["box"]["patching_rect"][:4] = value

    @property
    def dict(self) -> dict:
        return self.box_dict


class InletBox(MaxBox):
    def __init__(self, hint: str, comment: str, index: int) -> None:
        super().__init__()
        id = f"{self.__class__.__name__}-{index}"
        print(index, id)
        self.box_dict = {
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
                "patching_rect": [0.0, 0.0, 30.0, 30.0]
            }
        }

    @property
    def index(self) -> int:
        return self.box_dict["box"]["index"]


class OutletBox(MaxBox):
    def __init__(self, comment: str, index: int) -> None:
        super().__init__()
        id = f"{self.__class__.__name__}-{index}"
        self.box_dict = {
            "box": {
                "comment": comment,
                "id": id,
                "index": index,
                "maxclass": "outlet",
                "numinlets": 1,
                "numoutlets": 0,
                "patching_rect": [0.0, 0.0, 30.0, 30.0]
            }
        }

    @property
    def index(self) -> int:
        return self.box_dict["box"]["index"]


class PrependBox(MaxBox):
    Counter = 0

    def __init__(self, message: str) -> None:
        super().__init__()
        self.__class__.Counter += 1
        id = f"{self.__class__.__name__}-{self.__class__.Counter}"
        text = f"prepend {message}"
        self.box_dict = {
            "box": {
                "id": id,
                "maxclass": "newobj",
                "numinlets": 1,
                "numoutlets": 1,
                "outlettype": [""],
                "patching_rect": [0.0, 0.0, 80.0, 22.0],
                "text": text
            }
        }


class JsBox(MaxBox):
    Counter = 0

    def __init__(self, filename: str, args: list[str]) -> None:
        super().__init__()
        self.__class__.Counter += 1
        id = f"{self.__class__.__name__}-{self.Counter}"
        text = f"js {filename} {' '.join(args)}"
        color = [1.0, 0.588235294117647, 0.317647058823529, 1.0]
        self.box_dict = {
            "box": {
                "color": color,
                "id": id,
                "maxclass": "newobj",
                "numinlets": 1,
                "numoutlets": 2,
                "outlettype": ["", ""],
                "patching_rect": [0.0, 0.0, 100.0, 22.0],
                "saved_object_attributes": {
                    "filename": filename,
                    "parameter_enable": 0
                },
                "text": text,
                "textcolor": color
            }
        }


class MaxLine:
    def __init__(self, source_box: MaxBox, dest_box: MaxBox, src_outlet: int = 0, dst_inlet: int = 0) -> None:
        self.line_dict = {
            "patchline": {
                "source": [source_box.id, src_outlet],
                "destination": [dest_box.id, dst_inlet]
            }
        }

    @property
    def dict(self) -> dict:
        return self.line_dict


class IGridSpace:
    def compute_xy(self, X, Y) -> tuple[float, float]:
        raise NotImplemented()


class Patch:
    def __init__(self, gridspace: IGridSpace) -> None:
        self.boxes: list[MaxBox] = []
        self.lines: list[MaxLine] = []
        self.gridspace = gridspace
        self.X = 0
        self.Y = 0

    def set_XY(self, x: int, y: int) -> None:
        self.X = x
        self.Y = y

    @property
    def XY(self) -> tuple[int, int]:
        return self.X, self.Y

    @XY.setter
    def XY(self, value: tuple[int, int]) -> None:
        self.X, self.Y = value

    def add_box(self, box: MaxBox) -> None:
        box.patching_rect_xy = self.gridspace.compute_xy(self.X, self.Y)
        self.boxes.append(box)

    def add_line(self, line: MaxLine) -> None:
        self.lines.append(line)

    @property
    def dict(self) -> dict:
        return {
            "patcher": {
                "fileversion": 1,
                "appversion": {
                    "major": 8,
                    "minor": 6,
                    "revision": 0,
                    "architecture": "x64",
                    "modernui": 1
                },
                "classnamespace": "box",
                "rect": [50.0, 50.0, 900.0, 600.0],
                "bglocked": 0,
                "openinpresentation": 0,
                "default_fontsize": 12.0,
                "default_fontface": 0,
                "default_fontname": "Arial",
                "gridonopen": 1,
                "gridsize": [15.0, 15.0],
                "gridsnaponopen": 1,
                "objectsnaponopen": 1,
                "statusbarvisible": 2,
                "toolbarvisible": 1,
                "lefttoolbarpinned": 0,
                "toptoolbarpinned": 0,
                "righttoolbarpinned": 0,
                "bottomtoolbarpinned": 0,
                "toolbars_unpinned_last_save": 0,
                "tallnewobj": 0,
                "boxanimatetime": 200,
                "enablehscroll": 1,
                "enablevscroll": 1,
                "devicewidth": 0.0,
                "description": "",
                "digest": "",
                "tags": "",
                "style": "",
                "subpatcher_template": "",
                "assistshowspatchername": 0,
                "boxes": [box.dict for box in self.boxes],
                "lines": [line.dict for line in self.lines],
                "saved_attribute_attributes": {
                    "default_plcolor": {
                        "expression": ""
                    }
                }
            }
        }


class SubPatchBox(MaxBox):
    Counter = 0

    def __init__(self, name: str, maxpatch: Patch) -> None:
        super().__init__()
        self.__class__.Counter += 1
        self.id_ = f"{self.__class__.__name__}-{self.__class__.Counter}"
        self.name = name
        self.maxpatch = maxpatch

        text = f"p {self.name}"
        numinlets = sum(1 for box in self.maxpatch.boxes if isinstance(box, InletBox))
        numoutlets = sum(1 for box in self.maxpatch.boxes if isinstance(box, OutletBox))
        self.box_dict = {
            "box": {
                "maxclass": "newobj",
                "text": text,
                "numinlets": numinlets,
                "numoutlets": numoutlets,
                "id": self.id_,
                "patching_rect": [0.0, 0.0, 35.0, 20.0],
                "patcher": self.maxpatch.dict,
                "saved_object_attributes": {
                    "description": "",
                    "digest": "",
                    "globalpatchername": "",
                    "tags": ""
                },
                "saved_attribute_attributes": {
                    "default_plcolor": {
                        "expression": ""
                    }
                }
            }
        }

    @property
    def dict(self) -> dict:
        return self.box_dict


class DialBox(MaxBox):
    Counter = 0

    def __init__(self, name: str, shortname: str, n: int) -> None:
        super().__init__()
        self.__class__.Counter += 1
        self.id_ = f"{self.__class__.__name__}-{self.__class__.Counter}"
        self.box_dict = {
            "box": {
                "maxclass": "live.dial",
                "varname": name,
                "numinlets": 1,
                "numoutlets": 2,
                "appearance": 1,
                "parameter_enable": 1,
                "showname": 0,
                "outlettype": ["", "float"],
                "presentation": 1,
                "id": self.id_,
                "patching_rect": [0.0, 0.0, 25.0, 23.0],
                "saved_attribute_attributes": {
                    "valueof": {
                        "parameter_linknames": 1,
                        "parameter_longname": name,
                        "parameter_mmax": n-1,
                        "parameter_shortname": shortname,
                        "parameter_type": 1,
                        "parameter_unitstyle": 0
                    }
                }
            }
        }

class ToggleBox(MaxBox):
    Counter = 0

    def __init__(self, name: str, shortname: str) -> None:
        super().__init__()
        self.__class__.Counter += 1
        self.id_ = f"{self.__class__.__name__}-{self.__class__.Counter}"
        self.box_dict = {
			"box" : 			{
				"maxclass" : "toggle",
				"varname" : name,
				"presentation_rect" : [ 176.5, 61.0, 24.0, 24.0 ],
				"numinlets" : 1,
				"numoutlets" : 1,
				"parameter_enable" : 1,
				"outlettype" : [ "int" ],
				"id" : self.id_,
				"patching_rect" : [ 0.0, 0.0, 24.0, 24.0 ],
				"saved_attribute_attributes" : 				{
					"valueof" : 					{
						"parameter_enum" : [ "off", "on" ],
						"parameter_initial" : [ 0.0 ],
						"parameter_initial_enable" : 1,
						"parameter_linknames" : 1,
						"parameter_longname" : name,
						"parameter_mmax" : 1,
						"parameter_shortname" : shortname,
						"parameter_type" : 2
					}
				}
			}
		}

class NumberBox(MaxBox):
    Counter = 0

    def __init__(self, name: str, shortname: str, n: int) -> None:
        super().__init__()
        self.__class__.Counter += 1
        self.id_ = f"{self.__class__.__name__}-{self.__class__.Counter}"
        self.box_dict = {
			"box" : 			{
				"maxclass" : "number",
				"varname" : name,
				"numinlets" : 1,
				"minimum" : 0,
				"maximum" : n-1,
				"numoutlets" : 2,
				"parameter_enable" : 1,
				"outlettype" : [ "", "bang" ],
                "id": self.id_,
                "patching_rect" : [ 0.0, 0.0, 50.0, 20.0 ],
				"saved_attribute_attributes" : 				{
					"valueof" : 					{
						"parameter_longname" : name,
						"parameter_mmax" : n-1,
						"parameter_shortname" : shortname,
						"parameter_type" : 0
					}
				}
			}
		}

class MessageBox(MaxBox):
    Counter = 0

    def __init__(self, message: str) -> None:
        super().__init__()
        self.__class__.Counter += 1
        self.id_ = f"{self.__class__.__name__}-{self.__class__.Counter}"
        self.box_dict = {
			"box" : 			{
				"maxclass" : "message",
				"text" : message,
				"numinlets" : 2,
				"numoutlets" : 1,
				"outlettype" : [ "" ],
				"id" : self.id_,
				"patching_rect" : [ 0.0, 0.0, 89.0, 20.0 ]
			}

		}

class TriggerBox(MaxBox):
    Counter = 0

    types_map = {
        "b": "bang",
        "i": "integer",
        "f": "float",
        "s": "symbols",
        "bang": "bang",
        "integer": "integer",
        "float": "float",
        "symbols": "symbols",
    }

    def __init__(self, outlet: str) -> None:
        super().__init__()
        self.__class__.Counter += 1
        self.id_ = f"{self.__class__.__name__}-{self.__class__.Counter}"
        text = f"t {outlet}"
        outlettype = [TriggerBox.types_map[t] for t in outlet.split()]
        self.box_dict = {
			"box" : 			{
				"maxclass" : "newobj",
				"text" : text,
				"numinlets" : 1,
				"numoutlets" : len(outlettype),
				"outlettype" : outlettype,
				"id" : self.id_,
				"patching_rect" : [ 0.0, 0.0, 20.0, 20.0 ]
			}
		}
