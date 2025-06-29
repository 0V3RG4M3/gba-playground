// This file has been automatically generated
use gba::mmio;
use gba::video::Color;

pub fn load() {
    mmio::BG_PALETTE.index(1).write(Color(0b0_00011_00010_00011));
    mmio::BG_PALETTE.index(2).write(Color(0b0_00111_00100_00100));
    mmio::BG_PALETTE.index(3).write(Color(0b0_00111_01000_00110));
    mmio::BG_PALETTE.index(4).write(Color(0b0_11101_11000_01010));
    mmio::BG_PALETTE.index(5).write(Color(0b0_10001_10000_10001));
    mmio::BG_PALETTE.index(6).write(Color(0b0_10111_10110_10011));
    mmio::BG_PALETTE.index(7).write(Color(0b0_01010_11011_11111));
    mmio::BG_PALETTE.index(8).write(Color(0b0_01101_10100_11011));
    mmio::BG_PALETTE.index(9).write(Color(0b0_00101_01110_11100));
    mmio::BG_PALETTE.index(10).write(Color(0b0_11010_01000_11100));
    mmio::BG_PALETTE.index(11).write(Color(0b0_10011_11000_11110));
    mmio::BG_PALETTE.index(12).write(Color(0b0_01010_01000_11110));
    mmio::BG_PALETTE.index(13).write(Color(0b0_11001_01000_11100));
    mmio::BG_PALETTE.index(14).write(Color(0b0_01001_00111_11110));
    mmio::BG_PALETTE.index(15).write(Color(0b0_01001_01000_11110));
    mmio::BG_PALETTE.index(16).write(Color(0b0_00011_00011_00011));
    mmio::BG_PALETTE.index(17).write(Color(0b0_00010_00011_00011));
    mmio::BG_PALETTE.index(18).write(Color(0b0_01000_00101_01001));
    mmio::BG_PALETTE.index(19).write(Color(0b0_11101_10111_01001));
    mmio::BG_PALETTE.index(20).write(Color(0b0_11101_10111_01010));
    mmio::BG_PALETTE.index(21).write(Color(0b0_01101_01101_01101));
    mmio::BG_PALETTE.index(22).write(Color(0b0_00110_00111_01101));
    mmio::BG_PALETTE.index(23).write(Color(0b0_00111_01011_10010));
    mmio::BG_PALETTE.index(24).write(Color(0b0_01001_10011_10010));
    mmio::BG_PALETTE.index(25).write(Color(0b0_01001_11011_11111));
    mmio::BG_PALETTE.index(26).write(Color(0b0_11111_11011_11001));
    mmio::BG_PALETTE.index(27).write(Color(0b0_10111_01111_11011));
    mmio::BG_PALETTE.index(28).write(Color(0b0_01001_00111_11101));
    mmio::BG_PALETTE.index(29).write(Color(0b0_01010_00111_11110));
    mmio::BG_PALETTE.index(30).write(Color(0b0_11111_11111_11111));

    // ../src/assets/graphics/backgrounds/wood1_16.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(0).write([
        0x02020202, 0x02020202, 0x17091702, 0x09090209, 0x17091702, 0x09090209, 0x09171702,
        0x09090209, 0x09091702, 0x12090209, 0x09090902, 0x17090209, 0x09090902, 0x17090209,
        0x09170902, 0x09090217,
    ]);
    mmio::CHARBLOCK0_8BPP.index(1).write([
        0x02020202, 0x02020202, 0x17021717, 0x02170917, 0x17021717, 0x02170917, 0x17021717,
        0x02171717, 0x17021709, 0x02091709, 0x17021709, 0x02091709, 0x17021709, 0x02091717,
        0x17021709, 0x02090917,
    ]);
    mmio::CHARBLOCK0_8BPP.index(2).write([
        0x09170902, 0x09090217, 0x12090902, 0x09090209, 0x12090902, 0x17090209, 0x17091702,
        0x17090209, 0x09091702, 0x17090217, 0x09091702, 0x17090209, 0x09091702, 0x17090209,
        0x02020202, 0x02020202,
    ]);
    mmio::CHARBLOCK0_8BPP.index(3).write([
        0x17021709, 0x02090912, 0x17021717, 0x02090912, 0x17021717, 0x02090917, 0x09021717,
        0x02090917, 0x09021717, 0x02170917, 0x09021709, 0x02171717, 0x09021709, 0x02091717,
        0x02020202, 0x02020202,
    ]);

    // ../src/assets/graphics/backgrounds/concrete1_16.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(4).write([
        0x0303031e, 0x03030303, 0x06060603, 0x06060606, 0x05060603, 0x06050606, 0x06050603,
        0x06060606, 0x06060603, 0x06060606, 0x06060603, 0x06060606, 0x06060603, 0x06060605,
        0x06060603, 0x06060606,
    ]);
    mmio::CHARBLOCK0_8BPP.index(5).write([
        0x03030303, 0x1e030303, 0x06060606, 0x03060606, 0x06060606, 0x03060606, 0x06060606,
        0x03060506, 0x06060606, 0x03060606, 0x06061506, 0x03060606, 0x06060606, 0x03060606,
        0x06060606, 0x03060606,
    ]);
    mmio::CHARBLOCK0_8BPP.index(6).write([
        0x06060603, 0x06060606, 0x06060603, 0x06060605, 0x06060603, 0x06060606, 0x06150603,
        0x06060606, 0x06060603, 0x06060606, 0x06060603, 0x06060606, 0x06060603, 0x06060606,
        0x0303031e, 0x03030303,
    ]);
    mmio::CHARBLOCK0_8BPP.index(7).write([
        0x06060605, 0x03060605, 0x06060606, 0x03060606, 0x06060606, 0x03060606, 0x06060606,
        0x03060606, 0x06060606, 0x03060606, 0x06060606, 0x03060606, 0x06060606, 0x03050606,
        0x03030303, 0x1e030303,
    ]);

    // ../src/assets/graphics/backgrounds/concrete2_16.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(8).write([
        0x0202021e, 0x02020202, 0x06050602, 0x1a060606, 0x051a0602, 0x061a0606, 0x06060602,
        0x06060605, 0x06060602, 0x1a060606, 0x06151a02, 0x05060506, 0x06061802, 0x0b1a1a06,
        0x06060b02, 0x0b061a06,
    ]);
    mmio::CHARBLOCK0_8BPP.index(9).write([
        0x02020202, 0x1e020202, 0x1a060b06, 0x0206060b, 0x1a0b0b06, 0x0206061a, 0x0b060b0b,
        0x0206050b, 0x0b1a0b1a, 0x02060606, 0x1a1a0b0b, 0x0206060b, 0x1a0b061a, 0x02060615,
        0x0b060b06, 0x0206060b,
    ]);
    mmio::CHARBLOCK0_8BPP.index(10).write([
        0x06060602, 0x1a0b1a1a, 0x06051a02, 0x0b1a0b06, 0x06060b02, 0x0b060606, 0x06061a02,
        0x0b060606, 0x061a0602, 0x0b1a0606, 0x050b0602, 0x0b0b0b1a, 0x15151502, 0x06151505,
        0x0202021e, 0x02020202,
    ]);
    mmio::CHARBLOCK0_8BPP.index(11).write([
        0x1a0b060b, 0x0206060b, 0x0b060b1a, 0x0206060b, 0x0b0b1a0b, 0x0206061a, 0x1a0b060b,
        0x02151a0b, 0x06060b1a, 0x0206060b, 0x06060b06, 0x02060606, 0x15151506, 0x02151515,
        0x02020202, 0x1e020202,
    ]);

    // ../src/assets/graphics/backgrounds/discofloor_16.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(12).write([
        0x10101010, 0x10101010, 0x19190710, 0x0c0e1007, 0x07190710, 0x0f1d1007, 0x19190710,
        0x0e0f1007, 0x19070710, 0x0e0f1007, 0x10101010, 0x10101010, 0x0e0e0e10, 0x0707100e,
        0x1d1d0e10, 0x0719101d,
    ]);
    mmio::CHARBLOCK0_8BPP.index(13).write([
        0x10011010, 0x10101010, 0x13100e0f, 0x10130414, 0x14010c1c, 0x10140414, 0x14100e0e,
        0x10131414, 0x14100e0e, 0x10141404, 0x10101010, 0x10101010, 0x0a100707, 0x100a0d0a,
        0x0d100707, 0x100d0d0a,
    ]);
    mmio::CHARBLOCK0_8BPP.index(14).write([
        0x1d0f0e10, 0x1907101d, 0x0e0f0e10, 0x0707101d, 0x10101010, 0x10101010, 0x14141410,
        0x0a0d1014, 0x14141410, 0x0d0d1014, 0x04141410, 0x0d0a1014, 0x14141410, 0x0a0a1014,
        0x10101010, 0x10101010,
    ]);
    mmio::CHARBLOCK0_8BPP.index(15).write([
        0x0a100707, 0x100a0d0a, 0x0a100707, 0x100d0a0a, 0x10101010, 0x10101010, 0x19100a0d,
        0x10070707, 0x07100a0d, 0x11070719, 0x07100d0a, 0x10071907, 0x07100a0a, 0x10190719,
        0x10101010, 0x10101010,
    ]);

    // ../src/assets/graphics/backgrounds/wood2_16.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(16).write([
        0x02020202, 0x02020202, 0x09090902, 0x09090909, 0x12120202, 0x09160912, 0x12120202,
        0x09090909, 0x12120202, 0x09090909, 0x09120202, 0x09090909, 0x09120202, 0x09090909,
        0x09120202, 0x09090909,
    ]);
    mmio::CHARBLOCK0_8BPP.index(17).write([
        0x02020202, 0x02020202, 0x09090909, 0x09090909, 0x16081612, 0x02161209, 0x161b0912,
        0x02161209, 0x161b0916, 0x02161209, 0x161b0916, 0x02161209, 0x16080909, 0x02160909,
        0x09081209, 0x02160916,
    ]);
    mmio::CHARBLOCK0_8BPP.index(18).write([
        0x09120202, 0x09160916, 0x09120202, 0x09160916, 0x09120202, 0x09160916, 0x09120202,
        0x09161216, 0x09120202, 0x09161216, 0x09120202, 0x09121216, 0x09120202, 0x09121616,
        0x02020202, 0x02020202,
    ]);
    mmio::CHARBLOCK0_8BPP.index(19).write([
        0x09091209, 0x02160916, 0x16090209, 0x02160816, 0x09080916, 0x02161609, 0x091b0916,
        0x02161209, 0x09080916, 0x02161209, 0x09160909, 0x02161609, 0x09160909, 0x02161609,
        0x02020202, 0x02020202,
    ]);
}

// ../src/assets/graphics/backgrounds/wood1_16.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_WOOD1_16: usize = 0;
pub const SIZE_WOOD1_16: usize = 4;

// ../src/assets/graphics/backgrounds/concrete1_16.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_CONCRETE1_16: usize = 4;
pub const SIZE_CONCRETE1_16: usize = 4;

// ../src/assets/graphics/backgrounds/concrete2_16.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_CONCRETE2_16: usize = 8;
pub const SIZE_CONCRETE2_16: usize = 4;

// ../src/assets/graphics/backgrounds/discofloor_16.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_DISCOFLOOR_16: usize = 12;
pub const SIZE_DISCOFLOOR_16: usize = 4;

// ../src/assets/graphics/backgrounds/wood2_16.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_WOOD2_16: usize = 16;
pub const SIZE_WOOD2_16: usize = 4;
