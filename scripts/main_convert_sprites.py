import numpy as np
import cv2
import os
import utils


def u5(img_u8):
    img_u5 = np.clip((img_u8.astype(int) + 4) >> 3, 0, 31).astype(np.uint8)
    return img_u5


def color15(img):
    img5 = u5(img)

    res = img5[:, :, 2].astype(int) + (img5[:, :, 1].astype(int) << 5) + (img5[:, :, 0].astype(int) << 10)
    res = res.astype(int)
    if img5.shape[2] == 4:
        res[img5[:, :, 3] == 0] = -1
    return res


def extract_colors_palette_15(img):
    palette = img.copy().flatten()
    palette = palette[palette >= 0]
    palette = np.unique(palette[palette >= 0], axis=0)

    return palette


def generate_rust_palette(palette, palette_register):
    """

    :param palette: list of colors encoded on 15 bits
    :param palette_register: typically "OBJ_TILES" or "CHARBLOCK0_8BPP"
    :return:
    """
    lines = ""
    for i, color in enumerate(palette):
        bin_color = f"{color:015b}"
        bin_color = f"0b0_{bin_color[:5]}_{bin_color[5:10]}_{bin_color[10:15]}"
        lines += f"    mmio::{palette_register}.index({i + 1}).write(Color({bin_color}));\n"
    return lines


def _test_generate_rust_palette():
    palette = [32 ** 0, 32 ** 1, 32 ** 2]
    palette_register = "TEST_REGISTER"
    expected = """    mmio::TEST_REGISTER.index(1).write(Color(0b0_00000_00000_00001));
    mmio::TEST_REGISTER.index(2).write(Color(0b0_00000_00001_00000));
    mmio::TEST_REGISTER.index(3).write(Color(0b0_00001_00000_00000));
"""
    result = generate_rust_palette(palette, palette_register)
    assert result == expected


def create_palette(png_files: list[str]):
    full_palette = set([])
    for filename in png_files:
        img = cv2.imread(filename, flags=cv2.IMREAD_UNCHANGED)
        img = color15(img)
        h, w = img.shape
        assert h % 4 == 0, "height must be a multiple of 4"
        assert w % 8 == 0, "width must be a multiple of 8"
        palette = extract_colors_palette_15(img)
        full_palette = full_palette.union(set(palette.tolist()))

    return list(full_palette)


def img15_to_ind(img15, palette):
    index_img = np.zeros_like(img15, dtype=np.uint8)
    for i, color15 in enumerate(palette):
        ind = i + 1
        index_img[img15 == color15] = ind

    return index_img


def generate_indimgby4_as_rust_array(filename, index_img_by4, block_register, block_width_u8, block_height_u8,
                                     block_register_index):
    """
    :param filename: this filename is only used to define the name of the constant in the rust code
    :param index_img_by4:  image of indexes merged 4by4 (4 x u8 = u32)
    :param block_register: the name of the register (typically "OBJ_TILES" or "CHARBLOCK0_8BPP")
    :param block_width_u8: width of the block (typically 8)
    :param block_height_u8: height of the block (typically 4 or 8)
    :param block_register_index: the starting index for this image.
    :return: tuple:
        - the rust lines
        - the new block_register_index that you must pass to this function next time you use it
    """
    # fix Windows path

    name = os.path.split(filename)[-1]
    assert name.endswith(".png")
    name = name[:-4]  # remove extension
    name = name.upper().replace("-", "_")  # format for const name in rust

    filename = filename.replace('\\', '/')

    im_height_u8, im_width_u32 = index_img_by4.shape
    im_width_u8 = 4 * im_width_u32  # pixels are grouped by 4
    block_width_u32 = block_width_u8 // 4

    lines_func = "\n\n"
    line_count = im_height_u8 * im_width_u8 // (block_height_u8 * block_width_u8)
    line_length = block_height_u8 * block_width_u32
    comment_line = f"// {filename} ({im_height_u8}x{im_width_u8} pixels) -> ({line_count}x{line_length} u32)\n"
    lines_func += f"    {comment_line}"

    lines_const = "\n"
    lines_const += f"{comment_line}"
    lines_const += f"pub const INDEX_{name}: usize = {block_register_index};\n"
    lines_const += f"pub const SIZE_{name}: usize = {line_count};\n"

    assert (
                   block_register_index + line_count) < 1024, f"Registers can hold max 1024 elements, got {block_register_index + line_count}"

    for i0 in range(0, im_height_u8, 8):
        for j in range(0, im_width_u32, block_width_u32):
            for i1 in range(0, 8, block_height_u8):
                i = i0 + i1
                hex_values = []
                print(i, j)
                for y in range(block_height_u8):
                    for x in range(block_width_u32):
                        hex_values.append(f"0x{index_img_by4[i + y, j + x]:08x}")

                lines_func += f"    mmio::{block_register}.index({block_register_index}).write([{', '.join(hex_values)}]);\n"
                block_register_index += 1
    return lines_func, lines_const, block_register_index


