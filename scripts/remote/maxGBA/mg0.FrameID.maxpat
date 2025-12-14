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
            2072.0,
            -68.0,
            882.0,
            591.0
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
                    "id": "obj-21",
                    "maxclass": "message",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "patching_rect": [
                        383.0,
                        66.0,
                        29.5,
                        22.0
                    ],
                    "text": "60"
                }
            },
            {
                "box": {
                    "id": "obj-13",
                    "maxclass": "message",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "patching_rect": [
                        187.0,
                        199.0,
                        50.0,
                        22.0
                    ],
                    "text": "128."
                }
            },
            {
                "box": {
                    "id": "obj-32",
                    "maxclass": "comment",
                    "numinlets": 1,
                    "numoutlets": 0,
                    "patching_rect": [
                        142.0,
                        132.0,
                        150.0,
                        20.0
                    ],
                    "text": "Get bpm [beat/min]"
                }
            },
            {
                "box": {
                    "id": "obj-30",
                    "maxclass": "comment",
                    "numinlets": 1,
                    "numoutlets": 0,
                    "patching_rect": [
                        164.0,
                        248.0,
                        129.0,
                        20.0
                    ],
                    "text": "Convert to seconds [s]"
                }
            },
            {
                "box": {
                    "id": "obj-26",
                    "maxclass": "newobj",
                    "numinlets": 3,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "patching_rect": [
                        53.0,
                        247.0,
                        115.0,
                        22.0
                    ],
                    "text": "expr $f1 * ($f2 / $f3)"
                }
            },
            {
                "box": {
                    "color": [
                        0.466666666666667,
                        0.694117647058824,
                        0.815686274509804,
                        1.0
                    ],
                    "id": "obj-3",
                    "maxclass": "newobj",
                    "numinlets": 1,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "patching_rect": [
                        115.0,
                        157.0,
                        234.0,
                        22.0
                    ],
                    "text": "mg0.live_property_getter live_set @tempo",
                    "textcolor": [
                        0.466666666666667,
                        0.694117647058824,
                        0.815686274509804,
                        1.0
                    ]
                }
            },
            {
                "box": {
                    "id": "obj-12",
                    "linecount": 2,
                    "maxclass": "comment",
                    "numinlets": 1,
                    "numoutlets": 0,
                    "patching_rect": [
                        320.0,
                        523.0,
                        78.0,
                        33.0
                    ],
                    "text": "Returns bang"
                }
            },
            {
                "box": {
                    "id": "obj-11",
                    "linecount": 2,
                    "maxclass": "comment",
                    "numinlets": 1,
                    "numoutlets": 0,
                    "patching_rect": [
                        153.0,
                        523.0,
                        78.0,
                        33.0
                    ],
                    "text": "Returns FrameID"
                }
            },
            {
                "box": {
                    "annotation": "",
                    "comment": "bang",
                    "hint": "bangs on new FrameID",
                    "id": "obj-8",
                    "index": 2,
                    "maxclass": "outlet",
                    "numinlets": 1,
                    "numoutlets": 0,
                    "patching_rect": [
                        280.0,
                        524.0,
                        30.0,
                        30.0
                    ],
                    "varname": "BangOut"
                }
            },
            {
                "box": {
                    "id": "obj-7",
                    "maxclass": "newobj",
                    "numinlets": 1,
                    "numoutlets": 2,
                    "outlettype": [
                        "int",
                        "bang"
                    ],
                    "patching_rect": [
                        261.0,
                        452.0,
                        29.5,
                        22.0
                    ],
                    "text": "t i b"
                }
            },
            {
                "box": {
                    "id": "obj-59",
                    "maxclass": "newobj",
                    "numinlets": 1,
                    "numoutlets": 1,
                    "outlettype": [
                        "bang"
                    ],
                    "patching_rect": [
                        410.0,
                        17.0,
                        58.0,
                        22.0
                    ],
                    "text": "loadbang"
                }
            },
            {
                "box": {
                    "color": [
                        0.466666666666667,
                        0.694117647058824,
                        0.815686274509804,
                        1.0
                    ],
                    "id": "obj-56",
                    "maxclass": "newobj",
                    "numinlets": 1,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "patching_rect": [
                        410.0,
                        118.0,
                        254.0,
                        22.0
                    ],
                    "text": "mg0.live_property_getter live_set @is_playing",
                    "textcolor": [
                        0.466666666666667,
                        0.694117647058824,
                        0.815686274509804,
                        1.0
                    ]
                }
            },
            {
                "box": {
                    "id": "obj-36",
                    "maxclass": "comment",
                    "numinlets": 1,
                    "numoutlets": 0,
                    "patching_rect": [
                        488.0,
                        18.0,
                        180.0,
                        20.0
                    ],
                    "text": "Observe is_playing state {0, 1}"
                }
            },
            {
                "box": {
                    "id": "obj-28",
                    "maxclass": "newobj",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        "int"
                    ],
                    "patching_rect": [
                        410.0,
                        236.0,
                        33.0,
                        22.0
                    ],
                    "text": "== 0"
                }
            },
            {
                "box": {
                    "id": "obj-20",
                    "maxclass": "message",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "patching_rect": [
                        311.0,
                        452.0,
                        50.0,
                        22.0
                    ],
                    "text": "55"
                }
            },
            {
                "box": {
                    "color": [
                        0.466666666666667,
                        0.694117647058824,
                        0.815686274509804,
                        1.0
                    ],
                    "id": "obj-18",
                    "maxclass": "newobj",
                    "numinlets": 0,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "patching_rect": [
                        433.0,
                        166.0,
                        270.0,
                        22.0
                    ],
                    "text": "mg0.live_property_observer live_set @is_playing",
                    "textcolor": [
                        0.466666666666667,
                        0.694117647058824,
                        0.815686274509804,
                        1.0
                    ]
                }
            },
            {
                "box": {
                    "id": "obj-16",
                    "linecount": 3,
                    "maxclass": "comment",
                    "numinlets": 1,
                    "numoutlets": 0,
                    "patching_rect": [
                        521.0,
                        297.0,
                        171.0,
                        47.0
                    ],
                    "text": "Output at 60 Hz. \n - 240 bars/min in 4:4 => 1 Hz\n - multiplied by 60 => 60Hz"
                }
            },
            {
                "box": {
                    "id": "obj-10",
                    "maxclass": "toggle",
                    "numinlets": 1,
                    "numoutlets": 1,
                    "outlettype": [
                        "int"
                    ],
                    "parameter_enable": 0,
                    "patching_rect": [
                        410.0,
                        264.0,
                        24.0,
                        24.0
                    ]
                }
            },
            {
                "box": {
                    "id": "obj-5",
                    "maxclass": "newobj",
                    "numinlets": 4,
                    "numoutlets": 1,
                    "outlettype": [
                        "int"
                    ],
                    "patching_rect": [
                        410.0,
                        297.0,
                        92.0,
                        22.0
                    ],
                    "text": "tempo 240 1 60"
                }
            },
            {
                "box": {
                    "id": "obj-4",
                    "maxclass": "message",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "patching_rect": [
                        175.0,
                        452.0,
                        73.0,
                        22.0
                    ],
                    "text": "691199"
                }
            },
            {
                "box": {
                    "color": [
                        0.466666666666667,
                        0.694117647058824,
                        0.815686274509804,
                        1.0
                    ],
                    "id": "obj-2",
                    "maxclass": "newobj",
                    "numinlets": 0,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "patching_rect": [
                        54.0,
                        95.0,
                        316.0,
                        22.0
                    ],
                    "text": "mg0.live_property_observer live_set @current_song_time",
                    "textcolor": [
                        0.466666666666667,
                        0.694117647058824,
                        0.815686274509804,
                        1.0
                    ]
                }
            },
            {
                "box": {
                    "comment": "Frame ID",
                    "hint": "returns FrameID",
                    "id": "obj-1",
                    "index": 1,
                    "maxclass": "outlet",
                    "numinlets": 1,
                    "numoutlets": 0,
                    "patching_rect": [
                        241.0,
                        524.0,
                        30.0,
                        30.0
                    ],
                    "varname": "FrameID"
                }
            },
            {
                "box": {
                    "id": "obj-15",
                    "linecount": 2,
                    "maxclass": "comment",
                    "numinlets": 1,
                    "numoutlets": 0,
                    "patching_rect": [
                        180.0,
                        292.0,
                        181.0,
                        33.0
                    ],
                    "text": "Convert to frame ID at 60 Hz\n- delayed by half a frame"
                }
            },
            {
                "box": {
                    "fontname": "Arial Bold",
                    "fontsize": 10.0,
                    "id": "obj-14",
                    "maxclass": "newobj",
                    "numinlets": 1,
                    "numoutlets": 3,
                    "outlettype": [
                        "",
                        "int",
                        "int"
                    ],
                    "patching_rect": [
                        53.0,
                        316.0,
                        50.0,
                        20.0
                    ],
                    "text": "change 0"
                }
            },
            {
                "box": {
                    "id": "obj-6",
                    "maxclass": "newobj",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "patching_rect": [
                        53.0,
                        291.0,
                        121.0,
                        22.0
                    ],
                    "text": "expr int($f1*$f2 - 0.5)"
                }
            },
            {
                "box": {
                    "id": "obj-29",
                    "maxclass": "comment",
                    "numinlets": 1,
                    "numoutlets": 0,
                    "patching_rect": [
                        54.0,
                        70.0,
                        150.0,
                        20.0
                    ],
                    "text": "Observe time value [beat]"
                }
            },
            {
                "box": {
                    "fontname": "Arial Bold",
                    "fontsize": 10.0,
                    "id": "obj-39",
                    "maxclass": "newobj",
                    "numinlets": 1,
                    "numoutlets": 3,
                    "outlettype": [
                        "",
                        "int",
                        "int"
                    ],
                    "patching_rect": [
                        53.0,
                        181.0,
                        52.0,
                        20.0
                    ],
                    "text": "change 0."
                }
            }
        ],
        "lines": [
            {
                "patchline": {
                    "destination": [
                        "obj-5",
                        0
                    ],
                    "source": [
                        "obj-10",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-4",
                        1
                    ],
                    "order": 1,
                    "source": [
                        "obj-14",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-7",
                        0
                    ],
                    "order": 0,
                    "source": [
                        "obj-14",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-28",
                        0
                    ],
                    "order": 0,
                    "source": [
                        "obj-18",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-3",
                        0
                    ],
                    "midpoints": [
                        442.5,
                        191.5,
                        365.0,
                        191.5,
                        365.0,
                        152.5,
                        124.5,
                        152.5
                    ],
                    "order": 1,
                    "source": [
                        "obj-18",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-39",
                        0
                    ],
                    "source": [
                        "obj-2",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-26",
                        1
                    ],
                    "midpoints": [
                        392.5,
                        229.0,
                        110.5,
                        229.0
                    ],
                    "order": 2,
                    "source": [
                        "obj-21",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-5",
                        3
                    ],
                    "midpoints": [
                        392.5,
                        222.0,
                        492.5,
                        222.0
                    ],
                    "order": 0,
                    "source": [
                        "obj-21",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-6",
                        1
                    ],
                    "midpoints": [
                        392.5,
                        279.0,
                        165.0,
                        279.0,
                        165.0,
                        285.0,
                        164.5,
                        285.0
                    ],
                    "order": 1,
                    "source": [
                        "obj-21",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-6",
                        0
                    ],
                    "source": [
                        "obj-26",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-10",
                        0
                    ],
                    "source": [
                        "obj-28",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-13",
                        1
                    ],
                    "midpoints": [
                        124.5,
                        192.5,
                        227.5,
                        192.5
                    ],
                    "order": 0,
                    "source": [
                        "obj-3",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-26",
                        2
                    ],
                    "midpoints": [
                        124.5,
                        239.0,
                        158.5,
                        239.0
                    ],
                    "order": 1,
                    "source": [
                        "obj-3",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-26",
                        0
                    ],
                    "source": [
                        "obj-39",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-20",
                        1
                    ],
                    "order": 0,
                    "source": [
                        "obj-5",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-7",
                        0
                    ],
                    "order": 1,
                    "source": [
                        "obj-5",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-28",
                        0
                    ],
                    "source": [
                        "obj-56",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-21",
                        0
                    ],
                    "midpoints": [
                        419.5,
                        51.0,
                        392.5,
                        51.0
                    ],
                    "order": 1,
                    "source": [
                        "obj-59",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-3",
                        0
                    ],
                    "midpoints": [
                        419.5,
                        104.5,
                        380.0,
                        104.5,
                        380.0,
                        152.5,
                        124.5,
                        152.5
                    ],
                    "order": 2,
                    "source": [
                        "obj-59",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-56",
                        0
                    ],
                    "order": 0,
                    "source": [
                        "obj-59",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-14",
                        0
                    ],
                    "source": [
                        "obj-6",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-1",
                        0
                    ],
                    "source": [
                        "obj-7",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "destination": [
                        "obj-8",
                        0
                    ],
                    "source": [
                        "obj-7",
                        1
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
