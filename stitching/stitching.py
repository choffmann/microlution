import numpy as np
import cv2
import os

rows, cols = 4, 4

img_dir = "img"
img_number = "5"
tile_dir = "tiles"
output_dir = "stitched"

tile_heights = []
tile_widths = []

input_path = os.path.join(img_dir, img_number, tile_dir)
output_path = os.path.join(img_dir, img_number, output_dir)
os.makedirs(output_path, exist_ok=True)

for col in range(cols):
    path = os.path.join(input_path, f"tile_{col}_0.jpeg")
    img = cv2.imread(path)
    if img is None:
        print(f"Datei nicht gefunden: {path}")
        tile_widths.append(0)
    else:
        tile_widths.append(img.shape[1])

for row in range(rows):
    path = os.path.join(input_path, f"tile_0_{row}.jpeg")
    img = cv2.imread(path)
    if img is None:
        print(f"Datei nicht gefunden: {path}")
        tile_heights.append(0)
    else:
        tile_heights.append(img.shape[0])

stitched_width = sum(tile_widths)
stitched_height = sum(tile_heights)
stitched_image = np.zeros((stitched_height, stitched_width, 3), dtype=np.uint8)

y = 0
for row in range(rows):
    x = 0
    for col in range(cols):
        tile_path = os.path.join(input_path, f"tile_{col}_{row}.jpeg")
        img = cv2.imread(tile_path)
        if img is None:
            print(f"Überspringe fehlende Datei: {tile_path}")
            x += tile_widths[col]
            continue

        h, w = img.shape[:2]
        stitched_image[y:y+h, x:x+w] = img
        x += w
    y += tile_heights[row]

output_file = os.path.join(output_path, "stitched.jpg")
cv2.imwrite(output_file, stitched_image)
print(f"Bild gespeichert unter: {output_file}")

