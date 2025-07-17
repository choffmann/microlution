import os
import sys
import zipfile
import re
import cv2
import numpy as np
from tqdm import tqdm
from collections import deque
import time

OVERLAP_PX        = 400
MIN_SCORE_FULL    = 15
MIN_SCORE_OVERLAP = 10
DEBUG             = True

def extract_zip(zip_path, extract_to="tmp_tiles"):
    os.makedirs(extract_to, exist_ok=True)
    with zipfile.ZipFile(zip_path, 'r') as zip_ref:
        zip_ref.extractall(extract_to)
    return extract_to

def parse_tile_filename(name):
    m = re.match(r"tile_(\d+)_(\d+)\.(?:jpg|jpeg|png)", name)
    return (int(m[1]), int(m[2])) if m else None

class Tile:
    def __init__(self, coord, img, orb):
        self.coord  = coord
        self.img    = img
        self.pos    = None
        self._orb   = orb
        gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
        self.kp_full, self.des_full = orb.detectAndCompute(gray, None)
        self.crop_cache = {}

    def descriptors_for(self, direction, overlap_px):
        key = (direction, overlap_px)
        if key in self.crop_cache:
            return self.crop_cache[key]
        h, w = self.img.shape[:2]
        if direction == "right":
            crop = self.img[:, w-overlap_px:]
        elif direction == "left":
            crop = self.img[:, :overlap_px]
        elif direction == "bottom":
            crop = self.img[h-overlap_px:, :]
        elif direction == "top":
            crop = self.img[:overlap_px, :]
        else:
            return None, None, None
        gray = cv2.cvtColor(crop, cv2.COLOR_BGR2GRAY)
        kp, des = self._orb.detectAndCompute(gray, None)
        self.crop_cache[key] = (crop, kp, des)
        return crop, kp, des

def match_descriptors(des_q, des_t, matcher, ratio=0.75):
    if des_q is None or des_t is None:
        return []
    raw = matcher.knnMatch(des_q, des_t, k=2)
    return [m for m,n in raw if m.distance < ratio * n.distance]

def compute_offset_full(tile_a, tile_b, matcher):
    matches = match_descriptors(tile_b.des_full, tile_a.des_full, matcher)
    if len(matches) < MIN_SCORE_FULL:
        return None, len(matches)
    dx = [ tile_a.kp_full[m.trainIdx].pt[0] - tile_b.kp_full[m.queryIdx].pt[0] for m in matches ]
    dy = [ tile_a.kp_full[m.trainIdx].pt[1] - tile_b.kp_full[m.queryIdx].pt[1] for m in matches ]
    return (int(np.median(dx)), int(np.median(dy))), len(matches)

def compute_offset_overlap(tile_a, tile_b, direction, overlap_px, matcher):
    crop_a, kp_a, des_a = tile_a.descriptors_for(direction, overlap_px)
    inv_dir = {"right":"left","left":"right","top":"bottom","bottom":"top"}[direction]
    crop_b, kp_b, des_b = tile_b.descriptors_for(inv_dir, overlap_px)
    matches = match_descriptors(des_b, des_a, matcher)
    if len(matches) < MIN_SCORE_OVERLAP:
        return None, len(matches)
    dx = [ kp_a[m.trainIdx].pt[0] - kp_b[m.queryIdx].pt[0] for m in matches ]
    dy = [ kp_a[m.trainIdx].pt[1] - kp_b[m.queryIdx].pt[1] for m in matches ]
    h, w = tile_a.img.shape[:2]
    if direction == "right":
        dx = [d + (w - overlap_px) for d in dx]
    elif direction == "left":
        dx = [d - (w - overlap_px) for d in dx]
    elif direction == "bottom":
        dy = [d + (h - overlap_px) for d in dy]
    elif direction == "top":
        dy = [d - (h - overlap_px) for d in dy]
    return (int(np.median(dx)), int(np.median(dy))), len(matches)

