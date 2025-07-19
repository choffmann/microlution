import numpy as np
import cv2
import os

rows, cols = 4, 6
overlap = 5
img_dir = "img"
img_number = "2"
tile_dir = "tiles_with_overlap"
output_dir = "stitched"

input_path = os.path.join(img_dir, img_number, tile_dir)
output_path = os.path.join(img_dir, img_number, output_dir)
os.makedirs(output_path, exist_ok=True)

tile_heights = []
tile_widths = []

for col in range(cols):
    path = os.path.join(input_path, f"tile_0_{col}_overlap.jpg")
    img = cv2.imread(path)
    if img is None:
        print(f"Warnung: Datei fehlt oder ungültig: {path}")
        tile_widths.append(0)
    else:
        tile_widths.append(img.shape[1])

for row in range(rows):
    path = os.path.join(input_path, f"tile_{row}_0_overlap.jpg")
    img = cv2.imread(path)
    if img is None:
        print(f"Warnung: Datei fehlt oder ungültig: {path}")
        tile_heights.append(0)
    else:
        tile_heights.append(img.shape[0])

stitched_width = sum(tile_widths) - overlap * (cols - 1)
stitched_height = sum(tile_heights) - overlap * (rows - 1)
stitched_image = np.zeros((stitched_height, stitched_width, 3), dtype=np.uint8)

y = 0
for row in range(rows):
    x = 0
    for col in range(cols):
        tile_path = os.path.join(input_path, f"tile_{row}_{col}_overlap.jpg")
        img = cv2.imread(tile_path)
        if img is None:
            print(f"Überspringe fehlende Datei: {tile_path}")
            x += tile_widths[col] - overlap
            continue

        h, w = img.shape[:2]
        stitched_image[y:y+h, x:x+w] = img
        x += w - overlap
    y += tile_heights[row] - overlap

output_file = os.path.join(output_path, "stitched.jpg")
cv2.imwrite(output_file, stitched_image)
print(f"Bild gespeichert unter: {output_file}")

