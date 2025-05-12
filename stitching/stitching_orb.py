import cv2
import numpy as np
import os

img_dir = "img"
img_number = "1"
tile_dir = "tiles"
output_dir = "stitched"
rows, cols = 4, 6
overlap = 20 

tile_w, tile_h = None, None

input_path = os.path.join(img_dir, img_number, tile_dir)
output_path = os.path.join(img_dir, img_number, output_dir)
os.makedirs(output_path, exist_ok=True)

def compute_offset_orb(img1, img2):
    gray1 = cv2.cvtColor(img1, cv2.COLOR_BGR2GRAY)
    gray2 = cv2.cvtColor(img2, cv2.COLOR_BGR2GRAY)

    orb = cv2.ORB_create(500)
    kp1, des1 = orb.detectAndCompute(gray1, None)
    kp2, des2 = orb.detectAndCompute(gray2, None)

    if des1 is None or des2 is None or len(kp1) < 5 or len(kp2) < 5:
        print("  [!] Zu wenige Features – setze Offset (0, 0)")
        return (0.0, 0.0)

    bf = cv2.BFMatcher(cv2.NORM_HAMMING, crossCheck=True)
    matches = bf.match(des1, des2)
    matches = sorted(matches, key=lambda x: x.distance)

    dxs, dys = [], []
    for m in matches[:20]:
        pt1 = kp1[m.queryIdx].pt
        pt2 = kp2[m.trainIdx].pt
        dxs.append(pt2[0] - pt1[0])
        dys.append(pt2[1] - pt1[1])

    if dxs and dys:
        return (np.median(dxs), np.median(dys))
    return (0.0, 0.0)

tiles = {}
for row in range(rows):
    for col in range(cols):
        tile_path = os.path.join(input_path, f"tile_{row}_{col}_overlap.jpg")
        img = cv2.imread(tile_path)
        if img is None:
            print(f"  [!] Fehler beim Laden: {tile_path} – wird übersprungen")
            continue
        if tile_w is None:
            tile_h, tile_w = img.shape[:2]
        tiles[(row, col)] = img
        print(f"  [+] OK: {tile_path}")

positions = {}
if (0, 0) in tiles:
    positions[(0, 0)] = (0.0, 0.0)
else:
    print("[!] Tile (0,0) fehlt – Stitching abgebrochen.")
    exit(1)

for row in range(rows):
    for col in range(cols):
        if (row, col) not in tiles or (row, col) == (0, 0):
            continue

        if col > 0 and (row, col - 1) in tiles:
            left_tile = tiles[(row, col - 1)]
            curr_tile = tiles[(row, col)]

            patch_left = left_tile[:, -overlap:]
            patch_right = curr_tile[:, :overlap]

            dx, dy = compute_offset_orb(patch_left, patch_right)
            base_x, base_y = positions.get((row, col - 1), (0, 0))
            new_x = base_x + tile_w - overlap + dx
            new_y = base_y + dy
            positions[(row, col)] = (new_x, new_y)

        elif row > 0 and (row - 1, col) in tiles:
            top_tile = tiles[(row - 1, col)]
            curr_tile = tiles[(row, col)]

            patch_top = top_tile[-overlap:, :]
            patch_bottom = curr_tile[:overlap, :]

            dx, dy = compute_offset_orb(patch_top, patch_bottom)
            base_x, base_y = positions.get((row - 1, col), (0, 0))
            new_x = base_x + dx
            new_y = base_y + tile_h - overlap + dy
            positions[(row, col)] = (new_x, new_y)

xs = [p[0] for p in positions.values()]
ys = [p[1] for p in positions.values()]
min_x, min_y = int(np.floor(min(xs))), int(np.floor(min(ys)))

corrected_positions = {
    k: (int(round(x - min_x)), int(round(y - min_y)))
    for k, (x, y) in positions.items()
}

max_x = max([x + tile_w for x, y in corrected_positions.values()])
max_y = max([y + tile_h for x, y in corrected_positions.values()])
stitched = np.zeros((max_y, max_x, 3), dtype=np.uint8)

for (row, col), (x, y) in corrected_positions.items():
    tile = tiles.get((row, col))
    if tile is not None:
        stitched[y:y+tile.shape[0], x:x+tile.shape[1]] = tile
        print(f"  [+] Eingesetzt: ({row}, {col}) bei ({x}, {y})")
    else:
        print(f"  [!] Tile fehlt in tiles: ({row}, {col}) – übersprungen")

output_file = os.path.join(output_path, "stitched_orb.jpg")
cv2.imwrite(output_file, stitched)
print(f"Bild gespeichert unter {output_file}")

