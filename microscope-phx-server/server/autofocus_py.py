import urllib.request
from PIL import Image, ImageFilter
import io
import numpy as np
import time
import json
import argparse

parser = argparse.ArgumentParser()
parser.add_argument("--steps", type=int, default=3)
args = parser.parse_args()

url = "http://192.168.188.58:5000/api/v2/streams/mjpeg"  # Your MJPEG URL
stream = urllib.request.urlopen(url)

def variance_of_laplacian(img_array):
    # Simple Laplacian kernel
    kernel = np.array([[0, 1, 0],
                       [1, -4, 1],
                       [0, 1, 0]])
    filtered = np.abs(np.convolve(img_array.flatten(), kernel.flatten(), 'same'))
    return filtered.var()

start_time = time.time()
duration = 2  # seconds
sharpness = 0
prev_sharpness = 0
first_highest_sharpness = 0
second_highest_sharpness = 0
step = args.steps
new_step = 0
steps_ran = 0
liste = []
i = 0
j = 0
stream = urllib.request.urlopen("http://192.168.188.58:5000/api/v2/streams/mjpeg")

def measure_sharpness_from_stream(stream, max_frames=6):
    bytes_buffer = b''
    sharpness = 0
    highest_sharpness = 0
    i = 0
    while i < max_frames:
        bytes_buffer += stream.read(1024)
        a = bytes_buffer.find(b'\xff\xd8')
        b = bytes_buffer.find(b'\xff\xd9')
        if a != -1 and b != -1:
            jpg = bytes_buffer[a:b+2]
            bytes_buffer = bytes_buffer[b+2:]
            try:
                img = Image.open(io.BytesIO(jpg)).convert('L')  # grayscale
                lap_img = img.filter(ImageFilter.FIND_EDGES)
                lap_np = np.array(lap_img)
                sharpness = lap_np.var()
                print(f"sharpness: {sharpness}")
                if sharpness > highest_sharpness:
                    highest_sharpness = sharpness
                else:
                    pass
                i += 1
            except Exception as e:
                pass
    return highest_sharpness

while first_highest_sharpness < 250 and i < 15:
    first_highest_sharpness = measure_sharpness_from_stream(stream)
    # if sharpness > prev_sharpness  : 
    #     step = step
    # else:
    #     step = step * -1
    print(f"first {first_highest_sharpness}")
    if first_highest_sharpness < 250:
        data = {
        'x': 0,
        'y': 0,
        'z': step
        }
        json_data = json.dumps(data).encode('utf-8')
        req = urllib.request.Request("http://192.168.188.58:5000/api/v2/actions/stage/move/", data=json_data, headers={'Content-Type': 'application/json'})

        try:
            with urllib.request.urlopen(req, timeout=5) as response:
                response_text = response.read().decode('utf-8')
        except Exception as e:
            print('Fehler bei der Anfrage:', e)
        steps_ran += step
        liste.append({"step": steps_ran, "sharpness": sharpness})
    else:
        if measure_sharpness_from_stream(stream) < 600:
            while second_highest_sharpness < first_highest_sharpness and j < 15:
                if second_highest_sharpness > first_highest_sharpness:
                    break
                second_highest_sharpness = measure_sharpness_from_stream(stream)
                print(f"second {second_highest_sharpness}")
                data = {
                'x': 0,
                'y': 0,
                'z': -step / 3
                }
                json_data = json.dumps(data).encode('utf-8')
                req = urllib.request.Request("http://192.168.188.58:5000/api/v2/actions/stage/move/", data=json_data, headers={'Content-Type': 'application/json'})

                try:
                    with urllib.request.urlopen(req, timeout=5) as response:
                        response_text = response.read().decode('utf-8')
                except Exception as e:
                    print('Fehler bei der Anfrage:', e)
                steps_ran += step
                liste.append({"step": steps_ran, "sharpness": sharpness})
                j += 1
    i += 1

print("")
# import urllib.request
# from PIL import Image, ImageFilter
# import io
# import numpy as np
# import time
# import json
# import argparse

# parser = argparse.ArgumentParser()
# parser.add_argument("--steps", type=int, default=3)
# args = parser.parse_args()

# URL = "http://192.168.188.58:5000/api/v2/streams/snapshot"  # Your MJPEG URL
# #https://cors.redoc.ly/api/v2/streams/snapshot
# duration = args.steps / 1000  # seconds
# list_fps_sharpness = []
# list_fps_sharpness_sorted = []
# frame_count = 0
# start_time = time.time()
# last_fps_time = start_time
# list_fps_sharpness = []
# list_fps_sharpness_sorted = []
# prev_sharpness = 0
# prev_sharpness = 0
# sharpness = 0
# fps=0
# i = 0

# def get_image_from_url(url):
#     data = {
#         "bayer": False,
#         "resize": {
#             "height": 480,
#             "width": 640
#         },
#         "use_video_port": False
#         }
#     json_data = json.dumps(data).encode('utf-8')
#     req = urllib.request.Request(URL)
#     with urllib.request.urlopen(req, timeout=5) as response:
#         img_data = response.read()
#     img = Image.open(io.BytesIO(img_data)).convert('L')  # convert to grayscale
#     return img

# def sharpnessdef(img):
#     lap = img.filter(ImageFilter.FIND_EDGES)
#     lap_np = np.array(lap)
#     return lap_np.var()

# while True:
#     image = get_image_from_url(URL)
#     sharpness = sharpnessdef(image)
    
#     if sharpness != 0:
#         data = {
#         'x': 0,
#         'y': 0,
#         'z': args.steps * i
#         }
#         json_data = json.dumps(data).encode('utf-8')
#         req = urllib.request.Request("http://192.168.188.58:5000/api/v2/actions/stage/move/", data=json_data, headers={'Content-Type': 'application/json'})

#         try:
#             with urllib.request.urlopen(req, timeout=5) as response:
#                 response_text = response.read().decode('utf-8')
#                 print('Antwort vom Server:', response_text)
#         except Exception as e:
#             print('Fehler bei der Anfrage:', e)

#         list_fps_sharpness.append({"fps": fps, "sharpness": sharpness,"steps": args.steps * i})
#         list_fps_sharpness_sorted = sorted(list_fps_sharpness, key=lambda d: d['sharpness'], reverse=True)
#         i += 1

#         if i > 5:
#             break
#     # if elapsed_total > duration:
#     #     #print("Done.")
#     #     break
# time.sleep(1)
# data = {
# 'x': 0,
# 'y': 0,
# 'z': list_fps_sharpness_sorted[0]["steps"] + (args.steps * i) * -1 
# }
# json_data = json.dumps(data).encode('utf-8')
# req = urllib.request.Request("http://192.168.188.58:5000/api/v2/actions/stage/move/", data=json_data, headers={'Content-Type': 'application/json'})

# try:
#     with urllib.request.urlopen(req, timeout=5) as response:
#         response_text = response.read().decode('utf-8')
#         print('Antwort vom Server:', response_text)
# except Exception as e:
#     print('Fehler bei der Anfrage:', e)

# for a in list_fps_sharpness_sorted:
#     print(a)
# print(list_fps_sharpness_sorted[0]["steps"] + (args.steps * i) * -1)
# #print(list_fps_sharpness_sorted[0]["time"])