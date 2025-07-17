from stitch2d import create_mosaic
import numpy as np
import cv2
import time

start_time = time.time()

mosaic = create_mosaic("img/5/tiles")

mosaic.downsample(0.6)
mosaic.align()
mosaic.reset_tiles()


mosaic.smooth_seams()
mosaic.save("img/5/stitched/stitched_mosaic.jpg")

end_time = time.time()
duration = end_time - start_time
print(f"\n✅ Laufzeit: {duration:.2f} Sekunden")
