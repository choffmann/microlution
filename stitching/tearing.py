from PIL import Image
import os

img_dir = "img"
img_number = "2"
image_path = f"{img_dir}/{img_number}/original.jpg"
output_dir = "tiles_with_overlap" 
cols, rows = 6, 4                 
overlap = 5                       

img = Image.open(image_path)
width, height = img.size

tile_width = width // cols
tile_height = height // rows

os.makedirs(f"{img_dir}/{img_number}/{output_dir}", exist_ok=True)

for row in range(rows):
    for col in range(cols):
        left = max(col * tile_width - overlap, 0)
        upper = max(row * tile_height - overlap, 0)
        right = min((col + 1) * tile_width + overlap, width)
        lower = min((row + 1) * tile_height + overlap, height)

        tile = img.crop((left, upper, right, lower))
        tile_name = f"tile_{row}_{col}_overlap.jpg"
        tile.save(os.path.join(f"{img_dir}/{img_number}/{output_dir}", tile_name))

print("Alle Kacheln wurden gespeichert in:", output_dir)
