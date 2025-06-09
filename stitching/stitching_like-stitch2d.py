import os
import re
import cv2
import json
import numpy as np
from tqdm import tqdm
from collections import deque

def parse_tile_filename(name):
    m = re.match(r"tile_(\d+)_(\d+)\.(jpg|jpeg|png)", name)
    return (int(m.group(1)), int(m.group(2))) if m else None

def load_tiles(path):
    tiles = {}
    for file in os.listdir(path):
        coords = parse_tile_filename(file)
        if coords:
            img = cv2.imread(os.path.join(path, file))
            if img is not None:
                tiles[coords] = {"img": img, "pos": None, "filename": file}
            else:
                print("Fehler beim Laden:", file)
    return tiles

def compute_offset(a, b):
    sift = cv2.SIFT_create()
    kp1, des1 = sift.detectAndCompute(cv2.cvtColor(a, cv2.COLOR_BGR2GRAY), None)
    kp2, des2 = sift.detectAndCompute(cv2.cvtColor(b, cv2.COLOR_BGR2GRAY), None)
    if des1 is None or des2 is None:
        return None, 0
    m = cv2.BFMatcher().knnMatch(des2, des1, k=2)
    good = [m0 for m0, m1 in m if m0.distance < 0.7 * m1.distance]
    if len(good) < 10:
        return None, len(good)
    dx = [kp1[m.trainIdx].pt[0] - kp2[m.queryIdx].pt[0] for m in good]
    dy = [kp1[m.trainIdx].pt[1] - kp2[m.queryIdx].pt[1] for m in good]
    return (int(np.median(dx)), int(np.median(dy))), len(good)

def build_positions(tiles):
    if (0, 0) not in tiles:
        print("tile_0_0 fehlt")
        return
    tiles[(0, 0)]["pos"] = (0, 0)
    q = deque([(0, 0)])
    done = set([(0, 0)])
    with tqdm(total=len(tiles)) as bar:
        bar.update(1)
        while q:
            cx, cy = q.popleft()
            base = tiles[(cx, cy)]
            for dx, dy in [(-1,0),(1,0),(0,-1),(0,1)]:
                nx, ny = cx + dx, cy + dy
                if (nx, ny) in tiles and tiles[(nx, ny)]["pos"] is None:
                    off, score = compute_offset(base["img"], tiles[(nx, ny)]["img"])
                    if off and score >= 10:
                        bx, by = base["pos"]
                        dx, dy = off
                        tiles[(nx, ny)]["pos"] = (bx + dx, by + dy)
                        q.append((nx, ny))
                        done.add((nx, ny))
                        bar.update(1)

def stitch_tiles(tiles):
    coords = [t["pos"] for t in tiles.values() if t["pos"]]
    xs = [x for x, y in coords]
    ys = [y for x, y in coords]
    minx, miny = min(xs), min(ys)
    maxx = max(x + t["img"].shape[1] for t in tiles.values() if t["pos"])
    maxy = max(y + t["img"].shape[0] for t in tiles.values() if t["pos"])
    w, h = maxx - minx, maxy - miny
    canvas = np.zeros((h, w, 3), np.uint8)
    for t in tqdm(tiles.values()):
        if t["pos"] is None:
            continue
        x, y = t["pos"]
        h, w = t["img"].shape[:2]
        canvas[y - miny:y - miny + h, x - minx:x - minx + w] = t["img"]
    return canvas, minx, miny

def stitch_soft_blend(tiles, save_weight_map_path=None, blend_mask_dir=None):
    pos = [t["pos"] for t in tiles.values() if t["pos"]]
    sizes = [t["img"].shape[:2] for t in tiles.values() if t["pos"]]
    minx = min(x for x, y in pos)
    miny = min(y for x, y in pos)
    maxx = max(x + w for (x, y), (h, w) in zip(pos, sizes))
    maxy = max(y + h for (x, y), (h, w) in zip(pos, sizes))
    canvas = np.zeros((maxy - miny, maxx - minx, 3), np.float32)
    weight = np.zeros((maxy - miny, maxx - minx, 1), np.float32)
    if blend_mask_dir:
        os.makedirs(blend_mask_dir, exist_ok=True)
    for (coord, t) in tiles.items():
        if t["pos"] is None:
            continue
        img = t["img"].astype(np.float32)
        h, w = img.shape[:2]
        x, y = t["pos"]
        x -= minx
        y -= miny
        wy = np.linspace(0, 1, h).reshape(h, 1)
        wx = np.linspace(0, 1, w).reshape(1, w)
        b = np.minimum(wy, 1 - wy) * np.minimum(wx, 1 - wx)
        b = b[..., np.newaxis]
        b = b / b.max()
        b = np.clip(b, 0.01, 1.0)
        if blend_mask_dir:
            mask = (b[:, :, 0] * 255).astype(np.uint8)
            name = f"blend_tile_{coord[0]}_{coord[1]}.png"
            cv2.imwrite(os.path.join(blend_mask_dir, name), mask)
        canvas[y:y+h, x:x+w] += img * b
        weight[y:y+h, x:x+w] += b
    if save_weight_map_path:
        vis = (255 * weight / weight.max()).astype(np.uint8)
        cv2.imwrite(save_weight_map_path, vis)
    res = canvas / np.clip(weight, 1e-5, None)
    return np.clip(res, 0, 255).astype(np.uint8)


def main():
    folder = "img/5/tiles"
    tiles = load_tiles(folder)
    print("Tiles geladen:", len(tiles))
    build_positions(tiles)
    stitched, minx, miny = stitch_tiles(tiles)
    result = stitch_soft_blend(tiles, "weight_map.png", blend_mask_dir="img/5/stitched/weight_map/")
    cv2.imwrite("img/5/stitched/stitched_seams.png", result)
    print("Fertig")

if __name__ == "__main__":
    main()

