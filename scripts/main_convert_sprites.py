import numpy as np
import cv2
import os
import utils


def u5(img_u8):
    img_u5 = np.clip((img_u8.astype(int) + 4) >> 3, 0, 31).astype(np.uint8)
    return img_u5


def color15(img):
    img5 = u5(img)

    res = img5[:, :, 0].astype(int) + (img5[:, :, 1].astype(int) << 5) + (img5[:, :, 2].astype(int) << 10)
    res = res.astype(int)
    if img5.shape[2] == 4:
        res[img5[:, :, 3] == 0] = -1
    return res


def extract_colors_palette_15(img):
    palette = img.copy().flatten()
    palette = palette[palette >= 0]
    palette = np.unique(palette[palette >= 0], axis=0)

    return palette


def generate_rust_palette(palette):
    lines = ""
    for i, color in enumerate(palette):
        bin_color = f"{color:015b}"
        bin_color = f"0b0_{bin_color[:5]}_{bin_color[5:10]}_{bin_color[10:15]}"
        lines += f"    mmio::OBJ_PALETTE.index({i + 1}).write(Color({bin_color}));\n"
    return lines


def create_palette(sprites_folder):
    full_palette = set([])
    for filename in os.listdir(sprites_folder):
        img = cv2.imread(os.path.join(sprites_folder, filename), flags=cv2.IMREAD_UNCHANGED)
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


def generate_indimgby4_as_rust_array(name, ind_img_by4, ind):
    """

    :param name: the name that will appear in the rust commented line
    :param ind_img_by4: image of indexes merged 4by4 (4 x u8 = u32)
    :param ind: the starting index for this image.
    :return:the rust lines AND the new index that must be passed to this function next time you use it
    """
    h, w = ind_img_by4.shape
    lines = f"\n\n"
    lines += f"    // {name} ({h}x{w})\n"
    for i in range(0, h, 4):
        for j in range(0, w, 8):

            hex_values = []
            for y in range(4):
                for x in range(2):
                    hex_values.append(f"0x{ind_img_by4[y, x]:08x}")

            lines += f"    mmio::OBJ_TILES.index({ind}).write([{', '.join(hex_values)}]);\n"
            ind += 1
    return lines, ind


def main():
    full_palette = set([])
    sprites_folder = os.path.normpath("../src/assets/graphics/sprites/")
    dst_rust_file = os.path.normpath("../src/sprites.rs")

    palette = create_palette(sprites_folder)

    rust_lines = "use gba::mmio;\n"
    rust_lines += "use gba::video::Color;\n"
    rust_lines += "\n"
    rust_lines += "pub fn load_sprites(){\n"

    rust_lines += generate_rust_palette(palette)

    OBJ_TILES_index = 0
    for filename in os.listdir(sprites_folder):
        img = cv2.imread(os.path.join(sprites_folder, filename), flags=cv2.IMREAD_UNCHANGED)
        img15 = color15(img)
        index_img = img15_to_ind(img15, palette)
        h, w = index_img.shape

        index_img_0 = index_img[:, 0::4].astype(np.uint32)
        index_img_1 = index_img[:, 1::4].astype(np.uint32)
        index_img_2 = index_img[:, 2::4].astype(np.uint32)
        index_img_3 = index_img[:, 3::4].astype(np.uint32)
        index_img_by4 = (index_img_0 << (8 * 3)) + (index_img_1 << (8 * 2)) + (index_img_2 << 8) + index_img_3

        assert h, w // 4 == index_img_by4.shape

        lines, OBJ_TILES_index = generate_indimgby4_as_rust_array(filename, index_img_by4, OBJ_TILES_index)

        rust_lines += lines

    rust_lines += "}\n"

    print("#",dst_rust_file)
    print(rust_lines)
    with open(dst_rust_file, "w") as fio:
        fio.write(rust_lines)

    utils.format_rust_file(dst_rust_file)

if __name__ == '__main__':
    main()
