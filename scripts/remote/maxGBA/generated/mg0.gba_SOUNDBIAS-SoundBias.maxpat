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
                    "text": "SOUNDBIAS",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-85",
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
                    "maxclass": "number",
                    "varname": "SOUNDBIAS-bias_level",
                    "numinlets": 1,
                    "minimum": 0,
                    "maximum": 511,
                    "numoutlets": 2,
                    "parameter_enable": 1,
                    "outlettype": [
                        "",
                        "bang"
                    ],
                    "id": "NumberBox-3",
                    "patching_rect": [
                        234.0,
                        74.0,
                        50.0,
                        20.0
                    ],
                    "saved_attribute_attributes": {
                        "valueof": {
                            "parameter_longname": "SOUNDBIAS-bias_level",
                            "parameter_mmax": 511,
                            "parameter_shortname": "bias_level",
                            "parameter_type": 0
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "bias_level",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-86",
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
                    "varname": "SOUNDBIAS-sample_cycle",
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
                    "id": "DialBox-19",
                    "patching_rect": [
                        202.0,
                        106.0,
                        25.0,
                        23.0
                    ],
                    "saved_attribute_attributes": {
                        "valueof": {
                            "parameter_linknames": 1,
                            "parameter_longname": "SOUNDBIAS-sample_cycle",
                            "parameter_mmax": 3,
                            "parameter_shortname": "sample_cycle",
                            "parameter_type": 1,
                            "parameter_unitstyle": 0
                        }
                    }
                }
            },
            {
                "box": {
                    "maxclass": "message",
                    "text": "sample_cycle",
                    "numinlets": 2,
                    "numoutlets": 1,
                    "outlettype": [
                        ""
                    ],
                    "id": "MessageBox-87",
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
                    "maxclass": "newobj",
                    "text": "p SoundBias",
                    "numinlets": 3,
                    "numoutlets": 1,
                    "id": "SubPatchBox-11",
                    "patching_rect": [
                        170.0,
                        170.0,
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
                                        "annotation": "bias_level",
                                        "comment": "bias_level in [0, 511]",
                                        "hint": "bias_level",
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
                                        "id": "PrependBox-65",
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
                                        "text": "prepend bias_level"
                                    }
                                },
                                {
                                    "box": {
                                        "annotation": "sample_cycle",
                                        "comment": "sample_cycle in [0, 3]",
                                        "hint": "sample_cycle",
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
                                        "id": "PrependBox-66",
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
                                        "text": "prepend sample_cycle"
                                    }
                                },
                                {
                                    "box": {
                                        "id": "PrependBox-67",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 1,
                                        "outlettype": [
                                            ""
                                        ],
                                        "patching_rect": [
                                            170.0,
                                            223.33333333333331,
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
                                        "id": "JsBox-21",
                                        "maxclass": "newobj",
                                        "numinlets": 1,
                                        "numoutlets": 2,
                                        "outlettype": [
                                            "",
                                            ""
                                        ],
                                        "patching_rect": [
                                            170.0,
                                            303.3333333333333,
                                            100.0,
                                            22.0
                                        ],
                                        "saved_object_attributes": {
                                            "filename": "gba_sound.js",
                                            "parameter_enable": 0
                                        },
                                        "text": "js gba_sound.js SoundBias",
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
                                            170.0,
                                            383.33333333333337,
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
                                            "InletBox-3",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-65",
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
                                            "PrependBox-66",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-65",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-67",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-66",
                                            0
                                        ],
                                        "destination": [
                                            "PrependBox-67",
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
                                            "JsBox-21",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "PrependBox-67",
                                            0
                                        ],
                                        "destination": [
                                            "JsBox-21",
                                            0
                                        ]
                                    }
                                },
                                {
                                    "patchline": {
                                        "source": [
                                            "JsBox-21",
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
                    "id": "MessageBox-88",
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
                    "color": [
                        1.0,
                        0.588235294117647,
                        0.317647058823529,
                        1.0
                    ],
                    "id": "JsBox-22",
                    "maxclass": "newobj",
                    "numinlets": 1,
                    "numoutlets": 2,
                    "outlettype": [
                        "",
                        ""
                    ],
                    "patching_rect": [
                        170.0,
                        234.0,
                        100.0,
                        22.0
                    ],
                    "saved_object_attributes": {
                        "filename": "gba_mmio.js",
                        "parameter_enable": 0
                    },
                    "text": "js gba_mmio.js SOUNDBIAS",
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
                    "id": "TriggerBox-11",
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
                    "id": "MessageBox-89",
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
                        "NumberBox-3",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-11",
                        2
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "DialBox-19",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-11",
                        1
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "SubPatchBox-11",
                        0
                    ],
                    "destination": [
                        "JsBox-22",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-88",
                        0
                    ],
                    "destination": [
                        "SubPatchBox-11",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "MessageBox-88",
                        0
                    ],
                    "destination": [
                        "JsBox-22",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "TriggerBox-11",
                        0
                    ],
                    "destination": [
                        "NumberBox-3",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "TriggerBox-11",
                        0
                    ],
                    "destination": [
                        "DialBox-19",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "TriggerBox-11",
                        0
                    ],
                    "destination": [
                        "MessageBox-88",
                        0
                    ]
                }
            },
            {
                "patchline": {
                    "source": [
                        "TriggerBox-11",
                        0
                    ],
                    "destination": [
                        "MessageBox-89",
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