import RPi.GPIO as GPIO
import time
import requests
import board
import busio
import digitalio
import displayio
from adafruit_ili9341 import ILI9341
from adafruit_display_text import label
import terminalio

displayio.release_displays()

# GPIO-Pin-Config Roatary encoder
CLK = 17
DT = 18
SW = 27

# OpenFlexure Server URL
URL = "http://localhost:5000/api/v2/actions/stage/move/"
STEP_SIZES = {'z': 200, 'x': 200, 'y': 200}

# SPI-Setup
spi = busio.SPI(clock=board.SCK, MOSI=board.MOSI)

backlight = digitalio.DigitalInOut(board.D16)
backlight.direction = digitalio.Direction.OUTPUT
backlight.value = True

display_bus = displayio.FourWire(
    spi,
    command=board.D24,
    chip_select=board.D8,
    reset=board.D25
)

display = ILI9341(display_bus, width=320, height=240, rotation= 180)

# GUI
splash = displayio.Group()
display.root_group = splash

# Achse-Label
label_static = label.Label(
    terminalio.FONT,
    text="Achse: ",
    color=0xFFFFFF,
    x=260,
    y=10
)

# Achsen-Kennung
label_dynamic = label.Label(
    terminalio.FONT,
    text="Z",
    color=0xFFFFFF,
    background_color=0x0000FF,
    x=300,
    y=10
)

splash.append(label_static)
splash.append(label_dynamic)

mode_colors = {'z': 0x0000FF, 'x': 0x00AA00, 'y': 0xFF0000}

# GPIO Setup Rotary-Encoder
GPIO.setmode(GPIO.BCM)
GPIO.setup(CLK, GPIO.IN, pull_up_down=GPIO.PUD_UP)
GPIO.setup(DT, GPIO.IN, pull_up_down=GPIO.PUD_UP)
GPIO.setup(SW, GPIO.IN, pull_up_down=GPIO.PUD_UP)

last_clk_state = GPIO.input(CLK)
modes = ['z', 'x', 'y']
mode_index = 0

print(f"Aktueller Modus: {modes[mode_index].upper()}")

def update_display_mode():
    axis = modes[mode_index]
    label_dynamic.text = axis.upper()
    label_dynamic.background_color = mode_colors[axis]

def move_axis(axis, amount):
    payload = {axis: amount, 'absolute': False}
    try:
        response = requests.post(URL, json=payload, timeout=1)
        if response.status_code == 201:
            print(f"{axis.upper()} bewegt um {amount} – Antwort: {response.json()}")
        else:
            print(f"Fehlerhafte Antwort ({response.status_code}): {response.text}")
    except requests.exceptions.RequestException as e:
        print("API-Fehler:", e)

def button_callback(channel):
    global mode_index
    mode_index = (mode_index + 1) % len(modes)
    print(f"Modus gewechselt: {modes[mode_index].upper()}")
    update_display_mode()

GPIO.add_event_detect(SW, GPIO.FALLING, callback=button_callback, bouncetime=300)

try:
    while True:
        clk_state = GPIO.input(CLK)
        dt_state = GPIO.input(DT)

        if clk_state != last_clk_state and clk_state == GPIO.HIGH:
            if dt_state != clk_state:
                move_axis(modes[mode_index], STEP_SIZES[modes[mode_index]])
            else:
                move_axis(modes[mode_index], -STEP_SIZES[modes[mode_index]])
            time.sleep(0.02)  # Entprellung

        last_clk_state = clk_state
        time.sleep(0.001)

except KeyboardInterrupt:
    print("Beendet")
    GPIO.cleanup()
    splash.pop()
