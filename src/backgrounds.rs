// This file has been automatically generated
use gba::mmio;
use gba::video::Color;

pub fn load() {
    mmio::BG_PALETTE.index(1).write(Color(0b0_00111_00100_00100));
    mmio::BG_PALETTE.index(2).write(Color(0b0_00111_01000_00110));
    mmio::BG_PALETTE.index(3).write(Color(0b0_10000_01100_00110));
    mmio::BG_PALETTE.index(4).write(Color(0b0_01111_01000_01000));
    mmio::BG_PALETTE.index(5).write(Color(0b0_11101_11010_01100));
    mmio::BG_PALETTE.index(6).write(Color(0b0_10001_10000_10001));
    mmio::BG_PALETTE.index(7).write(Color(0b0_10111_01111_11011));
    mmio::BG_PALETTE.index(8).write(Color(0b0_00111_11110_11111));
    mmio::BG_PALETTE.index(9).write(Color(0b0_00110_01101_01001));
    mmio::BG_PALETTE.index(10).write(Color(0b0_00101_01001_01010));
    mmio::BG_PALETTE.index(11).write(Color(0b0_01010_01011_01011));
    mmio::BG_PALETTE.index(12).write(Color(0b0_11111_10011_01100));
    mmio::BG_PALETTE.index(13).write(Color(0b0_01101_01101_01101));
    mmio::BG_PALETTE.index(14).write(Color(0b0_01000_00101_01001));
    mmio::BG_PALETTE.index(15).write(Color(0b0_00110_00111_01101));
    mmio::BG_PALETTE.index(16).write(Color(0b0_01001_10011_10010));
    mmio::BG_PALETTE.index(17).write(Color(0b0_00111_01011_10010));
    mmio::BG_PALETTE.index(18).write(Color(0b0_11111_11011_11001));
    mmio::BG_PALETTE.index(19).write(Color(0b0_01100_01011_11011));
    mmio::BG_PALETTE.index(20).write(Color(0b0_11111_11111_11111));

    // ../src/assets/graphics/backgrounds/2_mosais_blue.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(0).write([
        0x12121212, 0x0c0c0c0c, 0x05050505, 0x0505120c, 0x05050505, 0x0505120c, 0x0c0c0c0c,
        0x0505120c, 0x0c0c0c0c, 0x0505120c, 0x0505120c, 0x0505120c, 0x0505120c, 0x0505120c,
        0x0505120c, 0x1212120c,
    ]);
    mmio::CHARBLOCK0_8BPP.index(1).write([
        0x0505120c, 0x1212120c, 0x0505120c, 0x0505120c, 0x0505120c, 0x0505120c, 0x1212120c,
        0x0c0c0c0c, 0x1212120c, 0x12121212, 0x0505120c, 0x05050505, 0x0505120c, 0x05050505,
        0x0c0c0c0c, 0x0c0c0c0c,
    ]);
    mmio::CHARBLOCK0_8BPP.index(2).write([
        0x0505120c, 0x1212120c, 0x0505120c, 0x0505120c, 0x0505120c, 0x0505120c, 0x1212120c,
        0x0c0c0c0c, 0x1212120c, 0x12121212, 0x0505120c, 0x05050505, 0x0505120c, 0x05050505,
        0x0c0c0c0c, 0x0c0c0c0c,
    ]);
    mmio::CHARBLOCK0_8BPP.index(3).write([
        0x12121212, 0x0c0c0c0c, 0x05050505, 0x0505120c, 0x05050505, 0x0505120c, 0x0c0c0c0c,
        0x0505120c, 0x0c0c0c0c, 0x0505120c, 0x0505120c, 0x0505120c, 0x0505120c, 0x0505120c,
        0x0505120c, 0x1212120c,
    ]);

    // ../src/assets/graphics/backgrounds/1_grass.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(4).write([
        0x0d0d0d06, 0x0d091006, 0x0a0a090b, 0x09090909, 0x0a090d09, 0x02090909, 0x09090909,
        0x020a0a09, 0x02090d09, 0x09090a0a, 0x0a090909, 0x02020909, 0x09090909, 0x0a020209,
        0x02090909, 0x09020202,
    ]);
    mmio::CHARBLOCK0_8BPP.index(5).write([
        0x09090909, 0x09060903, 0x0b090909, 0x09090909, 0x020a090a, 0x09090902, 0x09090201,
        0x090b090a, 0x0a090a02, 0x09090909, 0x090a0a0a, 0x09090909, 0x02020909, 0x09090209,
        0x0902020a, 0x0309090a,
    ]);
    mmio::CHARBLOCK0_8BPP.index(6).write([
        0x0a020909, 0x0a0a0202, 0x09010309, 0x0909020a, 0x0a0a0a09, 0x09090302, 0x0a0a090a,
        0x020a0902, 0x0a090109, 0x09090909, 0x09090909, 0x09090909, 0x0a090309, 0x09090a09,
        0x06060610, 0x09090909,
    ]);
    mmio::CHARBLOCK0_8BPP.index(7).write([
        0x03090a03, 0x09090902, 0x0a090909, 0x0a0b0902, 0x0a030909, 0x0b0a0909, 0x03020909,
        0x090a0902, 0x03090909, 0x09090a02, 0x03090909, 0x060d0302, 0x02030909, 0x0d090a09,
        0x06060606, 0x06060606,
    ]);

    // ../src/assets/graphics/backgrounds/5_rock.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(8).write([
        0x0d0b0b0d, 0x0b0b0b0d, 0x0b0d0d0d, 0x0b0b0d0b, 0x0b0d0d0d, 0x040d060d, 0x0d0d0d0d,
        0x0d060d0d, 0x030d0d0d, 0x060d0d0d, 0x0d0d0d0d, 0x0d0d0b0d, 0x0d0d0d0d, 0x0d0d0b0d,
        0x0d0d0d0d, 0x0d0d0d0d,
    ]);
    mmio::CHARBLOCK0_8BPP.index(9).write([
        0x1006060d, 0x06060606, 0x0606060b, 0x06060d06, 0x06060d0b, 0x060d0d06, 0x060d0d0b,
        0x0d060606, 0x060d030a, 0x06060606, 0x0d030b0a, 0x0d0d0606, 0x0d03030a, 0x0606060d,
        0x03030b0a, 0x06060606,
    ]);
    mmio::CHARBLOCK0_8BPP.index(10).write([
        0x0d0d0d0d, 0x0d0b0b0b, 0x0d060d0d, 0x0d060d0b, 0x0a0d0d0d, 0x0d0d0d0d, 0x0a0d0d06,
        0x0d0d0d0d, 0x0d0a0d0d, 0x0d0d0d0d, 0x0d0a0b0d, 0x0d0d0d0d, 0x0d0d040b, 0x0d0d0d0d,
        0x0d0d0b0a, 0x0d0d0d0d,
    ]);
    mmio::CHARBLOCK0_8BPP.index(11).write([
        0x0303040b, 0x0d06060d, 0x030b0b0d, 0x0d060d03, 0x0d0a0d0d, 0x060d0d0d, 0x030b0d0d,
        0x06060d0d, 0x090d0d06, 0x060d0d06, 0x0d060d0d, 0x06060d0d, 0x0d0d0d0d, 0x060d0d0d,
        0x0d0d0d0d, 0x0d0d0d0d,
    ]);

    // ../src/assets/graphics/backgrounds/6_woodflore.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(12).write([
        0x0f0f0f0f, 0x0f0f0f0f, 0x11111111, 0x11111111, 0x11111111, 0x11111111, 0x11111111,
        0x11111111, 0x0f0f0f0f, 0x0f0f0f0f, 0x0e0e0e0e, 0x0e0e0e0e, 0x01010101, 0x01010101,
        0x01010101, 0x01010101,
    ]);
    mmio::CHARBLOCK0_8BPP.index(13).write([
        0x0f0f0f0f, 0x0f0f0f0f, 0x11111111, 0x11111111, 0x11111111, 0x11111111, 0x11111111,
        0x11111111, 0x0f0f0f0f, 0x0f0f0f0f, 0x0e0e0e0e, 0x0e0e0e0e, 0x01010101, 0x01010101,
        0x01010101, 0x01010101,
    ]);
    mmio::CHARBLOCK0_8BPP.index(14).write([
        0x01010101, 0x01010101, 0x0e020101, 0x01010102, 0x0a020202, 0x02020202, 0x0b0b0a0a,
        0x0a0a0b0b, 0x0b0b0b0b, 0x0b0b0b0b, 0x0b0b0a0a, 0x0a0a0b0b, 0x0f0f0f0f, 0x0f0f0f0f,
        0x0f0f0f0f, 0x0f0f0f0f,
    ]);
    mmio::CHARBLOCK0_8BPP.index(15).write([
        0x01010101, 0x01010101, 0x01010101, 0x01010101, 0x02020202, 0x02020202, 0x0a0a0a0a,
        0x0a0a0a0a, 0x0b0b0b0b, 0x0b0b0b0b, 0x0a0a0a0a, 0x0a0a0a0a, 0x0f0f0f0f, 0x0f0f0f0f,
        0x0f0f0f0f, 0x0f0f0f0f,
    ]);

    // ../src/assets/graphics/backgrounds/3_mosais_brown.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(16).write([
        0x0f0f0f0f, 0x01010101, 0x0e0e0e0e, 0x0e0e0f01, 0x0e0e0e0e, 0x0e0e0f01, 0x01010101,
        0x0e0e0f01, 0x01010101, 0x0e0e0f01, 0x0e0e0f01, 0x0e0e0f01, 0x0e0e0f01, 0x0e0e0f01,
        0x0e0e0f01, 0x0f0f0f01,
    ]);
    mmio::CHARBLOCK0_8BPP.index(17).write([
        0x0e0e0f01, 0x0f0f0f01, 0x0e0e0f01, 0x0e0e0f01, 0x0e0e0f01, 0x0e0e0f01, 0x0f0f0f01,
        0x01010101, 0x0f0f0f01, 0x0f0f0f0f, 0x0e0e0f01, 0x0e0e0e0e, 0x0e0e0f01, 0x0e0e0e0e,
        0x01010101, 0x01010101,
    ]);
    mmio::CHARBLOCK0_8BPP.index(18).write([
        0x0e0e0f01, 0x0f0f0f01, 0x0e0e0f01, 0x0e0e0f01, 0x0e0e0f01, 0x0e0e0f01, 0x0f0f0f01,
        0x01010101, 0x0f0f0f01, 0x0f0f0f0f, 0x0e0e0f01, 0x0e0e0e0e, 0x0e0e0f01, 0x0e0e0e0e,
        0x01010101, 0x01010101,
    ]);
    mmio::CHARBLOCK0_8BPP.index(19).write([
        0x0f0f0f0f, 0x01010101, 0x0e0e0e0e, 0x0e0e0f01, 0x0e0e0e0e, 0x0e0e0f01, 0x01010101,
        0x0e0e0f01, 0x01010101, 0x0e0e0f01, 0x0e0e0f01, 0x0e0e0f01, 0x0e0e0f01, 0x0e0e0f01,
        0x0e0e0f01, 0x0f0f0f01,
    ]);

    // ../src/assets/graphics/backgrounds/4_mosais_colorfull.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(20).write([
        0x13141414, 0x14141205, 0x13141414, 0x14141205, 0x08131313, 0x14141205, 0x07070707,
        0x12120805, 0x05081212, 0x07070707, 0x05121414, 0x13131308, 0x05121414, 0x14141413,
        0x05121414, 0x14141413,
    ]);
    mmio::CHARBLOCK0_8BPP.index(21).write([
        0x14141414, 0x13141414, 0x14141414, 0x13141414, 0x13131313, 0x08131313, 0x07070707,
        0x07070707, 0x07070707, 0x14141205, 0x13131313, 0x14141205, 0x14141414, 0x14141205,
        0x14141414, 0x14141205,
    ]);
    mmio::CHARBLOCK0_8BPP.index(22).write([
        0x05121414, 0x14141414, 0x05121414, 0x14141414, 0x05121414, 0x13131313, 0x05121414,
        0x07070707, 0x14141414, 0x12141414, 0x14141414, 0x12141414, 0x12121212, 0x08121212,
        0x05050505, 0x05050505,
    ]);
    mmio::CHARBLOCK0_8BPP.index(23).write([
        0x13141414, 0x14141205, 0x13141414, 0x14141205, 0x08131313, 0x14141205, 0x07070707,
        0x12120805, 0x05081212, 0x07070707, 0x05121414, 0x13131308, 0x05121414, 0x14141413,
        0x05121414, 0x14141413,
    ]);
}

// ../src/assets/graphics/backgrounds/2_mosais_blue.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_2_MOSAIS_BLUE: usize = 0;
pub const SIZE_2_MOSAIS_BLUE: usize = 4;

// ../src/assets/graphics/backgrounds/1_grass.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_1_GRASS: usize = 4;
pub const SIZE_1_GRASS: usize = 4;

// ../src/assets/graphics/backgrounds/5_rock.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_5_ROCK: usize = 8;
pub const SIZE_5_ROCK: usize = 4;

// ../src/assets/graphics/backgrounds/6_woodflore.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_6_WOODFLORE: usize = 12;
pub const SIZE_6_WOODFLORE: usize = 4;

// ../src/assets/graphics/backgrounds/3_mosais_brown.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_3_MOSAIS_BROWN: usize = 16;
pub const SIZE_3_MOSAIS_BROWN: usize = 4;

// ../src/assets/graphics/backgrounds/4_mosais_colorfull.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_4_MOSAIS_COLORFULL: usize = 20;
pub const SIZE_4_MOSAIS_COLORFULL: usize = 4;
