// This file has been automatically generated
use gba::mmio;
use gba::video::Color;

pub fn load() {
    mmio::BG_PALETTE.index(1).write(Color(0b0_00111_00100_00100));
    mmio::BG_PALETTE.index(2).write(Color(0b0_00111_01000_00110));
    mmio::BG_PALETTE.index(3).write(Color(0b0_10000_01100_00110));
    mmio::BG_PALETTE.index(4).write(Color(0b0_01111_01000_01000));
    mmio::BG_PALETTE.index(5).write(Color(0b0_11101_11000_01010));
    mmio::BG_PALETTE.index(6).write(Color(0b0_10001_10000_10001));
    mmio::BG_PALETTE.index(7).write(Color(0b0_01101_10100_11011));
    mmio::BG_PALETTE.index(8).write(Color(0b0_11010_01000_11100));
    mmio::BG_PALETTE.index(9).write(Color(0b0_11001_01000_11100));
    mmio::BG_PALETTE.index(10).write(Color(0b0_01010_01000_11110));
    mmio::BG_PALETTE.index(11).write(Color(0b0_01001_01000_11110));
    mmio::BG_PALETTE.index(12).write(Color(0b0_10011_11000_11110));
    mmio::BG_PALETTE.index(13).write(Color(0b0_01000_00101_01001));
    mmio::BG_PALETTE.index(14).write(Color(0b0_00101_01001_01010));
    mmio::BG_PALETTE.index(15).write(Color(0b0_00110_01101_01001));
    mmio::BG_PALETTE.index(16).write(Color(0b0_01101_01101_01101));
    mmio::BG_PALETTE.index(17).write(Color(0b0_01001_11011_11111));
    mmio::BG_PALETTE.index(18).write(Color(0b0_01010_00111_11110));
    mmio::BG_PALETTE.index(19).write(Color(0b0_00011_00010_00011));
    mmio::BG_PALETTE.index(20).write(Color(0b0_11101_11010_01100));
    mmio::BG_PALETTE.index(21).write(Color(0b0_10111_10110_10011));
    mmio::BG_PALETTE.index(22).write(Color(0b0_10111_01111_11011));
    mmio::BG_PALETTE.index(23).write(Color(0b0_00101_01110_11100));
    mmio::BG_PALETTE.index(24).write(Color(0b0_00111_11110_11111));
    mmio::BG_PALETTE.index(25).write(Color(0b0_00011_00011_00011));
    mmio::BG_PALETTE.index(26).write(Color(0b0_00010_00011_00011));
    mmio::BG_PALETTE.index(27).write(Color(0b0_11101_10111_01001));
    mmio::BG_PALETTE.index(28).write(Color(0b0_11101_10111_01010));
    mmio::BG_PALETTE.index(29).write(Color(0b0_01010_01011_01011));
    mmio::BG_PALETTE.index(30).write(Color(0b0_11111_10011_01100));
    mmio::BG_PALETTE.index(31).write(Color(0b0_00110_00111_01101));
    mmio::BG_PALETTE.index(32).write(Color(0b0_00111_01011_10010));
    mmio::BG_PALETTE.index(33).write(Color(0b0_01001_10011_10010));
    mmio::BG_PALETTE.index(34).write(Color(0b0_01100_01011_11011));
    mmio::BG_PALETTE.index(35).write(Color(0b0_11111_11011_11001));
    mmio::BG_PALETTE.index(36).write(Color(0b0_11111_11111_11111));
    mmio::BG_PALETTE.index(37).write(Color(0b0_01001_00111_11101));
    mmio::BG_PALETTE.index(38).write(Color(0b0_01001_00111_11110));
    mmio::BG_PALETTE.index(39).write(Color(0b0_01010_11011_11111));

    // ../src/assets/graphics/backgrounds/6_woodflore.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(0).write([
        0x1f1f1f1f, 0x1f1f1f1f, 0x20202020, 0x20202020, 0x20202020, 0x20202020, 0x20202020,
        0x20202020, 0x1f1f1f1f, 0x1f1f1f1f, 0x0d0d0d0d, 0x0d0d0d0d, 0x01010101, 0x01010101,
        0x01010101, 0x01010101,
    ]);
    mmio::CHARBLOCK0_8BPP.index(1).write([
        0x1f1f1f1f, 0x1f1f1f1f, 0x20202020, 0x20202020, 0x20202020, 0x20202020, 0x20202020,
        0x20202020, 0x1f1f1f1f, 0x1f1f1f1f, 0x0d0d0d0d, 0x0d0d0d0d, 0x01010101, 0x01010101,
        0x01010101, 0x01010101,
    ]);
    mmio::CHARBLOCK0_8BPP.index(2).write([
        0x01010101, 0x01010101, 0x0d020101, 0x01010102, 0x0e020202, 0x02020202, 0x1d1d0e0e,
        0x0e0e1d1d, 0x1d1d1d1d, 0x1d1d1d1d, 0x1d1d0e0e, 0x0e0e1d1d, 0x1f1f1f1f, 0x1f1f1f1f,
        0x1f1f1f1f, 0x1f1f1f1f,
    ]);
    mmio::CHARBLOCK0_8BPP.index(3).write([
        0x01010101, 0x01010101, 0x01010101, 0x01010101, 0x02020202, 0x02020202, 0x0e0e0e0e,
        0x0e0e0e0e, 0x1d1d1d1d, 0x1d1d1d1d, 0x0e0e0e0e, 0x0e0e0e0e, 0x1f1f1f1f, 0x1f1f1f1f,
        0x1f1f1f1f, 0x1f1f1f1f,
    ]);

    // ../src/assets/graphics/backgrounds/10_wood1_16.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(4).write([
        0x01010101, 0x01010101, 0x20172001, 0x17170117, 0x20172001, 0x17170117, 0x17202001,
        0x17170117, 0x17172001, 0x0d170117, 0x17171701, 0x20170117, 0x17171701, 0x20170117,
        0x17201701, 0x17170120,
    ]);
    mmio::CHARBLOCK0_8BPP.index(5).write([
        0x01010101, 0x01010101, 0x20012020, 0x01201720, 0x20012020, 0x01201720, 0x20012020,
        0x01202020, 0x20012017, 0x01172017, 0x20012017, 0x01172017, 0x20012017, 0x01172020,
        0x20012017, 0x01171720,
    ]);
    mmio::CHARBLOCK0_8BPP.index(6).write([
        0x17201701, 0x17170120, 0x0d171701, 0x17170117, 0x0d171701, 0x20170117, 0x20172001,
        0x20170117, 0x17172001, 0x20170120, 0x17172001, 0x20170117, 0x17172001, 0x20170117,
        0x01010101, 0x01010101,
    ]);
    mmio::CHARBLOCK0_8BPP.index(7).write([
        0x20012017, 0x0117170d, 0x20012020, 0x0117170d, 0x20012020, 0x01171720, 0x17012020,
        0x01171720, 0x17012020, 0x01201720, 0x17012017, 0x01202020, 0x17012017, 0x01172020,
        0x01010101, 0x01010101,
    ]);

    // ../src/assets/graphics/backgrounds/7_concrete1_16.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(8).write([
        0x02020224, 0x02020202, 0x15151502, 0x15151515, 0x06151502, 0x15061515, 0x15061502,
        0x15151515, 0x15151502, 0x15151515, 0x15151502, 0x15151515, 0x15151502, 0x15151506,
        0x15151502, 0x15151515,
    ]);
    mmio::CHARBLOCK0_8BPP.index(9).write([
        0x02020202, 0x24020202, 0x15151515, 0x02151515, 0x15151515, 0x02151515, 0x15151515,
        0x02150615, 0x15151515, 0x02151515, 0x15151015, 0x02151515, 0x15151515, 0x02151515,
        0x15151515, 0x02151515,
    ]);
    mmio::CHARBLOCK0_8BPP.index(10).write([
        0x15151502, 0x15151515, 0x15151502, 0x15151506, 0x15151502, 0x15151515, 0x15101502,
        0x15151515, 0x15151502, 0x15151515, 0x15151502, 0x15151515, 0x15151502, 0x15151515,
        0x02020224, 0x02020202,
    ]);
    mmio::CHARBLOCK0_8BPP.index(11).write([
        0x15151506, 0x02151506, 0x15151515, 0x02151515, 0x15151515, 0x02151515, 0x15151515,
        0x02151515, 0x15151515, 0x02151515, 0x15151515, 0x02151515, 0x15151515, 0x02061515,
        0x02020202, 0x24020202,
    ]);

    // ../src/assets/graphics/backgrounds/9_discofloor_16.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(12).write([
        0x19191919, 0x19191919, 0x11112719, 0x0a261927, 0x27112719, 0x0b121927, 0x11112719,
        0x260b1927, 0x11272719, 0x260b1927, 0x19191919, 0x19191919, 0x26262619, 0x27271926,
        0x12122619, 0x27111912,
    ]);
    mmio::CHARBLOCK0_8BPP.index(13).write([
        0x19131919, 0x19191919, 0x1b19260b, 0x191b051c, 0x1c130a25, 0x191c051c, 0x1c192626,
        0x191b1c1c, 0x1c192626, 0x191c1c05, 0x19191919, 0x19191919, 0x08192727, 0x19080908,
        0x09192727, 0x19090908,
    ]);
    mmio::CHARBLOCK0_8BPP.index(14).write([
        0x120b2619, 0x11271912, 0x260b2619, 0x27271912, 0x19191919, 0x19191919, 0x1c1c1c19,
        0x0809191c, 0x1c1c1c19, 0x0909191c, 0x051c1c19, 0x0908191c, 0x1c1c1c19, 0x0808191c,
        0x19191919, 0x19191919,
    ]);
    mmio::CHARBLOCK0_8BPP.index(15).write([
        0x08192727, 0x19080908, 0x08192727, 0x19090808, 0x19191919, 0x19191919, 0x11190809,
        0x19272727, 0x27190809, 0x1a272711, 0x27190908, 0x19271127, 0x27190808, 0x19112711,
        0x19191919, 0x19191919,
    ]);

    // ../src/assets/graphics/backgrounds/5_rock.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(16).write([
        0x101d1d10, 0x1d1d1d10, 0x1d101010, 0x1d1d101d, 0x1d101010, 0x04100610, 0x10101010,
        0x10061010, 0x03101010, 0x06101010, 0x10101010, 0x10101d10, 0x10101010, 0x10101d10,
        0x10101010, 0x10101010,
    ]);
    mmio::CHARBLOCK0_8BPP.index(17).write([
        0x21060610, 0x06060606, 0x0606061d, 0x06061006, 0x0606101d, 0x06101006, 0x0610101d,
        0x10060606, 0x0610030e, 0x06060606, 0x10031d0e, 0x10100606, 0x1003030e, 0x06060610,
        0x03031d0e, 0x06060606,
    ]);
    mmio::CHARBLOCK0_8BPP.index(18).write([
        0x10101010, 0x101d1d1d, 0x10061010, 0x1006101d, 0x0e101010, 0x10101010, 0x0e101006,
        0x10101010, 0x100e1010, 0x10101010, 0x100e1d10, 0x10101010, 0x1010041d, 0x10101010,
        0x10101d0e, 0x10101010,
    ]);
    mmio::CHARBLOCK0_8BPP.index(19).write([
        0x0303041d, 0x10060610, 0x031d1d10, 0x10061003, 0x100e1010, 0x06101010, 0x031d1010,
        0x06061010, 0x0f101006, 0x06101006, 0x10061010, 0x06061010, 0x10101010, 0x06101010,
        0x10101010, 0x10101010,
    ]);

    // ../src/assets/graphics/backgrounds/8_concrete2_16.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(20).write([
        0x01010124, 0x01010101, 0x15061501, 0x23151515, 0x06231501, 0x15231515, 0x15151501,
        0x15151506, 0x15151501, 0x23151515, 0x15102301, 0x06150615, 0x15152101, 0x0c232315,
        0x15150c01, 0x0c152315,
    ]);
    mmio::CHARBLOCK0_8BPP.index(21).write([
        0x01010101, 0x24010101, 0x23150c15, 0x0115150c, 0x230c0c15, 0x01151523, 0x0c150c0c,
        0x0115060c, 0x0c230c23, 0x01151515, 0x23230c0c, 0x0115150c, 0x230c1523, 0x01151510,
        0x0c150c15, 0x0115150c,
    ]);
    mmio::CHARBLOCK0_8BPP.index(22).write([
        0x15151501, 0x230c2323, 0x15062301, 0x0c230c15, 0x15150c01, 0x0c151515, 0x15152301,
        0x0c151515, 0x15231501, 0x0c231515, 0x060c1501, 0x0c0c0c23, 0x10101001, 0x15101006,
        0x01010124, 0x01010101,
    ]);
    mmio::CHARBLOCK0_8BPP.index(23).write([
        0x230c150c, 0x0115150c, 0x0c150c23, 0x0115150c, 0x0c0c230c, 0x01151523, 0x230c150c,
        0x0110230c, 0x15150c23, 0x0115150c, 0x15150c15, 0x01151515, 0x10101015, 0x01101010,
        0x01010101, 0x24010101,
    ]);

    // ../src/assets/graphics/backgrounds/1_grass.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(24).write([
        0x10101006, 0x100f2106, 0x0e0e0f1d, 0x0f0f0f0f, 0x0e0f100f, 0x020f0f0f, 0x0f0f0f0f,
        0x020e0e0f, 0x020f100f, 0x0f0f0e0e, 0x0e0f0f0f, 0x02020f0f, 0x0f0f0f0f, 0x0e02020f,
        0x020f0f0f, 0x0f020202,
    ]);
    mmio::CHARBLOCK0_8BPP.index(25).write([
        0x0f0f0f0f, 0x0f060f03, 0x1d0f0f0f, 0x0f0f0f0f, 0x020e0f0e, 0x0f0f0f02, 0x0f0f0201,
        0x0f1d0f0e, 0x0e0f0e02, 0x0f0f0f0f, 0x0f0e0e0e, 0x0f0f0f0f, 0x02020f0f, 0x0f0f020f,
        0x0f02020e, 0x030f0f0e,
    ]);
    mmio::CHARBLOCK0_8BPP.index(26).write([
        0x0e020f0f, 0x0e0e0202, 0x0f01030f, 0x0f0f020e, 0x0e0e0e0f, 0x0f0f0302, 0x0e0e0f0e,
        0x020e0f02, 0x0e0f010f, 0x0f0f0f0f, 0x0f0f0f0f, 0x0f0f0f0f, 0x0e0f030f, 0x0f0f0e0f,
        0x06060621, 0x0f0f0f0f,
    ]);
    mmio::CHARBLOCK0_8BPP.index(27).write([
        0x030f0e03, 0x0f0f0f02, 0x0e0f0f0f, 0x0e1d0f02, 0x0e030f0f, 0x1d0e0f0f, 0x03020f0f,
        0x0f0e0f02, 0x030f0f0f, 0x0f0f0e02, 0x030f0f0f, 0x06100302, 0x02030f0f, 0x100f0e0f,
        0x06060606, 0x06060606,
    ]);

    // ../src/assets/graphics/backgrounds/11_wood2_16.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(28).write([
        0x01010101, 0x01010101, 0x17171701, 0x17171717, 0x0d0d0101, 0x171f170d, 0x0d0d0101,
        0x17171717, 0x0d0d0101, 0x17171717, 0x170d0101, 0x17171717, 0x170d0101, 0x17171717,
        0x170d0101, 0x17171717,
    ]);
    mmio::CHARBLOCK0_8BPP.index(29).write([
        0x01010101, 0x01010101, 0x17171717, 0x17171717, 0x1f071f0d, 0x011f0d17, 0x1f16170d,
        0x011f0d17, 0x1f16171f, 0x011f0d17, 0x1f16171f, 0x011f0d17, 0x1f071717, 0x011f1717,
        0x17070d17, 0x011f171f,
    ]);
    mmio::CHARBLOCK0_8BPP.index(30).write([
        0x170d0101, 0x171f171f, 0x170d0101, 0x171f171f, 0x170d0101, 0x171f171f, 0x170d0101,
        0x171f0d1f, 0x170d0101, 0x171f0d1f, 0x170d0101, 0x170d0d1f, 0x170d0101, 0x170d1f1f,
        0x01010101, 0x01010101,
    ]);
    mmio::CHARBLOCK0_8BPP.index(31).write([
        0x17170d17, 0x011f171f, 0x1f170117, 0x011f071f, 0x1707171f, 0x011f1f17, 0x1716171f,
        0x011f0d17, 0x1707171f, 0x011f0d17, 0x171f1717, 0x011f1f17, 0x171f1717, 0x011f1f17,
        0x01010101, 0x01010101,
    ]);

    // ../src/assets/graphics/backgrounds/2_mosais_blue.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(32).write([
        0x23232323, 0x1e1e1e1e, 0x14141414, 0x1414231e, 0x14141414, 0x1414231e, 0x1e1e1e1e,
        0x1414231e, 0x1e1e1e1e, 0x1414231e, 0x1414231e, 0x1414231e, 0x1414231e, 0x1414231e,
        0x1414231e, 0x2323231e,
    ]);
    mmio::CHARBLOCK0_8BPP.index(33).write([
        0x1414231e, 0x2323231e, 0x1414231e, 0x1414231e, 0x1414231e, 0x1414231e, 0x2323231e,
        0x1e1e1e1e, 0x2323231e, 0x23232323, 0x1414231e, 0x14141414, 0x1414231e, 0x14141414,
        0x1e1e1e1e, 0x1e1e1e1e,
    ]);
    mmio::CHARBLOCK0_8BPP.index(34).write([
        0x1414231e, 0x2323231e, 0x1414231e, 0x1414231e, 0x1414231e, 0x1414231e, 0x2323231e,
        0x1e1e1e1e, 0x2323231e, 0x23232323, 0x1414231e, 0x14141414, 0x1414231e, 0x14141414,
        0x1e1e1e1e, 0x1e1e1e1e,
    ]);
    mmio::CHARBLOCK0_8BPP.index(35).write([
        0x23232323, 0x1e1e1e1e, 0x14141414, 0x1414231e, 0x14141414, 0x1414231e, 0x1e1e1e1e,
        0x1414231e, 0x1e1e1e1e, 0x1414231e, 0x1414231e, 0x1414231e, 0x1414231e, 0x1414231e,
        0x1414231e, 0x2323231e,
    ]);

    // ../src/assets/graphics/backgrounds/3_mosais_brown.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(36).write([
        0x1f1f1f1f, 0x01010101, 0x0d0d0d0d, 0x0d0d1f01, 0x0d0d0d0d, 0x0d0d1f01, 0x01010101,
        0x0d0d1f01, 0x01010101, 0x0d0d1f01, 0x0d0d1f01, 0x0d0d1f01, 0x0d0d1f01, 0x0d0d1f01,
        0x0d0d1f01, 0x1f1f1f01,
    ]);
    mmio::CHARBLOCK0_8BPP.index(37).write([
        0x0d0d1f01, 0x1f1f1f01, 0x0d0d1f01, 0x0d0d1f01, 0x0d0d1f01, 0x0d0d1f01, 0x1f1f1f01,
        0x01010101, 0x1f1f1f01, 0x1f1f1f1f, 0x0d0d1f01, 0x0d0d0d0d, 0x0d0d1f01, 0x0d0d0d0d,
        0x01010101, 0x01010101,
    ]);
    mmio::CHARBLOCK0_8BPP.index(38).write([
        0x0d0d1f01, 0x1f1f1f01, 0x0d0d1f01, 0x0d0d1f01, 0x0d0d1f01, 0x0d0d1f01, 0x1f1f1f01,
        0x01010101, 0x1f1f1f01, 0x1f1f1f1f, 0x0d0d1f01, 0x0d0d0d0d, 0x0d0d1f01, 0x0d0d0d0d,
        0x01010101, 0x01010101,
    ]);
    mmio::CHARBLOCK0_8BPP.index(39).write([
        0x1f1f1f1f, 0x01010101, 0x0d0d0d0d, 0x0d0d1f01, 0x0d0d0d0d, 0x0d0d1f01, 0x01010101,
        0x0d0d1f01, 0x01010101, 0x0d0d1f01, 0x0d0d1f01, 0x0d0d1f01, 0x0d0d1f01, 0x0d0d1f01,
        0x0d0d1f01, 0x1f1f1f01,
    ]);

    // ../src/assets/graphics/backgrounds/4_mosais_colorfull.png (16x16 pixels) -> (4x16 u32)
    mmio::CHARBLOCK0_8BPP.index(40).write([
        0x22242424, 0x24242314, 0x22242424, 0x24242314, 0x18222222, 0x24242314, 0x16161616,
        0x23231814, 0x14182323, 0x16161616, 0x14232424, 0x22222218, 0x14232424, 0x24242422,
        0x14232424, 0x24242422,
    ]);
    mmio::CHARBLOCK0_8BPP.index(41).write([
        0x24242424, 0x22242424, 0x24242424, 0x22242424, 0x22222222, 0x18222222, 0x16161616,
        0x16161616, 0x16161616, 0x24242314, 0x22222222, 0x24242314, 0x24242424, 0x24242314,
        0x24242424, 0x24242314,
    ]);
    mmio::CHARBLOCK0_8BPP.index(42).write([
        0x14232424, 0x24242424, 0x14232424, 0x24242424, 0x14232424, 0x22222222, 0x14232424,
        0x16161616, 0x24242424, 0x23242424, 0x24242424, 0x23242424, 0x23232323, 0x18232323,
        0x14141414, 0x14141414,
    ]);
    mmio::CHARBLOCK0_8BPP.index(43).write([
        0x22242424, 0x24242314, 0x22242424, 0x24242314, 0x18222222, 0x24242314, 0x16161616,
        0x23231814, 0x14182323, 0x16161616, 0x14232424, 0x22222218, 0x14232424, 0x24242422,
        0x14232424, 0x24242422,
    ]);
}

