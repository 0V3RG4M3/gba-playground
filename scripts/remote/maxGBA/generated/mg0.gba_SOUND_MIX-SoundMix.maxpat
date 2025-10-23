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
                    "text": "SOUND_MIX",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-63",
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
                    "varname": "SOUND_MIX-psg",
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
                    "id": "DialBox-18",
                    "patching_rect": [
                        522.0,
                        74.0,
                        25.0,
                        23.0
                    ],
                    "saved_attribute_attributes": {
                        "valueof": {
                            "parameter_linknames": 1,
                            "parameter_longname": "SOUND_MIX-psg",
                            "parameter_mmax": 3,
                            "parameter_shortname": "psg",
                            "parameter_type": 1,
                            "parameter_unitstyle": 0
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "psg",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-64",
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
                    "varname": "SOUND_MIX-sound_a_full",
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
                    "id": "ToggleBox-20",
                    "patching_rect": [
                        490.0,
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
                            "parameter_longname": "SOUND_MIX-sound_a_full",
                            "parameter_mmax": 1,
                            "parameter_shortname": "sound_a_full",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "sound_a_full",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-65",
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
                    "varname": "SOUND_MIX-sound_b_full",
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
                    "id": "ToggleBox-21",
                    "patching_rect": [
                        458.0,
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
                            "parameter_longname": "SOUND_MIX-sound_b_full",
                            "parameter_mmax": 1,
                            "parameter_shortname": "sound_b_full",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "sound_b_full",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-66",
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
                    "varname": "SOUND_MIX-sound_a_right",
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
                    "id": "ToggleBox-22",
                    "patching_rect": [
                        426.0,
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
                            "parameter_longname": "SOUND_MIX-sound_a_right",
                            "parameter_mmax": 1,
                            "parameter_shortname": "sound_a_right",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "sound_a_right",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-67",
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
                    "varname": "SOUND_MIX-sound_a_left",
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
                    "id": "ToggleBox-23",
                    "patching_rect": [
                        394.0,
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
                            "parameter_longname": "SOUND_MIX-sound_a_left",
                            "parameter_mmax": 1,
                            "parameter_shortname": "sound_a_left",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "sound_a_left",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-68",
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
                    "varname": "SOUND_MIX-sound_a_timer",
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
                    "id": "ToggleBox-24",
                    "patching_rect": [
                        362.0,
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
                            "parameter_longname": "SOUND_MIX-sound_a_timer",
                            "parameter_mmax": 1,
                            "parameter_shortname": "sound_a_timer",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "sound_a_timer",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-69",
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
                    "varname": "SOUND_MIX-sound_a_reset",
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
                    "id": "ToggleBox-25",
                    "patching_rect": [
                        330.0,
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
                            "parameter_longname": "SOUND_MIX-sound_a_reset",
                            "parameter_mmax": 1,
                            "parameter_shortname": "sound_a_reset",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "sound_a_reset",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-70",
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
                    "varname": "SOUND_MIX-sound_b_right",
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
                    "id": "ToggleBox-26",
                    "patching_rect": [
                        298.0,
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
                            "parameter_longname": "SOUND_MIX-sound_b_right",
                            "parameter_mmax": 1,
                            "parameter_shortname": "sound_b_right",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "sound_b_right",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-71",
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
                    "varname": "SOUND_MIX-sound_b_left",
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
                    "id": "ToggleBox-27",
                    "patching_rect": [
                        266.0,
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
                            "parameter_longname": "SOUND_MIX-sound_b_left",
                            "parameter_mmax": 1,
                            "parameter_shortname": "sound_b_left",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "sound_b_left",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-72",
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
                    "varname": "SOUND_MIX-sound_b_timer",
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
                    "id": "ToggleBox-28",
                    "patching_rect": [
                        234.0,
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
                            "parameter_longname": "SOUND_MIX-sound_b_timer",
                            "parameter_mmax": 1,
                            "parameter_shortname": "sound_b_timer",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "sound_b_timer",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-73",
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
                    "maxclass": "toggle",
                    "varname": "SOUND_MIX-sound_b_reset",
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
                    "id": "ToggleBox-29",
                    "patching_rect": [
                        202.0,
                        394.0,
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
                            "parameter_longname": "SOUND_MIX-sound_b_reset",
                            "parameter_mmax": 1,
                            "parameter_shortname": "sound_b_reset",
                            "parameter_type": 2
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "sound_b_reset",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-74",
                    "patching_rect": [
                        10.0,
                        394.0,
                        89.0,
                        20.0
                    ]
                }
            },
            {
                "box": {
                    "maxclass": "newobj",
                    "text": "p SoundMix",
                    "numinlets": 12,
                    "numoutlets": 1,
                    "id": "SubPatchBox-9",
                    "patching_rect": [
                        170.0,
                        458.0,
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
                                        "annotation": "psg",
                                        "comment": "psg in [0, 3]",
                                        "hint": "psg",
                                        "id": "InletBox-12",
                                        "index": 12,
                                        "maxclass": "inlet",
                                        "numinlets": 0,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            890.0,
                                            303.3333333333333,
                                            30.0,
                                            30.0
                                        ]
                                    }
                                },
                                {
                                    "box": {
                                        "id": "PrependBox-47",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            890.0,
                                            383.33333333333326,
                                            80.0,
                                            22.0
                                        ],
                                        "text": "prepend psg"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "sound_a_full",
                                        "comment": "sound_a_full in [0, 1]",
                                        "hint": "sound_a_full",
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
                                        "id": "PrependBox-48",
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
                                        "text": "prepend sound_a_full"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "sound_b_full",
                                        "comment": "sound_b_full in [0, 1]",
                                        "hint": "sound_b_full",
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
                                        "id": "PrependBox-49",
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
                                        "text": "prepend sound_b_full"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "sound_a_right",
                                        "comment": "sound_a_right in [0, 1]",
                                        "hint": "sound_a_right",
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
                                        "id": "PrependBox-50",
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
                                        "text": "prepend sound_a_right"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "sound_a_left",
                                        "comment": "sound_a_left in [0, 1]",
                                        "hint": "sound_a_left",
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
                                        "id": "PrependBox-51",
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
                                        "text": "prepend sound_a_left"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "sound_a_timer",
                                        "comment": "sound_a_timer in [0, 1]",
                                        "hint": "sound_a_timer",
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
                                        "id": "PrependBox-52",
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
                                        "text": "prepend sound_a_timer"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "sound_a_reset",
                                        "comment": "sound_a_reset in [0, 1]",
                                        "hint": "sound_a_reset",
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
                                        "id": "PrependBox-53",
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
                                        "text": "prepend sound_a_reset"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "sound_b_right",
                                        "comment": "sound_b_right in [0, 1]",
                                        "hint": "sound_b_right",
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
                                        "id": "PrependBox-54",
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
                                        "text": "prepend sound_b_right"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "sound_b_left",
                                        "comment": "sound_b_left in [0, 1]",
                                        "hint": "sound_b_left",
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
                                        "id": "PrependBox-55",
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
                                        "text": "prepend sound_b_left"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "sound_b_timer",
                                        "comment": "sound_b_timer in [0, 1]",
                                        "hint": "sound_b_timer",
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
                                        "id": "PrependBox-56",
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
                                        "text": "prepend sound_b_timer"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "sound_b_reset",
                                        "comment": "sound_b_reset in [0, 1]",
                                        "hint": "sound_b_reset",
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
                                        "id": "PrependBox-57",
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
                                        "text": "prepend sound_b_reset"
                                    }
                                },
                                {
                                    "box": {
                                        "id": "PrependBox-58",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            530.0,
                                            343.33333333333326,
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
                                        "id": "JsBox-17",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 2,
                                        "outlettype": [
                                            "",
                                            ""
                                        ],
                                        "patching_rect": [
                                            530.0,
                                            423.33333333333326,
                                            100.0,
                                            22.0
                                        ],
                                        "saved_object_attributes": {
                                            "filename": "gba_sound.js",
                                            "parameter_enable": 0
                                        },
                                        "text": "js gba_sound.js SoundMix",
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
                                            530.0,
                                            503.33333333333326,
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
                                            "InletBox-12",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-47",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "InletBox-11",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-48",
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
                                            "PrependBox-49",
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
                                            "PrependBox-50",
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
                                            "PrependBox-51",
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
                                            "PrependBox-52",
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
                                            "PrependBox-53",
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
                                            "PrependBox-54",
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
                                            "PrependBox-55",
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
                                            "PrependBox-56",
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
                                            "PrependBox-57",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-47",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-58",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-48",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-58",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-49",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-58",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-50",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-58",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-51",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-58",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-52",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-58",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-53",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-58",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-54",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-58",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-55",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-58",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-56",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-58",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-57",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-58",
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
                                            "JsBox-17",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-58",
                                            0
                                        ],
                                        "destination": [
                                            "JsBox-17",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "JsBox-17",
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
                    "id": "MessageBox-75",
                    "patching_rect": [
                        10.0,
                        458.0,
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
                    "id": "JsBox-18",
                    "maxclass": "newobj",
                    "numinlets": 1,
                    "numoutlets": 2,
                    "outlettype": [
                        "",
                        ""
                    ],
                    "patching_rect": [
                        170.0,
                        522.0,
                        100.0,
                        22.0
                    ],
                    "saved_object_attributes": {
                        "filename": "gba_mmio.js",
                        "parameter_enable": 0
                    },
                    "text": "js gba_mmio.js SOUND_MIX",
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
                    "id": "TriggerBox-9",
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
                    "id": "MessageBox-76",
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
                        "DialBox-18",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-9",
                        11
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-20",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-9",
                        10
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-21",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-9",
                        9
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-22",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-9",
                        8
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-23",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-9",
                        7
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-24",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-9",
                        6
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-25",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-9",
                        5
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-26",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-9",
                        4
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-27",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-9",
                        3
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-28",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-9",
                        2
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "ToggleBox-29",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-9",
                        1
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "SubPatchBox-9",
                        0
                    ],
                    "destination": [
                        "JsBox-18",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-75",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-9",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-75",
                        0
                    ],
                    "destination": [
                        "JsBox-18",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "TriggerBox-9",
                        0
                    ],
                    "destination": [
                        "DialBox-18",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-76",
                        0
                    ],
                    "destination": [
                        "ToggleBox-20",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-76",
                        0
                    ],
                    "destination": [
                        "ToggleBox-21",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-76",
                        0
                    ],
                    "destination": [
                        "ToggleBox-22",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-76",
                        0
                    ],
                    "destination": [
                        "ToggleBox-23",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-76",
                        0
                    ],
                    "destination": [
                        "ToggleBox-24",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-76",
                        0
                    ],
                    "destination": [
                        "ToggleBox-25",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-76",
                        0
                    ],
                    "destination": [
                        "ToggleBox-26",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-76",
                        0
                    ],
                    "destination": [
                        "ToggleBox-27",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-76",
                        0
                    ],
                    "destination": [
                        "ToggleBox-28",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-76",
                        0
                    ],
                    "destination": [
                        "ToggleBox-29",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "TriggerBox-9",
                        0
                    ],
                    "destination": [
                        "MessageBox-75",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "TriggerBox-9",
                        0
                    ],
                    "destination": [
                        "MessageBox-76",
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