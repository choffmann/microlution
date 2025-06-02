from stitch2d import StructuredMosaic

mosaic = StructuredMosaic(
    "img/5/tiles",
    dim=4,
    origin="upper left",
    direction="vertical",
    pattern="raster"
  )

mosaic.downsample(0.5)

mosaic.align()
mosaic.reset_tiles()
mosaic.smooth_seams()

mosaic.save("img/5/stitched/stitched_mosaic_structured.jpg")
