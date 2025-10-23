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
                    "text": "LEFT_RIGHT_VOLUME",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-50",
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
                    "varname": "LEFT_RIGHT_VOLUME-right_volume",
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
                    "id": "DialBox-16",
                    "patching_rect": [
                        490.0,
                        74.0,
                        25.0,
                        23.0
                    ],
                    "saved_attribute_attributes": {
                        "valueof": {
                            "parameter_linknames": 1,
                            "parameter_longname": "LEFT_RIGHT_VOLUME-right_volume",
                            "parameter_mmax": 7,
                            "parameter_shortname": "right_volume",
                            "parameter_type": 1,
                            "parameter_unitstyle": 0
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "right_volume",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-51",
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
                    "maxclass": "live.dial",
                    "varname": "LEFT_RIGHT_VOLUME-left_volume",
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
                    "id": "DialBox-17",
                    "patching_rect": [
                        458.0,
                        106.0,
                        25.0,
                        23.0
                    ],
                    "saved_attribute_attributes": {
                        "valueof": {
                            "parameter_linknames": 1,
                            "parameter_longname": "LEFT_RIGHT_VOLUME-left_volume",
                            "parameter_mmax": 7,
                            "parameter_shortname": "left_volume",
                            "parameter_type": 1,
                            "parameter_unitstyle": 0
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "left_volume",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-52",
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
                    "maxclass": "toggle",
                    "varname": "LEFT_RIGHT_VOLUME-tone1_right",
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
                    "id": "ToggleBox-12",
                    "patching_rect": [
                        426.0,
                        138.0,
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
                            "parameter_longname": "LEFT_RIGHT_VOLUME-tone1_right",
                            "parameter_mmax": 1,
                            "parameter_shortname": "tone1_right",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "tone1_right",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-53",
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
                    "varname": "LEFT_RIGHT_VOLUME-tone2_right",
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
                    "id": "ToggleBox-13",
                    "patching_rect": [
                        394.0,
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
                            "parameter_longname": "LEFT_RIGHT_VOLUME-tone2_right",
                            "parameter_mmax": 1,
                            "parameter_shortname": "tone2_right",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "tone2_right",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-54",
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
                    "varname": "LEFT_RIGHT_VOLUME-wave_right",
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
                    "id": "ToggleBox-14",
                    "patching_rect": [
                        362.0,
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
                            "parameter_longname": "LEFT_RIGHT_VOLUME-wave_right",
                            "parameter_mmax": 1,
                            "parameter_shortname": "wave_right",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "wave_right",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-55",
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
                    "maxclass": "toggle",
                    "varname": "LEFT_RIGHT_VOLUME-noise_right",
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
                    "id": "ToggleBox-15",
                    "patching_rect": [
                        330.0,
                        234.0,
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
                            "parameter_longname": "LEFT_RIGHT_VOLUME-noise_right",
                            "parameter_mmax": 1,
                            "parameter_shortname": "noise_right",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "noise_right",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-56",
                    "patching_rect": [
                        10.0,
                        234.0,
                        89.0,
                        20.0
                    ]
                }
            },
            {
                "box": {
                    "maxclass": "toggle",
                    "varname": "LEFT_RIGHT_VOLUME-tone1_left",
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
                    "id": "ToggleBox-16",
                    "patching_rect": [
                        298.0,
                        266.0,
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
                            "parameter_longname": "LEFT_RIGHT_VOLUME-tone1_left",
                            "parameter_mmax": 1,
                            "parameter_shortname": "tone1_left",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "tone1_left",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-57",
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
                    "maxclass": "toggle",
                    "varname": "LEFT_RIGHT_VOLUME-tone2_left",
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
                    "id": "ToggleBox-17",
                    "patching_rect": [
                        266.0,
                        298.0,
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
                            "parameter_longname": "LEFT_RIGHT_VOLUME-tone2_left",
                            "parameter_mmax": 1,
                            "parameter_shortname": "tone2_left",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "tone2_left",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-58",
                    "patching_rect": [
                        10.0,
                        298.0,
                        89.0,
                        20.0
                    ]
                }
            },
            {
                "box": {
                    "maxclass": "toggle",
                    "varname": "LEFT_RIGHT_VOLUME-wave_left",
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
                    "id": "ToggleBox-18",
                    "patching_rect": [
                        234.0,
                        330.0,
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
                            "parameter_longname": "LEFT_RIGHT_VOLUME-wave_left",
                            "parameter_mmax": 1,
                            "parameter_shortname": "wave_left",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "wave_left",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-59",
                    "patching_rect": [
                        10.0,
                        330.0,
                        89.0,
                        20.0
                    ]
                }
            },
            {
                "box": {
                    "maxclass": "toggle",
                    "varname": "LEFT_RIGHT_VOLUME-noise_left",
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
                    "id": "ToggleBox-19",
                    "patching_rect": [
                        202.0,
                        362.0,
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
                            "parameter_longname": "LEFT_RIGHT_VOLUME-noise_left",
                            "parameter_mmax": 1,
                            "parameter_shortname": "noise_left",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "noise_left",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-60",
                    "patching_rect": [
                        10.0,
                        362.0,
                        89.0,
                        20.0
                    ]
                }
            },
            {
                "box": {
                    "maxclass": "newobj",
                    "text": "p LeftRightVolume",
                    "numinlets": 11,
                    "numoutlets": 1,
                    "id": "SubPatchBox-8",
                    "patching_rect": [
                        170.0,
                        426.0,
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
                                        "annotation": "right_volume",
                                        "comment": "right_volume in [0, 7]",
                                        "hint": "right_volume",
                                        "id": "InletBox-11",
                                        "index": 11,
                                        "maxclass": "inlet",
                                        "numinlets": 0,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            810.0,
                                            276.6666666666667,
                                            30.0,
                                            30.0
                                        ]
                                    }
                                },
                                {
                                    "box": {
                                        "id": "PrependBox-36",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            810.0,
                                            356.66666666666674,
                                            80.0,
                                            22.0
                                        ],
                                        "text": "prepend right_volume"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "left_volume",
                                        "comment": "left_volume in [0, 7]",
                                        "hint": "left_volume",
                                        "id": "InletBox-10",
                                        "index": 10,
                                        "maxclass": "inlet",
                                        "numinlets": 0,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            730.0,
                                            250.0,
                                            30.0,
                                            30.0
                                        ]
                                    }
                                },
                                {
                                    "box": {
                                        "id": "PrependBox-37",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            730.0,
                                            330.0,
                                            80.0,
                                            22.0
                                        ],
                                        "text": "prepend left_volume"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "tone1_right",
                                        "comment": "tone1_right in [0, 1]",
                                        "hint": "tone1_right",
                                        "id": "InletBox-9",
                                        "index": 9,
                                        "maxclass": "inlet",
                                        "numinlets": 0,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            650.0,
                                            223.33333333333331,
                                            30.0,
                                            30.0
                                        ]
                                    }
                                },
                                {
                                    "box": {
                                        "id": "PrependBox-38",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            650.0,
                                            303.3333333333333,
                                            80.0,
                                            22.0
                                        ],
                                        "text": "prepend tone1_right"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "tone2_right",
                                        "comment": "tone2_right in [0, 1]",
                                        "hint": "tone2_right",
                                        "id": "InletBox-8",
                                        "index": 8,
                                        "maxclass": "inlet",
                                        "numinlets": 0,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            570.0,
                                            196.66666666666669,
                                            30.0,
                                            30.0
                                        ]
                                    }
                                },
                                {
                                    "box": {
                                        "id": "PrependBox-39",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            570.0,
                                            276.6666666666667,
                                            80.0,
                                            22.0
                                        ],
                                        "text": "prepend tone2_right"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "wave_right",
                                        "comment": "wave_right in [0, 1]",
                                        "hint": "wave_right",
                                        "id": "InletBox-7",
                                        "index": 7,
                                        "maxclass": "inlet",
                                        "numinlets": 0,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            490.0,
                                            170.0,
                                            30.0,
                                            30.0
                                        ]
                                    }
                                },
                                {
                                    "box": {
                                        "id": "PrependBox-40",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            490.0,
                                            250.0,
                                            80.0,
                                            22.0
                                        ],
                                        "text": "prepend wave_right"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "noise_right",
                                        "comment": "noise_right in [0, 1]",
                                        "hint": "noise_right",
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
                                        "id": "PrependBox-41",
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
                                        "text": "prepend noise_right"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "tone1_left",
                                        "comment": "tone1_left in [0, 1]",
                                        "hint": "tone1_left",
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
                                        "id": "PrependBox-42",
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
                                        "text": "prepend tone1_left"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "tone2_left",
                                        "comment": "tone2_left in [0, 1]",
                                        "hint": "tone2_left",
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
                                        "id": "PrependBox-43",
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
                                        "text": "prepend tone2_left"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "wave_left",
                                        "comment": "wave_left in [0, 1]",
                                        "hint": "wave_left",
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
                                        "id": "PrependBox-44",
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
                                        "text": "prepend wave_left"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "noise_left",
                                        "comment": "noise_left in [0, 1]",
                                        "hint": "noise_left",
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
                                        "id": "PrependBox-45",
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
                                        "text": "prepend noise_left"
                                    }
                                },
                                {
                                    "box": {
                                        "id": "PrependBox-46",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            490.0,
                                            330.0,
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
                                        "id": "JsBox-15",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 2,
                                        "outlettype": [
                                            "",
                                            ""
                                        ],
                                        "patching_rect": [
                                            490.0,
                                            410.0,
                                            100.0,
                                            22.0
                                        ],
                                        "saved_object_attributes": {
                                            "filename": "gba_sound.js",
                                            "parameter_enable": 0
                                        },
                                        "text": "js gba_sound.js LeftRightVolume",
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
                                            490.0,
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
                                            "InletBox-11",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-36",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "InletBox-10",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-37",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "InletBox-9",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-38",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "InletBox-8",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-39",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "InletBox-7",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-40",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "InletBox-6",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-41",
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
                                            "PrependBox-42",
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
                                            "PrependBox-43",
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
                                            "PrependBox-44",
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
                                            "PrependBox-45",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-36",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-46",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-37",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-46",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-38",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-46",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-39",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-46",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-40",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-46",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-41",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-46",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-42",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-46",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-43",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-46",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-44",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-46",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-45",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-46",
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
                                            "JsBox-15",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-46",
                                            0
                                        ],
                                        "destination": [
                                            "JsBox-15",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "JsBox-15",
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
                    "id": "MessageBox-61",
                    "patching_rect": [
                        10.0,
                        426.0,
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
                    "id": "JsBox-16",
                    "maxclass": "newobj",
                    "numinlets": 1,
                    "numoutlets": 2,
                    "outlettype": [
                        "",
                        ""
                    ],
                    "patching_rect": [
                        170.0,
                        490.0,
                        100.0,
                        22.0
                    ],
                    "saved_object_attributes": {
                        "filename": "gba_mmio.js",
                        "parameter_enable": 0
                    },
                    "text": "js gba_mmio.js LEFT_RIGHT_VOLUME",
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
                    "id": "TriggerBox-8",
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
                    "id": "MessageBox-62",
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
                        "DialBox-16",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-8",
                        10
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "DialBox-17",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-8",
                        9
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-12",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-8",
                        8
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-13",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-8",
                        7
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-14",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-8",
                        6
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-15",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-8",
                        5
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-16",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-8",
                        4
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-17",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-8",
                        3
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-18",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-8",
                        2
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-19",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-8",
                        1
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "SubPatchBox-8",
                        0
                    ],
                    "destination": [
                        "JsBox-16",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-61",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-8",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-61",
                        0
                    ],
                    "destination": [
                        "JsBox-16",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "TriggerBox-8",
                        0
                    ],
                    "destination": [
                        "DialBox-16",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "TriggerBox-8",
                        0
                    ],
                    "destination": [
                        "DialBox-17",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-62",
                        0
                    ],
                    "destination": [
                        "ToggleBox-12",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-62",
                        0
                    ],
                    "destination": [
                        "ToggleBox-13",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-62",
                        0
                    ],
                    "destination": [
                        "ToggleBox-14",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-62",
                        0
                    ],
                    "destination": [
                        "ToggleBox-15",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-62",
                        0
                    ],
                    "destination": [
                        "ToggleBox-16",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-62",
                        0
                    ],
                    "destination": [
                        "ToggleBox-17",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-62",
                        0
                    ],
                    "destination": [
                        "ToggleBox-18",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-62",
                        0
                    ],
                    "destination": [
                        "ToggleBox-19",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "TriggerBox-8",
                        0
                    ],
                    "destination": [
                        "MessageBox-61",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "TriggerBox-8",
                        0
                    ],
                    "destination": [
                        "MessageBox-62",
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