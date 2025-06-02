from stitch2d import create_mosaic
import numpy as np
import cv2

mosaic = create_mosaic("img/5/tiles")

try:
    mosaic.load_params()
except FileNotFoundError:
    mosaic.downsample(0.6)
    mosaic.align()
    mosaic.reset_tiles()


mosaic.smooth_seams()
mosaic.save("img/5/stitched/stitched_mosaic.jpg")

