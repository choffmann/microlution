import numpy as np
import cv2
import os

rows, cols = 3, 3
overlap_x = 0
overlap_y = 233
img_dir = "img"
img_number = "4"
tile_dir = "tiles"
output_dir = "stitched"
save_grayscale = True
blend_alpha = 0.5

input_path = os.path.join(img_dir, img_number, tile_dir)
output_path = os.path.join(img_dir, img_number, output_dir)
os.makedirs(output_path, exist_ok=True)

tile_widths, tile_heights = [], []

for col in range(cols):
    img = cv2.imread(os.path.join(input_path, f"tile_0_{col}_overlap.jpg"))
    tile_widths.append(img.shape[1])

for row in range(rows):
    img = cv2.imread(os.path.join(input_path, f"tile_{row}_0_overlap.jpg"))
    tile_heights.append(img.shape[0])

stitched_width = sum(tile_widths) - overlap_x * (cols - 1)
stitched_height = sum(tile_heights) - overlap_y * (rows - 1)
stitched_image = np.zeros((stitched_height, stitched_width, 3), dtype=np.uint8)

y = 0
for row in range(rows):
    x = 0
    for col in range(cols):
        tile_path = os.path.join(input_path, f"tile_{row}_{col}_overlap.jpg")
        img = cv2.imread(tile_path)
        h, w = img.shape[:2]

        for i in range(h):
            for j in range(w):
                target_y = y + i
                target_x = x + j

                if np.any(stitched_image[target_y, target_x] != 0):
                    stitched_image[target_y, target_x] = cv2.addWeighted(
    stitched_image[target_y, target_x].astype(np.float32), 1 - blend_alpha,
    img[i, j].astype(np.float32), blend_alpha, 0
).astype(np.uint8).reshape(3,)
                else:
                    stitched_image[target_y, target_x] = img[i, j]

        x += w - overlap_x
    y += tile_heights[row] - overlap_y

output_file_color = os.path.join(output_path, "stitched_transparent_overlap.jpg")
cv2.imwrite(output_file_color, stitched_image)
print(f"Farbbild gespeichert unter {output_file_color}")

if save_grayscale:
    stitched_gray = cv2.cvtColor(stitched_image, cv2.COLOR_BGR2GRAY)
    output_file_gray = os.path.join(output_path, "stitched_transparent_overlap_gray.jpg")
    cv2.imwrite(output_file_gray, stitched_gray)
    print(f"Schwarz-Weiß-Bild gespeichert unter {output_file_gray}")
