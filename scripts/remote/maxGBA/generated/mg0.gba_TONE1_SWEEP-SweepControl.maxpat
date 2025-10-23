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
                    "maxclass": "message",
                    "text": "TONE1_SWEEP",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-1",
                    "patching_rect": [
                        10.0,
                        42.0,
                        89.0,
                        20.0
                    ]
                }
            },
            {
                "box": {
                    "maxclass": "live.dial",
                    "varname": "TONE1_SWEEP-sweep_num",
                    "numinlets": 1,
                    "numoutlets": 2,
                    "appearance": 1,
                    "parameter_enable": 1,
                    "showname": 0,
                    "outlettype": [
                        "",
                        "float"
                    ],
                    "presentation": 1,
                    "id": "DialBox-1",
                    "patching_rect": [
                        266.0,
                        74.0,
                        25.0,
                        23.0
                    ],
                    "saved_attribute_attributes": {
                        "valueof": {
                            "parameter_linknames": 1,
                            "parameter_longname": "TONE1_SWEEP-sweep_num",
                            "parameter_mmax": 7,
                            "parameter_shortname": "sweep_num",
                            "parameter_type": 1,
                            "parameter_unitstyle": 0
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "sweep_num",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-2",
                    "patching_rect": [
                        10.0,
                        74.0,
                        89.0,
                        20.0
                    ]
                }
            },
            {
                "box": {
                    "maxclass": "toggle",
                    "varname": "TONE1_SWEEP-sweep_increasing",
                    "presentation_rect": [
                        176.5,
                        61.0,
                        24.0,
                        24.0
                    ],
                    "numinlets": 1,
                    "numoutlets": 1,
                    "parameter_enable": 1,
                    "outlettype": [
                        "int"
                    ],
                    "id": "ToggleBox-1",
                    "patching_rect": [
                        234.0,
                        106.0,
                        24.0,
                        24.0
                    ],
                    "saved_attribute_attributes": {
                        "valueof": {
                            "parameter_enum": [
                                "off",
                                "on"
                            ],
                            "parameter_initial": [
                                0.0
                            ],
                            "parameter_initial_enable": 1,
                            "parameter_linknames": 1,
                            "parameter_longname": "TONE1_SWEEP-sweep_increasing",
                            "parameter_mmax": 1,
                            "parameter_shortname": "sweep_increasing",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "sweep_increasing",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-3",
                    "patching_rect": [
                        10.0,
                        106.0,
                        89.0,
                        20.0
                    ]
                }
            },
            {
                "box": {
                    "maxclass": "live.dial",
                    "varname": "TONE1_SWEEP-sweep_time",
                    "numinlets": 1,
                    "numoutlets": 2,
                    "appearance": 1,
                    "parameter_enable": 1,
                    "showname": 0,
                    "outlettype": [
                        "",
                        "float"
                    ],
                    "presentation": 1,
                    "id": "DialBox-2",
                    "patching_rect": [
                        202.0,
                        138.0,
                        25.0,
                        23.0
                    ],
                    "saved_attribute_attributes": {
                        "valueof": {
                            "parameter_linknames": 1,
                            "parameter_longname": "TONE1_SWEEP-sweep_time",
                            "parameter_mmax": 7,
                            "parameter_shortname": "sweep_time",
                            "parameter_type": 1,
                            "parameter_unitstyle": 0
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "sweep_time",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-4",
                    "patching_rect": [
                        10.0,
                        138.0,
                        89.0,
                        20.0
                    ]
                }
            },
            {
                "box": {
                    "maxclass": "newobj",
                    "text": "p SweepControl",
                    "numinlets": 4,
                    "numoutlets": 1,
                    "id": "SubPatchBox-1",
                    "patching_rect": [
                        170.0,
                        202.0,
                        35.0,
                        20.0
                    ],
                    "patcher": {
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
                                        "id": "InletBox-1",
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
                                        "annotation": "sweep_num",
                                        "comment": "sweep_num in [0, 7]",
                                        "hint": "sweep_num",
                                        "id": "InletBox-4",
                                        "index": 4,
                                        "maxclass": "inlet",
                                        "numinlets": 0,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            250.0,
                                            90.0,
                                            30.0,
                                            30.0
                                        ]
                                    }
                                },
                                {
                                    "box": {
                                        "id": "PrependBox-1",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            250.0,
                                            170.0,
                                            80.0,
                                            22.0
                                        ],
                                        "text": "prepend sweep_num"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "sweep_increasing",
                                        "comment": "sweep_increasing in [0, 1]",
                                        "hint": "sweep_increasing",
                                        "id": "InletBox-3",
                                        "index": 3,
                                        "maxclass": "inlet",
                                        "numinlets": 0,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            170.0,
                                            63.33333333333333,
                                            30.0,
                                            30.0
                                        ]
                                    }
                                },
                                {
                                    "box": {
                                        "id": "PrependBox-2",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            170.0,
                                            143.33333333333331,
                                            80.0,
                                            22.0
                                        ],
                                        "text": "prepend sweep_increasing"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "sweep_time",
                                        "comment": "sweep_time in [0, 7]",
                                        "hint": "sweep_time",
                                        "id": "InletBox-2",
                                        "index": 2,
                                        "maxclass": "inlet",
                                        "numinlets": 0,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            90.0,
                                            36.666666666666664,
                                            30.0,
                                            30.0
                                        ]
                                    }
                                },
                                {
                                    "box": {
                                        "id": "PrependBox-3",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            90.0,
                                            116.66666666666666,
                                            80.0,
                                            22.0
                                        ],
                                        "text": "prepend sweep_time"
                                    }
                                },
                                {
                                    "box": {
                                        "id": "PrependBox-4",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            210.0,
                                            236.66666666666669,
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
                                        "id": "JsBox-1",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 2,
                                        "outlettype": [
                                            "",
                                            ""
                                        ],
                                        "patching_rect": [
                                            210.0,
                                            316.6666666666667,
                                            100.0,
                                            22.0
                                        ],
                                        "saved_object_attributes": {
                                            "filename": "gba_sound.js",
                                            "parameter_enable": 0
                                        },
                                        "text": "js gba_sound.js SweepControl",
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
                                        "id": "OutletBox-1",
                                        "index": 1,
                                        "maxclass": "outlet",
                                        "numinlets": 1,
                                        "numoutlets": 0,
                                        "patching_rect": [
                                            210.0,
                                            396.66666666666663,
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
                                            "InletBox-4",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-1",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "InletBox-3",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-2",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "InletBox-2",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-3",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-1",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-4",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-2",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-4",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-3",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-4",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "InletBox-1",
                                            0
                                        ],
                                        "destination": [
                                            "JsBox-1",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-4",
                                            0
                                        ],
                                        "destination": [
                                            "JsBox-1",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "JsBox-1",
                                            0
                                        ],
                                        "destination": [
                                            "OutletBox-1",
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
                    },
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
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "compile",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-5",
                    "patching_rect": [
                        10.0,
                        202.0,
                        89.0,
                        20.0
                    ]
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
                    "id": "JsBox-2",
                    "maxclass": "newobj",
                    "numinlets": 1,
                    "numoutlets": 2,
                    "outlettype": [
                        "",
                        ""
                    ],
                    "patching_rect": [
                        170.0,
                        266.0,
                        100.0,
                        22.0
                    ],
                    "saved_object_attributes": {
                        "filename": "gba_mmio.js",
                        "parameter_enable": 0
                    },
                    "text": "js gba_mmio.js TONE1_SWEEP",
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
                    "maxclass": "newobj",
                    "text": "t b",
                    "numinlets": 1,
                    "numoutlets": 1,
                    "outlettype": [
                        "bang"
                    ],
                    "id": "TriggerBox-1",
                    "patching_rect": [
                        170.0,
                        10.0,
                        20.0,
                        20.0
                    ]
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "outputvalue",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-6",
                    "patching_rect": [
                        202.0,
                        10.0,
                        89.0,
                        20.0
                    ]
                }
            }
        ],
        "lines": [
            {
                "patchline": {
                    "source": [
                        "DialBox-1",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-1",
                        3
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-1",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-1",
                        2
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "DialBox-2",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-1",
                        1
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "SubPatchBox-1",
                        0
                    ],
                    "destination": [
                        "JsBox-2",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-5",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-1",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-5",
                        0
                    ],
                    "destination": [
                        "JsBox-2",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "TriggerBox-1",
                        0
                    ],
                    "destination": [
                        "DialBox-1",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "TriggerBox-1",
                        0
                    ],
                    "destination": [
                        "DialBox-2",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-6",
                        0
                    ],
                    "destination": [
                        "ToggleBox-1",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "TriggerBox-1",
                        0
                    ],
                    "destination": [
                        "MessageBox-5",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "TriggerBox-1",
                        0
                    ],
                    "destination": [
                        "MessageBox-6",
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