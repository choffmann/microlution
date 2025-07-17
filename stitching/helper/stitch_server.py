from flask import Flask, request, jsonify
import os
import cv2
import time
from stitching_like_stitch2d import load_tiles, build_positions, stitch_tiles, stitch_soft_blend

app = Flask(__name__)

@app.route('/stitch', methods=['POST'])
def stitch():
    data = request.get_json()
    tile_dir = data.get("tile_dir")
    output_dir = data.get("output_dir", os.path.join(tile_dir, "../stitched"))

    if not tile_dir or not os.path.isdir(tile_dir):
        return jsonify({"error": "tile_dir fehlt oder ungültig"}), 400

    try:
        start_time = time.time()
        tiles = load_tiles(tile_dir)
        build_positions(tiles)
        print("Erzeuge Mosaik …")

        stitched_soft_blend = stitch_soft_blend(tiles, blend_mask_dir=os.path.join(output_dir, "weight_map"))

        os.makedirs(output_dir, exist_ok=True)
        result_path = os.path.join(output_dir, "stitched_image_api.png")
        cv2.imwrite(result_path, stitched_soft_blend)

        end_time = time.time()
        duration = end_time - start_time
        print(f"\n✅ Laufzeit: {duration:.2f} Sekunden")

        return jsonify({
            "status": "success",
            "result_image": os.path.abspath(result_path),
            "duration": duration
        })

    except Exception as e:
        return jsonify({"error": str(e)}), 500

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5001)
