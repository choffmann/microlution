import os
import re
import cv2
import json
import numpy as np
from tqdm import tqdm
from collections import deque
import time

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
    # Visualisierung der Matches
    match_vis = cv2.drawMatches(
        target_img, kp2, placed_img, kp1, good, None,
        flags=cv2.DrawMatchesFlags_NOT_DRAW_SINGLE_POINTS
    )
    cv2.imwrite(f"img/5/stitched/sift/matches/matches_{index}.png", match_vis)
    dx = [kp1[m.trainIdx].pt[0] - kp2[m.queryIdx].pt[0] for m in good]
    dy = [kp1[m.trainIdx].pt[1] - kp2[m.queryIdx].pt[1] for m in good]

    return (int(np.median(dx)), int(np.median(dy))), len(good)

def compute_offset_with_overlap(placed_img, target_img, index, direction, overlap_px=100, debug=False, debug_dir="debug"):

    if debug:
        os.makedirs(debug_dir, exist_ok=True)
        os.makedirs(f"{debug_dir}/crops", exist_ok=True)
        os.makedirs(f"{debug_dir}/keypoints", exist_ok=True)
        os.makedirs(f"{debug_dir}/keypoints_on_full", exist_ok=True)

    sift = cv2.SIFT_create()
    h, w = placed_img.shape[:2]

    if direction == "right":
        img1_crop = placed_img[:, w - overlap_px:]
        img2_crop = target_img[:, :overlap_px]
        offset_img1 = (w - overlap_px, 0)
        offset_img2 = (0, 0)
    elif direction == "left":
        img1_crop = placed_img[:, :overlap_px]
        img2_crop = target_img[:, w - overlap_px:]
        offset_img1 = (0, 0)
        offset_img2 = (w - overlap_px, 0)
    elif direction == "bottom":
        img1_crop = placed_img[h - overlap_px:, :]
        img2_crop = target_img[:overlap_px, :]
        offset_img1 = (0, h - overlap_px)
        offset_img2 = (0, 0)
    elif direction == "top":
        img1_crop = placed_img[:overlap_px, :]
        img2_crop = target_img[h - overlap_px:, :]
        offset_img1 = (0, 0)
        offset_img2 = (0, h - overlap_px)
    else:
        return None, 0

    if direction not in {"left", "right", "top", "bottom"}:
        return None, 0

    gray1 = cv2.cvtColor(img1_crop, cv2.COLOR_BGR2GRAY)
    gray2 = cv2.cvtColor(img2_crop, cv2.COLOR_BGR2GRAY)
    kp1, des1 = sift.detectAndCompute(gray1, None)
    kp2, des2 = sift.detectAndCompute(gray2, None)

    if debug:
        cv2.imwrite(f"{debug_dir}/crops/crop1_{index}_{direction}.png", img1_crop)
        cv2.imwrite(f"{debug_dir}/crops/crop2_{index}_{direction}.png", img2_crop)

        kp_img1 = cv2.drawKeypoints(img1_crop, kp1, None, flags=cv2.DRAW_MATCHES_FLAGS_DRAW_RICH_KEYPOINTS)
        kp_img2 = cv2.drawKeypoints(img2_crop, kp2, None, flags=cv2.DRAW_MATCHES_FLAGS_DRAW_RICH_KEYPOINTS)
        cv2.imwrite(f"{debug_dir}/keypoints/kp_crop1_{index}_{direction}.png", kp_img1)
        cv2.imwrite(f"{debug_dir}/keypoints/kp_crop2_{index}_{direction}.png", kp_img2)

    if des1 is None or des2 is None:
        return None, 0

    matcher = cv2.BFMatcher()
    matches = matcher.knnMatch(des2, des1, k=2)
    good = [m for m, n in matches if m.distance < 0.7 * n.distance]

    if len(good) < 10:
        return None, len(good)

    if debug:
        kp1_shifted = [cv2.KeyPoint(k.pt[0] + offset_img1[0], k.pt[1] + offset_img1[1], k.size) for k in kp1]
        kp2_shifted = [cv2.KeyPoint(k.pt[0] + offset_img2[0], k.pt[1] + offset_img2[1], k.size) for k in kp2]

        full1 = cv2.drawKeypoints(placed_img, kp1_shifted, None, flags=cv2.DRAW_MATCHES_FLAGS_DRAW_RICH_KEYPOINTS)
        full2 = cv2.drawKeypoints(target_img, kp2_shifted, None, flags=cv2.DRAW_MATCHES_FLAGS_DRAW_RICH_KEYPOINTS)
        cv2.imwrite(f"{debug_dir}/keypoints_on_full/full1_{index}_{direction}.png", full1)
        cv2.imwrite(f"{debug_dir}/keypoints_on_full/full2_{index}_{direction}.png", full2)

    dx = [kp1[m.trainIdx].pt[0] - kp2[m.queryIdx].pt[0] for m in good]
    dy = [kp1[m.trainIdx].pt[1] - kp2[m.queryIdx].pt[1] for m in good]

    if direction == "right":
        dx = [d + (w - overlap_px) for d in dx]
    elif direction == "left":
        dx = [d - (w - overlap_px) for d in dx]
    elif direction == "bottom":
        dy = [d + (h - overlap_px) for d in dy]
    elif direction == "top":
        dy = [d - (h - overlap_px) for d in dy]

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
                # # Diagonal
                # (cx - 1, cy - 1): "top-left",
                # (cx + 1, cy - 1): "top-right",
                # (cx - 1, cy + 1): "bottom-left",
                # (cx + 1, cy + 1): "bottom-right",
            }

            # print(f"[{index}] Tile {current} prüft Nachbarn: {list(neighbors.keys())}")

            for neighbor_coord, direction in neighbors.items():
                if neighbor_coord not in tiles:
                    continue

                neighbor_tile = tiles[neighbor_coord]
                if neighbor_tile["pos"] is not None:
                    continue

                # offset, score = compute_offset(current_tile["img"], neighbor_tile["img"], index)
                offset, score = compute_offset_with_overlap(
                    current_tile["img"], neighbor_tile["img"], index,
                    direction=direction, overlap_px=800, debug=True
                )

                if offset is not None and score >= 5:
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

def main():
    start_time = time.time()
    folder = "img/5/tiles"
    tiles = load_tiles(folder)
    print("Geladene Tiles:", len(tiles))

    print("Starte Ausrichtung...")
    build_positions(tiles)

    print("Erzeuge Mosaik …")
    mosaic, min_x, min_y = stitch_tiles(tiles)

    stitched_soft_blend = stitch_soft_blend(tiles, "weight_map.png", blend_mask_dir="img/5/stitched/weight_map/")

    cv2.imwrite("img/5/stitched/stitched_only_overlap.png", stitched_soft_blend)

    # cv2.imshow("Mosaic", mosaic)
    # cv2.waitKey(0)
    # cv2.destroyAllWindows()
    end_time = time.time()
    duration = end_time - start_time
    print(f"\n✅ Laufzeit: {duration:.2f} Sekunden")

if __name__ == "__main__":
    main()

