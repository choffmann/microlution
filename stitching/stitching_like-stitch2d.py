import os
import re
import cv2
import json
import numpy as np
from tqdm import tqdm
from collections import deque

def parse_tile_filename(name):
    match = re.match(r"tile_(\d+)_(\d+)\.(jpg|jpeg|png)", name)
    return (int(match.group(1)), int(match.group(2))) if match else None

def load_tiles(path):
    tiles = {}
    for file in os.listdir(path):
        coords = parse_tile_filename(file)
        if coords:
            img = cv2.imread(os.path.join(path, file))
            if img is not None:
                tiles[coords] = {"img": img, "pos": None, "filename": file}
    return tiles

def compute_offset(placed_img, target_img, index):
    sift = cv2.SIFT_create()

    kp1, des1 = sift.detectAndCompute(cv2.cvtColor(placed_img, cv2.COLOR_BGR2GRAY), None)
    sift_img = cv2.drawKeypoints(placed_img, kp1, None, flags=cv2.DRAW_MATCHES_FLAGS_DRAW_RICH_KEYPOINTS)
    cv2.imwrite("img/5/stitched/sift/sift_" + str(index) + ".png", sift_img)

    kp2, des2 = sift.detectAndCompute(cv2.cvtColor(target_img, cv2.COLOR_BGR2GRAY), None)

    if des1 is None or des2 is None:
        return None, 0

    matcher = cv2.BFMatcher()
    matches = matcher.knnMatch(des2, des1, k=2)
    good = [m for m, n in matches if m.distance < 0.7 * n.distance]

    if len(good) < 10:
        return None, len(good)

    dx = [kp1[m.trainIdx].pt[0] - kp2[m.queryIdx].pt[0] for m in good]
    dy = [kp1[m.trainIdx].pt[1] - kp2[m.queryIdx].pt[1] for m in good]

    return (int(np.median(dx)), int(np.median(dy))), len(good)

def build_positions(tiles):
    origin = (0, 0)
    if origin not in tiles:
        raise ValueError("tile_0_0.* nicht gefunden.")

    tiles[origin]["pos"] = (0, 0)
    queue = deque([origin])
    visited = set([origin])

    total = len(tiles)
    index = 0
    with tqdm(total=total, desc="Positioniere Tiles") as pbar:
        pbar.update(1)
        while queue:
            index += 1
            current = queue.popleft()
            cx, cy = current
            current_tile = tiles[current]

            neighbors = {
                (cx - 1, cy): "left",
                (cx + 1, cy): "right",
                (cx, cy - 1): "top",
                (cx, cy + 1): "bottom",
                (cx - 1, cy - 1): "top-left",
                (cx + 1, cy - 1): "top-right",
                (cx - 1, cy + 1): "bottom-left",
                (cx + 1, cy + 1): "bottom-right",
            }

            for neighbor_coord in neighbors:
                if neighbor_coord not in tiles:
                    continue

                neighbor_tile = tiles[neighbor_coord]
                if neighbor_tile["pos"] is not None:
                    continue

                offset, score = compute_offset(current_tile["img"], neighbor_tile["img"], index)
                if offset is not None and score >= 10:
                    base_x, base_y = current_tile["pos"]
                    dx, dy = offset
                    neighbor_tile["pos"] = (base_x + dx, base_y + dy)

                    queue.append(neighbor_coord)
                    visited.add(neighbor_coord)
                    pbar.update(1)

def stitch_tiles(tiles):
    coords = [tile["pos"] for tile in tiles.values() if tile["pos"] is not None]
    xs = [x for x, y in coords]
    ys = [y for x, y in coords]

    min_x, min_y = min(xs), min(ys)
    max_x = max([tile["pos"][0] + tile["img"].shape[1] for tile in tiles.values() if tile["pos"]])
    max_y = max([tile["pos"][1] + tile["img"].shape[0] for tile in tiles.values() if tile["pos"]])

    width = max_x - min_x
    height = max_y - min_y
    canvas = np.zeros((height, width, 3), dtype=np.uint8)

    for tile in tqdm(tiles.values(), desc="Stitche Tiles"):
        if tile["pos"] is None:
            continue
        x, y = tile["pos"]
        img = tile["img"]
        h, w = img.shape[:2]
        x1, y1 = x - min_x, y - min_y
        canvas[y1:y1+h, x1:x1+w] = img

    return canvas, min_x, min_y