def _test_generate_indimgby4_as_rust_array_1():
    ind_img_by4 = np.zeros((8, 16 // 4), dtype=np.uint32)
    ind = 10
    bw = 8
    bh = 4

    result_lines_func, result_lines_const, result_ind = generate_indimgby4_as_rust_array(
        filename="Test//test.png",
        index_img_by4=ind_img_by4,
        block_register="MY_TEST_REGISTER",
        block_width_u8=bw,
        block_height_u8=bh,
        block_register_index=ind,
    )
    expected_lines_func = """

    // Test//test.png (8x16 pixels) -> (4x8 u32)
    mmio::MY_TEST_REGISTER.index(10).write([0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000]);
    mmio::MY_TEST_REGISTER.index(11).write([0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000]);
    mmio::MY_TEST_REGISTER.index(12).write([0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000]);
    mmio::MY_TEST_REGISTER.index(13).write([0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000]);
"""

    expected_lines_const = """
// Test//test.png (8x16 pixels) -> (4x8 u32)
pub const INDEX_TEST: usize = 10;
pub const SIZE_TEST: usize = 4;
"""
    expected_ind = ind + (ind_img_by4.size * 4) // (bw * bh)
    assert expected_lines_func == result_lines_func
    assert expected_lines_const == result_lines_const
    assert expected_ind == result_ind


def _test_generate_indimgby4_as_rust_array_2():
    ind_img_by4 = np.zeros((8, 16 // 4), dtype=np.uint32)
    ind = 20
    bw = 8
    bh = 8

    result_lines_func, result_lines_const, result_ind = generate_indimgby4_as_rust_array(
        filename="Test//test2.png",
        index_img_by4=ind_img_by4,
        block_register="MY_TEST_REGISTER_2",
        block_width_u8=bw,
        block_height_u8=bh,
        block_register_index=ind,
    )
    expected_lines_func = """

    // Test//test2.png (8x16 pixels) -> (2x16 u32)
    mmio::MY_TEST_REGISTER_2.index(20).write([0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000]);
    mmio::MY_TEST_REGISTER_2.index(21).write([0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000]);
"""

    expected_lines_const = """
// Test//test2.png (8x16 pixels) -> (2x16 u32)
pub const INDEX_TEST2: usize = 20;
pub const SIZE_TEST2: usize = 2;
"""
    expected_ind = ind + (ind_img_by4.size * 4) // (bw * bh)
    assert expected_lines_func == result_lines_func
    assert expected_lines_const == result_lines_const
    assert expected_ind == result_ind


def convert_sprites(folder_path, palette_register, block_register, block_width, block_height):

    dst_rust_file, png_files = find_and_define_rs_file(folder_path)

    palette = create_palette(png_files)

    rust_lines = "// This file has been automatically generated\n"
    rust_lines += "#![allow(dead_code)]\n"
    rust_lines += "use gba::mmio;\n"
    rust_lines += "use gba::video::Color;\n"
    rust_lines += "\n"
    rust_lines += "pub fn load(){\n"

    rust_lines += generate_rust_palette(palette, palette_register)

    rust_lines_const = ""

    block_register_index = 0
    for filename in png_files:
        img = cv2.imread(filename, flags=cv2.IMREAD_UNCHANGED)
        img15 = color15(img)
        index_img = img15_to_ind(img15, palette)
        h, w = index_img.shape

        # concat 4x u8 pixels in 1x u32
        index_img_0 = index_img[:, 3::4].astype(np.uint32)
        index_img_1 = index_img[:, 2::4].astype(np.uint32)
        index_img_2 = index_img[:, 1::4].astype(np.uint32)
        index_img_3 = index_img[:, 0::4].astype(np.uint32)
        index_img_by4 = (index_img_0 << (8 * 3)) + (index_img_1 << (8 * 2)) + (index_img_2 << 8) + index_img_3

        assert h, w // 4 == index_img_by4.shape

        lines_func, lines_const, block_register_index = generate_indimgby4_as_rust_array(filename, index_img_by4,
                                                                                         block_register, block_width,
                                                                                         block_height,
                                                                                         block_register_index)

        rust_lines += lines_func
        rust_lines_const += lines_const
    rust_lines += "}\n"

    rust_lines += rust_lines_const

    print("#", dst_rust_file)
    print(rust_lines)
    with open(dst_rust_file, "w") as fio:
        fio.write(rust_lines)

    utils.format_rust_file(dst_rust_file)


def convert_backgrounds(folder_path, palette_register, block_register, block_width_u8, block_height_u8):
    dst_rust_file, png_files = find_and_define_rs_file(folder_path)

    palette = create_palette(png_files)

    header = "// This file has been automatically generated\n"
    header += "use gba::mmio;\n"
    header += "use gba::video::{Color, TextEntry};\n"
    header += "\n"

    # palette sub-function
    palette_fn = "fn load_palette() {\n"
    palette_fn += generate_rust_palette(palette, palette_register)
    palette_fn += "}\n\n"

    # one sub-function per image
    sub_fn_names = []
    sub_fns = ""

    block_register_indices = [0, 0, 0]
    charblock_index = 0
    screenblock_index = 24

    for filename in png_files:
        img = cv2.imread(filename, flags=cv2.IMREAD_UNCHANGED)
        img15 = color15(img)
        index_img = img15_to_ind(img15, palette)
        h, w = index_img.shape

        name = os.path.split(filename)[-1]
        assert name.endswith(".png")
        name = name[:-4]  # remove extension
        name = name.lower().replace("-", "_")  # snake_case for fn name

        # fix Windows path
        filename = filename.replace('\\', '/')

        # concat 4x u8 pixels in 1x u32
        index_img_0 = index_img[:, 3::4].astype(np.uint32)
        index_img_1 = index_img[:, 2::4].astype(np.uint32)
        index_img_2 = index_img[:, 1::4].astype(np.uint32)
        index_img_3 = index_img[:, 0::4].astype(np.uint32)
        index_img_by4 = (index_img_0 << (8 * 3)) + (index_img_1 << (8 * 2)) + (index_img_2 << 8) + index_img_3

        assert h, w // 4 == index_img_by4.shape

        im_height_u8, im_width_u32 = index_img_by4.shape
        im_width_u8 = 4 * im_width_u32  # pixels are grouped by 4
        block_width_u32 = block_width_u8 // 4

        line_count = im_height_u8 * im_width_u8 // (block_height_u8 * block_width_u8)
        line_length = block_height_u8 * block_width_u32
        comment_line = f"// {filename} ({im_height_u8}x{im_width_u8} pixels) -> ({line_count}x{line_length} u32)\n"

        fn_name = f"load_{name}"
        sub_fn_names.append(fn_name)
        lines_func = f"fn {fn_name}() {{\n"
        lines_func += f"    {comment_line}"

        blocks = set()
        for i0 in range(0, im_height_u8, 8):
            for j in range(0, im_width_u32, block_width_u32):
                for i1 in range(0, 8, block_height_u8):
                    i = i0 + i1
                    hex_values = []
                    for y in range(block_height_u8):
                        for x in range(block_width_u32):
                            hex_values.append(f"0x{index_img_by4[i + y, j + x]:08x}")
                    block = ', '.join(hex_values)
                    blocks.add(block)

        print(len(blocks))
        blocks = list(blocks)
        blocks.sort()

        for block in blocks:
            block_register_index = block_register_indices[charblock_index]
            lines_func += f"    mmio::CHARBLOCK{charblock_index}_8BPP.index({block_register_index}).write([{block}]);\n"
            block_register_indices[charblock_index] += 1

        sb_w = (im_width_u8 + 7) // 8
        sb_h = (im_height_u8 + 7) // 8
        sb_c_w = (sb_w + 31) // 32
        sb_c_h = (sb_h + 31) // 32
        for i0 in range(0, im_height_u8, 8):
            for j in range(0, im_width_u32, block_width_u32):
                for i1 in range(0, 8, block_height_u8):
                    i = i0 + i1
                    hex_values = []
                    for y in range(block_height_u8):
                        for x in range(block_width_u32):
                            hex_values.append(f"0x{index_img_by4[i + y, j + x]:08x}")
                    block = ', '.join(hex_values)
                    index = blocks.index(block)

                    x2 = (j // 2) % 32
                    y2 = (i // 8) % 32
                    i = screenblock_index + ((j // 2) // 32) + ((i // 8) // 32)
                    print(x2, y2, i)
                    lines_func += f"    mmio::TEXT_SCREENBLOCKS.get_frame({i}).unwrap().index({x2}, {y2}).write(TextEntry::new().with_tile({index}));\n"

        lines_func += "}\n\n"
        sub_fns += lines_func

        charblock_index = (charblock_index + 1) % 3
        screenblock_index += sb_c_w * sb_c_h

    # public entry point calling all sub-functions
    load_fn = "pub fn load() {\n"
    load_fn += "    load_palette();\n"
    for fn_name in sub_fn_names:
        load_fn += f"    {fn_name}();\n"
    load_fn += "}\n\n"

    rust_lines = header + load_fn + palette_fn + sub_fns

    print("#", dst_rust_file)
    with open(dst_rust_file, "w") as fio:
        fio.write(rust_lines)

    utils.format_rust_file(dst_rust_file)


def find_and_define_rs_file(folder_path):
    # find png files in folder_path and define the destination rust file.
    foldername = os.path.split(folder_path)[-1]
    png_files = utils.find_all_by_extension(folder_path, ".png")

    # Sort the files by name to ensure consistent order
    png_files.sort()

    # ignore files starting with "_"
    png_files = [f for f in png_files if not os.path.split(f)[-1].startswith("_")]
    dst_rust_file = os.path.normpath(f"../src/{foldername}.rs")
    return dst_rust_file, png_files


def main_test():
    _test_generate_rust_palette()
    _test_generate_indimgby4_as_rust_array_1()
    _test_generate_indimgby4_as_rust_array_2()


def convert_screens(folder_path):
    linesize = 8
    dst_rust_file, png_files = find_and_define_rs_file(folder_path)

    rust_lines = "// This file has been automatically generated\n"
    rust_lines += "#![allow(dead_code)]\n"
    rust_lines += "\n"
    rust_lines += "use gba::video::Video3Bitmap;\n"
    rust_lines += "\n"

    for filename in png_files:
        img = cv2.imread(filename, flags=cv2.IMREAD_UNCHANGED)
        img15 = color15(img)

        name = os.path.split(filename)[-1].split('.')[0].split('_')[1]
        rust_lines += "pub const SCREEN_%s: Video3Bitmap = Video3Bitmap::new_from_u16([\n" % name.upper()
        for vindex, vpixel in enumerate(img15, 0):
            for index, pixel in enumerate(vpixel, 0):
                if pixel == -1:
                    pixel = 0xFFFF
                rust_lines += "0x%x" % pixel
                if not ((vindex == len(img15) - 1) and (index == len(vpixel) - 1)):
                    rust_lines += ', '
                if (index % linesize) == 0 and (index > 0):
                    rust_lines += '\n\t\t'
        rust_lines += '\n'
        rust_lines += '\t]);\n\n'
    print("#", dst_rust_file)
    print(rust_lines)
    with open(dst_rust_file, "w") as fio:
        fio.write(rust_lines)

    utils.format_rust_file(dst_rust_file)

def main(sprites_location, background_location, screen_location):
    convert_sprites(
        folder_path=os.path.normpath(sprites_location),
        palette_register="OBJ_PALETTE",
        block_register="OBJ_TILES",
        block_width=8,
        block_height=4,
    )

    convert_backgrounds(
        folder_path=os.path.normpath(background_location),
        palette_register="BG_PALETTE",
        block_register="CHARBLOCK0_8BPP",
        block_width_u8=8,
        block_height_u8=8,
    )

    convert_screens(
        folder_path=os.path.normpath(screen_location)
    )

if __name__ == '__main__':
    main_test()

    main(
        sprites_location="../src/assets/graphics/sprites",
        background_location="../src/assets/graphics/backgrounds",
        screen_location="../src/assets/jukebox/screens"
    )
    
