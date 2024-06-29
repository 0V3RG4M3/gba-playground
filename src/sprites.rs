use gba::mmio;
use gba::video::Color;

pub fn load_sprites() {
    mmio::OBJ_PALETTE.index(1).write(Color(0b0_00110_01000_00111));
    mmio::OBJ_PALETTE.index(2).write(Color(0b0_00100_00100_00111));
    mmio::OBJ_PALETTE.index(3).write(Color(0b0_11011_10100_01101));
    mmio::OBJ_PALETTE.index(4).write(Color(0b0_01000_01000_01111));
    mmio::OBJ_PALETTE.index(5).write(Color(0b0_00110_01100_10000));
    mmio::OBJ_PALETTE.index(6).write(Color(0b0_10001_10000_10001));
    mmio::OBJ_PALETTE.index(7).write(Color(0b0_01111_01000_10001));
    mmio::OBJ_PALETTE.index(8).write(Color(0b0_11110_11000_10011));
    mmio::OBJ_PALETTE.index(9).write(Color(0b0_01010_01001_00101));
    mmio::OBJ_PALETTE.index(10).write(Color(0b0_01001_01101_00110));
    mmio::OBJ_PALETTE.index(11).write(Color(0b0_01001_00101_01000));
    mmio::OBJ_PALETTE.index(12).write(Color(0b0_01101_01101_01101));
    mmio::OBJ_PALETTE.index(13).write(Color(0b0_11001_11011_11111));
    mmio::OBJ_PALETTE.index(14).write(Color(0b0_11100_01110_00101));
    mmio::OBJ_PALETTE.index(15).write(Color(0b0_10001_01110_00110));
    mmio::OBJ_PALETTE.index(16).write(Color(0b0_11111_11110_00111));
    mmio::OBJ_PALETTE.index(17).write(Color(0b0_10110_00110_00110));
    mmio::OBJ_PALETTE.index(18).write(Color(0b0_10011_10110_10111));
    mmio::OBJ_PALETTE.index(19).write(Color(0b0_01011_01110_11100));
    mmio::OBJ_PALETTE.index(20).write(Color(0b0_01101_00111_00110));
    mmio::OBJ_PALETTE.index(21).write(Color(0b0_10010_01011_00111));
    mmio::OBJ_PALETTE.index(22).write(Color(0b0_10010_10011_01001));
    mmio::OBJ_PALETTE.index(23).write(Color(0b0_01011_01011_01010));
    mmio::OBJ_PALETTE.index(24).write(Color(0b0_11011_01011_01100));
    mmio::OBJ_PALETTE.index(25).write(Color(0b0_11011_01111_10111));
    mmio::OBJ_PALETTE.index(26).write(Color(0b0_11111_11111_11111));
    mmio::OBJ_PALETTE.index(27).write(Color(0b0_01100_10011_11111));

    // ../src/assets/graphics/backgrounds/grass.png (16x4)
    mmio::OBJ_TILES.index(0).write([
        0x060c0c0c, 0x06160a0c, 0x170a0909, 0x0a0a0a0a, 0x0a0c0a09, 0x0a0a0a01, 0x0a0a0a0a,
        0x0a090901,
    ]);
    mmio::OBJ_TILES.index(1).write([
        0x060c0c0c, 0x06160a0c, 0x170a0909, 0x0a0a0a0a, 0x0a0c0a09, 0x0a0a0a01, 0x0a0a0a0a,
        0x0a090901,
    ]);
    mmio::OBJ_TILES.index(2).write([
        0x060c0c0c, 0x06160a0c, 0x170a0909, 0x0a0a0a0a, 0x0a0c0a09, 0x0a0a0a01, 0x0a0a0a0a,
        0x0a090901,
    ]);
    mmio::OBJ_TILES.index(3).write([
        0x060c0c0c, 0x06160a0c, 0x170a0909, 0x0a0a0a0a, 0x0a0c0a09, 0x0a0a0a01, 0x0a0a0a0a,
        0x0a090901,
    ]);

    // ../src/assets/graphics/backgrounds/rock.png (64x16)
    mmio::OBJ_TILES.index(4).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(5).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(6).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(7).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(8).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(9).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(10).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(11).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(12).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(13).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(14).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(15).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(16).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(17).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(18).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(19).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(20).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(21).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(22).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(23).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(24).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(25).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(26).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(27).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(28).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(29).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(30).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(31).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(32).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(33).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(34).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);
    mmio::OBJ_TILES.index(35).write([
        0x0c0c0c0c, 0x17170909, 0x0c0c1717, 0x17090917, 0x0c0c1717, 0x17171717, 0x0c0c0c17,
        0x170c0c17,
    ]);

    // ../src/assets/graphics/backgrounds/woodflore.png (16x4)
    mmio::OBJ_TILES.index(36).write([
        0x14141414, 0x14141414, 0x15151515, 0x15151515, 0x15151515, 0x15151515, 0x15151515,
        0x15151515,
    ]);
    mmio::OBJ_TILES.index(37).write([
        0x14141414, 0x14141414, 0x15151515, 0x15151515, 0x15151515, 0x15151515, 0x15151515,
        0x15151515,
    ]);
    mmio::OBJ_TILES.index(38).write([
        0x14141414, 0x14141414, 0x15151515, 0x15151515, 0x15151515, 0x15151515, 0x15151515,
        0x15151515,
    ]);
    mmio::OBJ_TILES.index(39).write([
        0x14141414, 0x14141414, 0x15151515, 0x15151515, 0x15151515, 0x15151515, 0x15151515,
        0x15151515,
    ]);

    // ../src/assets/graphics/sprites/banana.png (16x8)
    mmio::OBJ_TILES.index(40).write([
        0x00000000, 0x00000000, 0x00000002, 0x02000000, 0x00000002, 0x02020200, 0x00000000,
        0x02021002,
    ]);
    mmio::OBJ_TILES.index(41).write([
        0x00000000, 0x00000000, 0x00000002, 0x02000000, 0x00000002, 0x02020200, 0x00000000,
        0x02021002,
    ]);
    mmio::OBJ_TILES.index(42).write([
        0x00000000, 0x00000000, 0x00000002, 0x02000000, 0x00000002, 0x02020200, 0x00000000,
        0x02021002,
    ]);
    mmio::OBJ_TILES.index(43).write([
        0x00000000, 0x00000000, 0x00000002, 0x02000000, 0x00000002, 0x02020200, 0x00000000,
        0x02021002,
    ]);

    // ../src/assets/graphics/sprites/snail-back.png (32x4)
    mmio::OBJ_TILES.index(44).write([
        0x00000002, 0x02000000, 0x00000202, 0x02020000, 0x0002021a, 0x040b0200, 0x00020c04,
        0x040b0200,
    ]);
    mmio::OBJ_TILES.index(45).write([
        0x00000002, 0x02000000, 0x00000202, 0x02020000, 0x0002021a, 0x040b0200, 0x00020c04,
        0x040b0200,
    ]);
    mmio::OBJ_TILES.index(46).write([
        0x00000002, 0x02000000, 0x00000202, 0x02020000, 0x0002021a, 0x040b0200, 0x00020c04,
        0x040b0200,
    ]);
    mmio::OBJ_TILES.index(47).write([
        0x00000002, 0x02000000, 0x00000202, 0x02020000, 0x0002021a, 0x040b0200, 0x00020c04,
        0x040b0200,
    ]);
    mmio::OBJ_TILES.index(48).write([
        0x00000002, 0x02000000, 0x00000202, 0x02020000, 0x0002021a, 0x040b0200, 0x00020c04,
        0x040b0200,
    ]);
    mmio::OBJ_TILES.index(49).write([
        0x00000002, 0x02000000, 0x00000202, 0x02020000, 0x0002021a, 0x040b0200, 0x00020c04,
        0x040b0200,
    ]);
    mmio::OBJ_TILES.index(50).write([
        0x00000002, 0x02000000, 0x00000202, 0x02020000, 0x0002021a, 0x040b0200, 0x00020c04,
        0x040b0200,
    ]);
    mmio::OBJ_TILES.index(51).write([
        0x00000002, 0x02000000, 0x00000202, 0x02020000, 0x0002021a, 0x040b0200, 0x00020c04,
        0x040b0200,
    ]);
}
