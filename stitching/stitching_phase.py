import cv2
import numpy as np
import os

img_dir = "img"
img_number = "1"
tile_dir = "tiles"
output_dir = "stitched"
rows, cols = 4, 6
overlap = 40

tile_w, tile_h = None, None

input_path = os.path.join(img_dir, img_number, tile_dir)
output_path = os.path.join(img_dir, img_number, output_dir)
os.makedirs(output_path, exist_ok=True)

def normalize(img):
    return (img - np.mean(img)) / (np.std(img) + 1e-5)
# Alle Tiles laden
tiles = {}
for row in range(rows):
    for col in range(cols):
        tile_path = os.path.join(input_path, f"tile_{row}_{col}_overlap.jpg")
        img = cv2.imread(tile_path)
        if tile_w is None:
            tile_h, tile_w = img.shape[:2]
        tiles[(row, col)] = img

positions = {(0, 0): (0.0, 0.0)} 
for row in range(rows):
    for col in range(cols):
        if (row, col) == (0, 0):
            continue

        if col > 0:
            left_tile = tiles[(row, col - 1)]
            current_tile = tiles[(row, col)]

            patch_left = left_tile[:, -overlap:]
            patch_right = current_tile[:, :overlap]

            gray_left = normalize(cv2.cvtColor(patch_left, cv2.COLOR_BGR2GRAY).astype(np.float32))
            gray_right = normalize(cv2.cvtColor(patch_right, cv2.COLOR_BGR2GRAY).astype(np.float32))
            shift, _ = cv2.phaseCorrelate(gray_left, gray_right)

            base_x, base_y = positions[(row, col - 1)]
            new_x = base_x + tile_w - overlap + shift[0]
            new_y = base_y + shift[1]
            positions[(row, col)] = (new_x, new_y)

        elif row > 0:
            top_tile = tiles[(row - 1, col)]
            current_tile = tiles[(row, col)]

            patch_top = top_tile[-overlap:, :]
            patch_bottom = current_tile[:overlap, :]

            gray_top = normalize(cv2.cvtColor(patch_top, cv2.COLOR_BGR2GRAY))
            gray_bottom = normalize(cv2.cvtColor(patch_bottom, cv2.COLOR_BGR2GRAY))
            shift, _ = cv2.phaseCorrelate(
            np.float32(gray_top), np.float32(gray_bottom))

            base_x, base_y = positions[(row - 1, col)]
            new_x = base_x + shift[0]
            new_y = base_y + tile_h - overlap + shift[1]
            positions[(row, col)] = (new_x, new_y)

xs = [pos[0] for pos in positions.values()]
ys = [pos[1] for pos in positions.values()]
min_x = int(np.floor(min(xs)))
min_y = int(np.floor(min(ys)))

corrected_positions = {
    k: (int(round(x - min_x)), int(round(y - min_y)))
    for k, (x, y) in positions.items()
}

max_x = max([x + tile_w for x, y in corrected_positions.values()])
max_y = max([y + tile_h for x, y in corrected_positions.values()])
stitched = np.zeros((max_y, max_x, 3), dtype=np.uint8)

for (row, col), (x, y) in corrected_positions.items():
    tile = tiles[(row, col)]
    stitched[y:y+tile.shape[0], x:x+tile.shape[1]] = tile

output_file = os.path.join(output_path, "stitched.jpg")
cv2.imwrite(output_file, stitched)
print(f"✅ Bild erfolgreich gestitcht: {output_file}")
