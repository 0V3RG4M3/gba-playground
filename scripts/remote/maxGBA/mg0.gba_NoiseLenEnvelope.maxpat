{
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
        "rect": [
            50.0,
            50.0,
            900.0,
            600.0
        ],
        "bglocked": 0,
        "openinpresentation": 0,
        "default_fontsize": 12.0,
        "default_fontface": 0,
        "default_fontname": "Arial",
        "gridonopen": 1,
        "gridsize": [
            15.0,
            15.0
        ],
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
        "boxes": [
            {
                "box": {
                    "annotation": "commands",
                    "comment": "js script commands",
                    "hint": "commands",
                    "id": "obj-0-0",
                    "index": 1,
                    "maxclass": "inlet",
                    "numinlets": 0,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "patching_rect": [
                        10.0,
                        10.0,
                        30.0,
                        30.0
                    ]
                }
            },
            {
                "box": {
                    "annotation": "length",
                    "comment": "length in [0, 63]",
                    "hint": "length",
                    "id": "obj-5-0",
                    "index": 6,
                    "maxclass": "inlet",
                    "numinlets": 0,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "patching_rect": [
                        460.0,
                        50.0,
                        30.0,
                        30.0
                    ]
                }
            },
            {
                "box": {
                    "annotation": "step_time",
                    "comment": "step_time in [0, 7]",
                    "hint": "step_time",
                    "id": "obj-3-0",
                    "index": 4,
                    "maxclass": "inlet",
                    "numinlets": 0,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "patching_rect": [
                        280.0,
                        50.0,
                        30.0,
                        30.0
                    ]
                }
            },
            {
                "box": {
                    "annotation": "step_increasing",
                    "comment": "step_increasing in [0, 1]",
                    "hint": "step_increasing",
                    "id": "obj-2-0",
                    "index": 3,
                    "maxclass": "inlet",
                    "numinlets": 0,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "patching_rect": [
                        190.0,
                        10.0,
                        30.0,
                        30.0
                    ]
                }
            },
            {
                "box": {
                    "annotation": "volume",
                    "comment": "volume in [0, 15]",
                    "hint": "volume",
                    "id": "obj-1-0",
                    "index": 2,
                    "maxclass": "inlet",
                    "numinlets": 0,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "patching_rect": [
                        100.0,
                        50.0,
                        30.0,
                        30.0
                    ]
                }
            },
            {
                "box": {
                    "id": "obj-5-1",
                    "maxclass": "newobj",
                    "numinlets": 1,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "patching_rect": [
                        460.0,
                        170.0,
                        80.0,
                        22.0
                    ],
                    "text": "prepend length"
                }
            },
            {
                "box": {
                    "id": "obj-3-1",
                    "maxclass": "newobj",
                    "numinlets": 1,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "patching_rect": [
                        280.0,
                        170.0,
                        80.0,
                        22.0
                    ],
                    "text": "prepend step_time"
                }
            },
            {
                "box": {
                    "id": "obj-2-1",
                    "maxclass": "newobj",
                    "numinlets": 1,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "patching_rect": [
                        190.0,
                        130.0,
                        80.0,
                        22.0
                    ],
                    "text": "prepend step_increasing"
                }
            },
            {
                "box": {
                    "id": "obj-1-1",
                    "maxclass": "newobj",
                    "numinlets": 1,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "patching_rect": [
                        100.0,
                        170.0,
                        80.0,
                        22.0
                    ],
                    "text": "prepend volume"
                }
            },
            {
                "box": {
                    "id": "obj-2-2",
                    "maxclass": "newobj",
                    "numinlets": 1,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "patching_rect": [
                        190.0,
                        250.0,
                        80.0,
                        22.0
                    ],
                    "text": "prepend set_value"
                }
            },
            {
                "box": {
                    "color": [
                        1.0,
                        0.588235294117647,
                        0.317647058823529,
                        1.0
                    ],
                    "id": "obj-2-3",
                    "maxclass": "newobj",
                    "numinlets": 1,
                    "numoutlets": 2,
                    "outlettype": [
                        "",
                        ""
                    ],
                    "patching_rect": [
                        190.0,
                        370.0,
                        80,
                        22.0
                    ],
                    "saved_object_attributes": {
                        "filename": "gba_sound.js",
                        "parameter_enable": 0
                    },
                    "text": "js gba_sound.js NoiseLenEnvelope",
                    "textcolor": [
                        1.0,
                        0.588235294117647,
                        0.317647058823529,
                        1.0
                    ]
                }
            },
            {
                "box": {
                    "comment": "current register value",
                    "id": "obj-2-4",
                    "index": 1,
                    "maxclass": "outlet",
                    "numinlets": 1,
                    "numoutlets": 0,
                    "patching_rect": [
                        190.0,
                        490.0,
                        30.0,
                        30.0
                    ]
                }
            }
        ],
        "lines": [
            {
                "patchline": {
                    "source": [
                        "obj-5-0",
                        0
                    ],
                    "destination": [
                        "obj-5-1",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "obj-3-0",
                        0
                    ],
                    "destination": [
                        "obj-3-1",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "obj-2-0",
                        0
                    ],
                    "destination": [
                        "obj-2-1",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "obj-1-0",
                        0
                    ],
                    "destination": [
                        "obj-1-1",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "obj-5-1",
                        0
                    ],
                    "destination": [
                        "obj-2-2",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "obj-3-1",
                        0
                    ],
                    "destination": [
                        "obj-2-2",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "obj-2-1",
                        0
                    ],
                    "destination": [
                        "obj-2-2",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "obj-1-1",
                        0
                    ],
                    "destination": [
                        "obj-2-2",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "obj-0-0",
                        0
                    ],
                    "destination": [
                        "obj-2-3",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "obj-2-2",
                        0
                    ],
                    "destination": [
                        "obj-2-3",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "obj-2-3",
                        0
                    ],
                    "destination": [
                        "obj-2-4",
                        0
                    ]
                }
            }
        ],
        "saved_attribute_attributes": {
            "default_plcolor": {
                "expression": ""
            }
        }
    }
}