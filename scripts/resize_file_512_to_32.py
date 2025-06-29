import os
from PIL import Image

SRC_DIR = 'src/assets/graphics'

def resize_image_to_32(root, filename, src_path):
    name, ext = os.path.splitext(filename)
    out_filename = f"{name}_32.png"
    out_path = os.path.join(root, out_filename)
    try:
        clean_image_edges(src_path)
        with Image.open(src_path) as img:
            img_32 = img.resize((32, 32), Image.LANCZOS)
            img_32.save(out_path)
            clean_image_edges(out_path)
            print(f"Resized {src_path} -> {out_path}")
    except Exception as e:
        print(f"Skipping {src_path}: {e}")




# function used before resize to clean the image crop the image near the the edges
def clean_image_edges(image_path):
    try:
        with Image.open(image_path) as img:
            # Convert to RGBA if not already in that mode
            if img.mode != 'RGBA':
                img = img.convert('RGBA')

            # Get the bounding box of the non-transparent pixels
            bbox = img.getbbox()
            if bbox:
                # Crop the image to the bounding box
                img_cropped = img.crop(bbox)
                return img_cropped
            else:
                print(f"No non-transparent pixels found in {image_path}.")
                return img  # Return original image if no crop is needed
    except Exception as e:
        print(f"Error processing {image_path}: {e}")
        return None

for root, _, files in os.walk(SRC_DIR):
    for filename in files:
        src_path = os.path.join(root, filename)

        if not os.path.isfile(src_path):
            continue
        resize_image_to_32(root, filename, src_path)