def stitch_soft_blend(tiles, save_weight_map_path=None, blend_mask_dir=None):
    positions = [t["pos"] for t in tiles.values() if t["pos"] is not None]
    sizes = [t["img"].shape[:2] for t in tiles.values() if t["pos"] is not None]

    min_x = min([x for x, y in positions])
    min_y = min([y for x, y in positions])
    max_x = max([x + w for (x, y), (h, w) in zip(positions, sizes)])
    max_y = max([y + h for (x, y), (h, w) in zip(positions, sizes)])

    height = max_y - min_y
    width = max_x - min_x

    canvas = np.zeros((height, width, 3), dtype=np.float32)
    weight = np.zeros((height, width, 1), dtype=np.float32)

    if blend_mask_dir:
        os.makedirs(blend_mask_dir, exist_ok=True)

    for (coord, tile) in tiles.items():
        if tile["pos"] is None:
            continue
        img = tile["img"].astype(np.float32)
        h, w = img.shape[:2]
        x, y = tile["pos"]
        x -= min_x
        y -= min_y

        wy = np.linspace(0, 1, h).reshape(h, 1)
        wx = np.linspace(0, 1, w).reshape(1, w)
        blend = np.minimum(wy, 1 - wy) * np.minimum(wx, 1 - wx)
        blend = blend[..., np.newaxis]
        blend = blend / blend.max()
        blend = np.clip(blend, 0.01, 1.0)

        if blend_mask_dir:
            blend_gray = (blend[:, :, 0] * 255).astype(np.uint8)
            filename = f"blend_tile_{coord[0]}_{coord[1]}.png"
            cv2.imwrite(os.path.join(blend_mask_dir, filename), blend_gray)

        canvas[y:y+h, x:x+w] += img * blend
        weight[y:y+h, x:x+w] += blend

    if save_weight_map_path:
        weight_vis = (255 * weight / weight.max()).astype(np.uint8)
        cv2.imwrite(save_weight_map_path, weight_vis)

    result = canvas / np.clip(weight, 1e-5, None)
    return np.clip(result, 0, 255).astype(np.uint8)

def correct_seams_in_place(tiles):
    tile_keys = list(tiles.keys())
    for (x, y) in tile_keys:
        tile = tiles[(x, y)]
        img_a = tile["img"]
        pos_a = tile["pos"]
        if pos_a is None:
            continue
        h_a, w_a = img_a.shape[:2]
        y1a, x1a, y2a, x2a = pos_a[1], pos_a[0], pos_a[1] + h_a, pos_a[0] + w_a

        for dx, dy in [(1, 0), (0, 1)]:            
            neighbor_key = (x + dx, y + dy)
            if neighbor_key not in tiles:
                continue
            neighbor = tiles[neighbor_key]
            img_b = neighbor["img"]
            pos_b = neighbor["pos"]
            if pos_b is None:
                continue

            h_b, w_b = img_b.shape[:2]
            y1b, x1b, y2b, x2b = pos_b[1], pos_b[0], pos_b[1] + h_b, pos_b[0] + w_b

            x1 = max(x1a, x1b)
            y1 = max(y1a, y1b)
            x2 = min(x2a, x2b)
            y2 = min(y2a, y2b)

            if x2 <= x1 or y2 <= y1:
                continue

            crop_a = img_a[y1 - y1a:y2 - y1a, x1 - x1a:x2 - x1a]
            crop_b = img_b[y1 - y1b:y2 - y1b, x1 - x1b:x2 - x1b]

            mean_a = np.mean(crop_a)
            mean_b = np.mean(crop_b)
            if mean_a == 0 or mean_b == 0:
                continue

            gamma = np.log(mean_b / 255.0) / np.log(mean_a / 255.0)
            corrected = np.power(img_a / 255.0, gamma) * 255.0
            tile["img"] = np.clip(corrected, 0, 255).astype(np.uint8)


def main():
    folder = "img/5/tiles"
    tiles = load_tiles(folder)
    print("Geladene Tiles:", len(tiles))

    print("Starte Ausrichtung...")
    build_positions(tiles)

    print("Erzeuge Mosaik …")
    mosaic, min_x, min_y = stitch_tiles(tiles)

    stitched_soft_blend = stitch_soft_blend(tiles, "weight_map.png", blend_mask_dir="img/5/stitched/weight_map/")

    cv2.imwrite("img/5/stitched/stitched_seams.png", stitched_soft_blend)

    # cv2.imshow("Mosaic", mosaic)
    # cv2.waitKey(0)
    # cv2.destroyAllWindows()

if __name__ == "__main__":
    main()


