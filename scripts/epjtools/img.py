from email.mime import image
from PIL import Image
from sys import argv
from os import path as file

def resize_image(image_path: str = "dragon_32x32px.png"):
    if not file.exists(image_path):
        raise FileNotFoundError(f"Image file '{image_path}' not found.")

    img = Image.open(image_path)
    (width, height) = img.size
    print(f"Original image size: {width}x{height}")

    original_name = image_path.split('/')[-1].split('.')[0]
    target_size = original_name.split('_')[-1].replace('px', '').split('x')
    if len(target_size) == 2:
        target_width, target_height = map(int, target_size)
    else:
        target_width, target_height = 32, 32

    print(f"Target size: {target_width}x{target_height}")
    #loop on each interpolation algorithm
    for interpolation in [Image.NEAREST, Image.BILINEAR, Image.BICUBIC, Image.LANCZOS]:
        img = img.resize((target_width, target_height), interpolation)
        img.save(f"resized_{original_name}_{str(interpolation)}.png")

        # create a image with white background with PIL
        img_with_bg = Image.new("RGBA", (target_width, target_height), (255, 255, 255, 255))
        img_with_bg.paste(img, (0, 0), img)
        img_with_bg.save(f"resized_{original_name}_{str(interpolation)}_bg.png")

# take command line arguments
if __name__ == "__main__":
    image_path = "dragon_32x32px.png"
    if len(argv) > 1:
        image_path = argv[1]
    resize_image(image_path)
