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
                    "text": "NOISE_FREQ",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-42",
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
                    "varname": "NOISE_FREQ-rate",
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
                    "id": "DialBox-14",
                    "patching_rect": [
                        330.0,
                        74.0,
                        25.0,
                        23.0
                    ],
                    "saved_attribute_attributes": {
                        "valueof": {
                            "parameter_linknames": 1,
                            "parameter_longname": "NOISE_FREQ-rate",
                            "parameter_mmax": 7,
                            "parameter_shortname": "rate",
                            "parameter_type": 1,
                            "parameter_unitstyle": 0
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "rate",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-43",
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
                    "varname": "NOISE_FREQ-counter7",
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
                    "id": "ToggleBox-9",
                    "patching_rect": [
                        298.0,
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
                            "parameter_longname": "NOISE_FREQ-counter7",
                            "parameter_mmax": 1,
                            "parameter_shortname": "counter7",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "counter7",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-44",
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
                    "varname": "NOISE_FREQ-shift",
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
                    "id": "DialBox-15",
                    "patching_rect": [
                        266.0,
                        138.0,
                        25.0,
                        23.0
                    ],
                    "saved_attribute_attributes": {
                        "valueof": {
                            "parameter_linknames": 1,
                            "parameter_longname": "NOISE_FREQ-shift",
                            "parameter_mmax": 15,
                            "parameter_shortname": "shift",
                            "parameter_type": 1,
                            "parameter_unitstyle": 0
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "shift",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-45",
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
                    "maxclass": "toggle",
                    "varname": "NOISE_FREQ-stop_when_expired",
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
                    "id": "ToggleBox-10",
                    "patching_rect": [
                        234.0,
                        170.0,
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
                            "parameter_longname": "NOISE_FREQ-stop_when_expired",
                            "parameter_mmax": 1,
                            "parameter_shortname": "stop_when_expired",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "stop_when_expired",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-46",
                    "patching_rect": [
                        10.0,
                        170.0,
                        89.0,
                        20.0
                    ]
                }
            },
            {
                "box": {
                    "maxclass": "toggle",
                    "varname": "NOISE_FREQ-enabled",
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
                    "id": "ToggleBox-11",
                    "patching_rect": [
                        202.0,
                        202.0,
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
                            "parameter_longname": "NOISE_FREQ-enabled",
                            "parameter_mmax": 1,
                            "parameter_shortname": "enabled",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "enabled",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-47",
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
                    "maxclass": "newobj",
                    "text": "p NoiseFrequency",
                    "numinlets": 6,
                    "numoutlets": 1,
                    "id": "SubPatchBox-7",
                    "patching_rect": [
                        170.0,
                        266.0,
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
                                        "annotation": "rate",
                                        "comment": "rate in [0, 7]",
                                        "hint": "rate",
                                        "id": "InletBox-6",
                                        "index": 6,
                                        "maxclass": "inlet",
                                        "numinlets": 0,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            410.0,
                                            143.33333333333334,
                                            30.0,
                                            30.0
                                        ]
                                    }
                                },
                                {
                                    "box": {
                                        "id": "PrependBox-30",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            410.0,
                                            223.33333333333337,
                                            80.0,
                                            22.0
                                        ],
                                        "text": "prepend rate"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "counter7",
                                        "comment": "counter7 in [0, 1]",
                                        "hint": "counter7",
                                        "id": "InletBox-5",
                                        "index": 5,
                                        "maxclass": "inlet",
                                        "numinlets": 0,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            330.0,
                                            116.66666666666666,
                                            30.0,
                                            30.0
                                        ]
                                    }
                                },
                                {
                                    "box": {
                                        "id": "PrependBox-31",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            330.0,
                                            196.66666666666663,
                                            80.0,
                                            22.0
                                        ],
                                        "text": "prepend counter7"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "shift",
                                        "comment": "shift in [0, 15]",
                                        "hint": "shift",
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
                                        "id": "PrependBox-32",
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
                                        "text": "prepend shift"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "stop_when_expired",
                                        "comment": "stop_when_expired in [0, 1]",
                                        "hint": "stop_when_expired",
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
                                        "id": "PrependBox-33",
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
                                        "text": "prepend stop_when_expired"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "enabled",
                                        "comment": "enabled in [0, 1]",
                                        "hint": "enabled",
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
                                        "id": "PrependBox-34",
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
                                        "text": "prepend enabled"
                                    }
                                },
                                {
                                    "box": {
                                        "id": "PrependBox-35",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            290.0,
                                            263.33333333333337,
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
                                        "id": "JsBox-13",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 2,
                                        "outlettype": [
                                            "",
                                            ""
                                        ],
                                        "patching_rect": [
                                            290.0,
                                            343.33333333333337,
                                            100.0,
                                            22.0
                                        ],
                                        "saved_object_attributes": {
                                            "filename": "gba_sound.js",
                                            "parameter_enable": 0
                                        },
                                        "text": "js gba_sound.js NoiseFrequency",
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
                                            290.0,
                                            423.33333333333337,
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
                                            "InletBox-6",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-30",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "InletBox-5",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-31",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "InletBox-4",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-32",
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
                                            "PrependBox-33",
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
                                            "PrependBox-34",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-30",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-35",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-31",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-35",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-32",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-35",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-33",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-35",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-34",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-35",
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
                                            "JsBox-13",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-35",
                                            0
                                        ],
                                        "destination": [
                                            "JsBox-13",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "JsBox-13",
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
                    "id": "MessageBox-48",
                    "patching_rect": [
                        10.0,
                        266.0,
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
                    "id": "JsBox-14",
                    "maxclass": "newobj",
                    "numinlets": 1,
                    "numoutlets": 2,
                    "outlettype": [
                        "",
                        ""
                    ],
                    "patching_rect": [
                        170.0,
                        330.0,
                        100.0,
                        22.0
                    ],
                    "saved_object_attributes": {
                        "filename": "gba_mmio.js",
                        "parameter_enable": 0
                    },
                    "text": "js gba_mmio.js NOISE_FREQ",
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
                    "id": "TriggerBox-7",
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
                    "id": "MessageBox-49",
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
                        "DialBox-14",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-7",
                        5
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-9",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-7",
                        4
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "DialBox-15",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-7",
                        3
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-10",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-7",
                        2
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-11",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-7",
                        1
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "SubPatchBox-7",
                        0
                    ],
                    "destination": [
                        "JsBox-14",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-48",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-7",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-48",
                        0
                    ],
                    "destination": [
                        "JsBox-14",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "TriggerBox-7",
                        0
                    ],
                    "destination": [
                        "DialBox-14",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "TriggerBox-7",
                        0
                    ],
                    "destination": [
                        "DialBox-15",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-49",
                        0
                    ],
                    "destination": [
                        "ToggleBox-9",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-49",
                        0
                    ],
                    "destination": [
                        "ToggleBox-10",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-49",
                        0
                    ],
                    "destination": [
                        "ToggleBox-11",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "TriggerBox-7",
                        0
                    ],
                    "destination": [
                        "MessageBox-48",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "TriggerBox-7",
                        0
                    ],
                    "destination": [
                        "MessageBox-49",
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