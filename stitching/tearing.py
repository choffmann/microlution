from PIL import Image
import os

img_dir = "img"
img_number = "1"
image_path = f"{img_dir}/{img_number}/original.jpg"
output_dir = "tiles"
cols, rows = 6, 4

img = Image.open(image_path)
width, height = img.size

tile_width = width // cols
tile_height = height // rows

os.makedirs(f"{img_dir}/{img_number}/{output_dir}", exist_ok=True)

for row in range(rows):
    for col in range(cols):
        left = col * tile_width
        upper = row * tile_height
        right = left + tile_width
        lower = upper + tile_height

        tile = img.crop((left, upper, right, lower))
        tile_name = f"tile_{row}_{col}.jpg"
        tile.save(os.path.join(f"{img_dir}/{img_number}/{output_dir}", tile_name))

print("Kacheln wurden gespeichert in:", output_dir)