def build_positions(tiles):
    origin = (0,0)
    if origin not in tiles:
        raise ValueError("tile_0_0 nicht gefunden!")
    tiles[origin].pos = (0,0)
    matcher = cv2.BFMatcher(cv2.NORM_HAMMING, crossCheck=False)
    def neighbors(coord):
        x,y = coord
        return {(x-1,y):"left",(x+1,y):"right",(x,y-1):"top",(x,y+1):"bottom"}.items()
    q = deque([origin]); idx=0; total=len(tiles)
    print(f"Starte BFS (Tiles: {total})…")
    while q:
        idx+=1; cur=q.popleft(); t_cur=tiles[cur]
        for nb,dirn in neighbors(cur):
            if nb not in tiles or tiles[nb].pos is not None: continue
            t_nb=tiles[nb]
            ov = min(OVERLAP_PX,*(d//2 for d in t_cur.img.shape[:2]))
            off_ov,sc_ov=compute_offset_overlap(t_cur,t_nb,dirn,ov,matcher)
            method,offset,score = ("overlap",off_ov,sc_ov)
            if offset is None:
                off_f,sc_f=compute_offset_full(t_cur,t_nb,matcher)
                if off_f: method,offset,score=("full",off_f,sc_f)
            print(f"[{idx}] {cur}->{nb} via {method}, score={score}")
            thresh=MIN_SCORE_OVERLAP if method=="overlap" else MIN_SCORE_FULL
            if offset and score>=thresh:
                bx,by=t_cur.pos; dx,dy=offset
                tiles[nb].pos=(bx+dx,by+dy); q.append(nb)
    placed=sum(1 for t in tiles.values() if t.pos is not None)
    print(f"Fertig: {placed}/{total} gesetzt.")

def stitch_tiles(tiles):
    poses=[t.pos for t in tiles.values() if t.pos]
    xs=[x for x,y in poses]; ys=[y for x,y in poses]
    min_x, min_y = min(xs), min(ys)
    max_x = max(t.pos[0]+t.img.shape[1] for t in tiles.values() if t.pos)
    max_y = max(t.pos[1]+t.img.shape[0] for t in tiles.values() if t.pos)
    W=max_x-min_x; H=max_y-min_y
    canvas=np.zeros((H,W,3),dtype=np.uint8)
    for t in tiles.values():
        if not t.pos: continue
        x,y=t.pos; h,w=t.img.shape[:2]
        x0,y0=x-min_x,y-min_y
        canvas[y0:y0+h, x0:x0+w]=t.img
    return canvas, min_x, min_y

def stitch_soft_blend(tiles, save_weight_map_path=None, blend_mask_dir=None):
    positions=[t.pos for t in tiles.values() if t.pos]
    sizes=[t.img.shape[:2] for t in tiles.values() if t.pos]
    min_x=min(x for x,y in positions); min_y=min(y for x,y in positions)
    max_x=max(x+w for (x,y),(h,w) in zip(positions,sizes))
    max_y=max(y+h for (x,y),(h,w) in zip(positions,sizes))
    H=max_y-min_y; W=max_x-min_x
    canvas=np.zeros((H,W,3),dtype=np.float32)
    weight=np.zeros((H,W,1),dtype=np.float32)
    if blend_mask_dir: os.makedirs(blend_mask_dir, exist_ok=True)
    for coord,t in tiles.items():
        if not t.pos: continue
        img=t.img.astype(np.float32)
        h,w=img.shape[:2]
        x,y=t.pos; x0,y0=x-min_x,y-min_y
        wy=np.linspace(0,1,h).reshape(h,1); wx=np.linspace(0,1,w).reshape(1,w)
        blend=np.minimum(wy,1-wy)*np.minimum(wx,1-wx); blend=blend/blend.max(); blend=np.clip(blend,0.01,1.0)[...,None]
        if blend_mask_dir:
            gray=(blend[:,:,0]*255).astype(np.uint8)
            cv2.imwrite(os.path.join(blend_mask_dir,f"blend_{coord[0]}_{coord[1]}.png"), gray)
        canvas[y0:y0+h, x0:x0+w]+=img*blend
        weight[y0:y0+h, x0:x0+w]+=blend
    if save_weight_map_path:
        wv=(255*weight/weight.max()).astype(np.uint8)
        cv2.imwrite(save_weight_map_path,wv)
    res=canvas/np.clip(weight,1e-5,None)
    return np.clip(res,0,255).astype(np.uint8)

def save_tile_with_overlap_keypoints(tile, overlap_px, out_dir):
    h, w = tile.img.shape[:2]
    canvas = tile.img.copy()

    directions = ["left", "right", "top", "bottom"]
    for direction in directions:
        crop, kp, _ = tile.descriptors_for(direction, overlap_px)
        if crop is None or kp is None:
            continue

        if direction == "left":
            dx, dy = 0, 0
        elif direction == "right":
            dx, dy = w - overlap_px, 0
        elif direction == "top":
            dx, dy = 0, 0
        elif direction == "bottom":
            dx, dy = 0, h - overlap_px
        else:
            continue

        shifted_kp = []
        for point in kp:
            p = cv2.KeyPoint(
                point.pt[0] + dx, point.pt[1] + dy,
                point.size, point.angle,
                point.response, point.octave, point.class_id
            )
            shifted_kp.append(p)

        canvas = cv2.drawKeypoints(
            canvas, shifted_kp, None,
            flags=cv2.DRAW_MATCHES_FLAGS_DRAW_RICH_KEYPOINTS
        )

    out_path = os.path.join(out_dir, f"orb_overlap_{tile.coord[0]}_{tile.coord[1]}.jpg")
    cv2.imwrite(out_path, canvas)

def main():
    start=time.time()
    zip_path=sys.argv[1]
    folder=extract_zip(zip_path)
    orb=cv2.ORB_create(nfeatures=2000)
    tiles={}
    for f in os.listdir(folder):
        c=parse_tile_filename(f)
        if not c: continue
        img=cv2.imread(os.path.join(folder,f))
        if img is None: continue
        tiles[c]=Tile(c,img,orb)
    print("Geladene Tiles:", len(tiles))
    build_positions(tiles)
    _,_,_=stitch_tiles(tiles)
    result=stitch_soft_blend(tiles)
    out=os.path.join(os.path.dirname(zip_path),"stitched__orb_optimized.png")
    cv2.imwrite(out, result)
    # save_tile_with_overlap_keypoints(tiles.get((3,3)), OVERLAP_PX, os.path.dirname(zip_path))
    print(f"✅ Laufzeit: {time.time()-start:.2f}s")

if __name__=="__main__":
    main()