// ../src/assets/graphics/backgrounds/6_woodflore.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_6_WOODFLORE: usize = 0;
pub const SIZE_6_WOODFLORE: usize = 4;

// ../src/assets/graphics/backgrounds/10_wood1_16.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_10_WOOD1_16: usize = 4;
pub const SIZE_10_WOOD1_16: usize = 4;

// ../src/assets/graphics/backgrounds/7_concrete1_16.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_7_CONCRETE1_16: usize = 8;
pub const SIZE_7_CONCRETE1_16: usize = 4;

// ../src/assets/graphics/backgrounds/9_discofloor_16.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_9_DISCOFLOOR_16: usize = 12;
pub const SIZE_9_DISCOFLOOR_16: usize = 4;

// ../src/assets/graphics/backgrounds/5_rock.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_5_ROCK: usize = 16;
pub const SIZE_5_ROCK: usize = 4;

// ../src/assets/graphics/backgrounds/8_concrete2_16.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_8_CONCRETE2_16: usize = 20;
pub const SIZE_8_CONCRETE2_16: usize = 4;

// ../src/assets/graphics/backgrounds/1_grass.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_1_GRASS: usize = 24;
pub const SIZE_1_GRASS: usize = 4;

// ../src/assets/graphics/backgrounds/11_wood2_16.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_11_WOOD2_16: usize = 28;
pub const SIZE_11_WOOD2_16: usize = 4;

// ../src/assets/graphics/backgrounds/2_mosais_blue.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_2_MOSAIS_BLUE: usize = 32;
pub const SIZE_2_MOSAIS_BLUE: usize = 4;

// ../src/assets/graphics/backgrounds/3_mosais_brown.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_3_MOSAIS_BROWN: usize = 36;
pub const SIZE_3_MOSAIS_BROWN: usize = 4;

// ../src/assets/graphics/backgrounds/4_mosais_colorfull.png (16x16 pixels) -> (4x16 u32)
pub const INDEX_4_MOSAIS_COLORFULL: usize = 40;
pub const SIZE_4_MOSAIS_COLORFULL: usize = 4;
