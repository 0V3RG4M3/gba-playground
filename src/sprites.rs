use gba::mmio;
use gba::video::Color;

pub fn load_sprites() {
    mmio::OBJ_PALETTE.index(1).write(Color(0b0_00000_00010_00011));
    mmio::OBJ_PALETTE.index(2).write(Color(0b0_00000_00010_00110));
    mmio::OBJ_PALETTE.index(3).write(Color(0b0_00000_00010_00111));
    mmio::OBJ_PALETTE.index(4).write(Color(0b0_00000_00110_01010));
    mmio::OBJ_PALETTE.index(5).write(Color(0b0_00000_00000_10000));
    mmio::OBJ_PALETTE.index(6).write(Color(0b0_00000_00010_10001));
    mmio::OBJ_PALETTE.index(7).write(Color(0b0_00000_00000_10010));
    mmio::OBJ_PALETTE.index(8).write(Color(0b0_00000_00010_10100));
    mmio::OBJ_PALETTE.index(9).write(Color(0b0_00000_00010_10111));
    mmio::OBJ_PALETTE.index(10).write(Color(0b0_00000_00000_11010));
    mmio::OBJ_PALETTE.index(11).write(Color(0b0_00000_00010_11100));
    mmio::OBJ_PALETTE.index(12).write(Color(0b0_00000_00010_11111));
    mmio::OBJ_PALETTE.index(13).write(Color(0b0_00000_00110_11111));
    mmio::OBJ_PALETTE.index(14).write(Color(0b0_00000_00101_00001));
    mmio::OBJ_PALETTE.index(15).write(Color(0b0_00000_00011_00110));
    mmio::OBJ_PALETTE.index(16).write(Color(0b0_00000_00101_01010));
    mmio::OBJ_PALETTE.index(17).write(Color(0b0_00000_00011_10010));
    mmio::OBJ_PALETTE.index(18).write(Color(0b0_00000_00001_10100));
    mmio::OBJ_PALETTE.index(19).write(Color(0b0_00000_00011_10110));
    mmio::OBJ_PALETTE.index(20).write(Color(0b0_00000_00111_11010));
    mmio::OBJ_PALETTE.index(21).write(Color(0b0_00000_00111_11011));
    mmio::OBJ_PALETTE.index(22).write(Color(0b0_00000_00001_11100));

    // banana.png (16x8)
    mmio::OBJ_TILES.index(0).write([
        0x00000000, 0x00000000, 0x00000012, 0x12000000, 0x00000012, 0x12121200, 0x00000000,
        0x12121312,
    ]);
    mmio::OBJ_TILES.index(1).write([
        0x00000000, 0x00000000, 0x00000012, 0x12000000, 0x00000012, 0x12121200, 0x00000000,
        0x12121312,
    ]);
    mmio::OBJ_TILES.index(2).write([
        0x00000000, 0x00000000, 0x00000012, 0x12000000, 0x00000012, 0x12121200, 0x00000000,
        0x12121312,
    ]);
    mmio::OBJ_TILES.index(3).write([
        0x00000000, 0x00000000, 0x00000012, 0x12000000, 0x00000012, 0x12121200, 0x00000000,
        0x12121312,
    ]);

    // snail-back.png (32x4)
    mmio::OBJ_TILES.index(4).write([
        0x00000012, 0x12000000, 0x00001212, 0x12120000, 0x0012120d, 0x08161200, 0x00121008,
        0x08161200,
    ]);
    mmio::OBJ_TILES.index(5).write([
        0x00000012, 0x12000000, 0x00001212, 0x12120000, 0x0012120d, 0x08161200, 0x00121008,
        0x08161200,
    ]);
    mmio::OBJ_TILES.index(6).write([
        0x00000012, 0x12000000, 0x00001212, 0x12120000, 0x0012120d, 0x08161200, 0x00121008,
        0x08161200,
    ]);
    mmio::OBJ_TILES.index(7).write([
        0x00000012, 0x12000000, 0x00001212, 0x12120000, 0x0012120d, 0x08161200, 0x00121008,
        0x08161200,
    ]);
    mmio::OBJ_TILES.index(8).write([
        0x00000012, 0x12000000, 0x00001212, 0x12120000, 0x0012120d, 0x08161200, 0x00121008,
        0x08161200,
    ]);
    mmio::OBJ_TILES.index(9).write([
        0x00000012, 0x12000000, 0x00001212, 0x12120000, 0x0012120d, 0x08161200, 0x00121008,
        0x08161200,
    ]);
    mmio::OBJ_TILES.index(10).write([
        0x00000012, 0x12000000, 0x00001212, 0x12120000, 0x0012120d, 0x08161200, 0x00121008,
        0x08161200,
    ]);
    mmio::OBJ_TILES.index(11).write([
        0x00000012, 0x12000000, 0x00001212, 0x12120000, 0x0012120d, 0x08161200, 0x00121008,
        0x08161200,
    ]);
}
