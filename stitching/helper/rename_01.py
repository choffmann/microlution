import os
import re
import shutil

# 📂 Eingabe- und Ausgabeordner (anpassen!)
input_dir = "img/4/tiles_original"
output_dir = "img/4/tiles"

# 📐 Tile-Größe in Pixeln
tile_width = 2000
tile_height = 2000

# === 🧠 Dateiname: timestamp_x_y_z.jpeg ===
filename_pattern = re.compile(r".*_(\d+)_(\d+)_(\-?\d+)\.jpeg", re.IGNORECASE)

# 📁 Zielordner erstellen
os.makedirs(output_dir, exist_ok=True)

# 🔍 Optional: Maximalen y-Wert bestimmen, falls du y invertieren willst
# (Deaktiviert – nur aktivieren, wenn y=0 UNTEN ist)
max_y = max(
    int(match.group(2)) for filename in os.listdir(input_dir)
    if (match := filename_pattern.match(filename))
)

# 🔁 Dateien verarbeiten
for filename in os.listdir(input_dir):
    match = filename_pattern.match(filename)
    if match:
        x = int(match.group(1))  # horizontal → Spalte
        y = int(match.group(2))  # vertikal → Zeile

        col = x // tile_width
        # row = y // tile_height  # Standardrichtung (y=0 ist oben)

        # Falls dein Koordinatensystem invertiert ist, verwende stattdessen:
        row = (max_y - y) // tile_height

        new_filename = f"tile_{row}_{col}_overlap.jpg"
        src_path = os.path.join(input_dir, filename)
        dst_path = os.path.join(output_dir, new_filename)

        # Datei kopieren und umbenennen
        shutil.copyfile(src_path, dst_path)
        print(f"✅ {filename} → {new_filename}")
    else:
        print(f"⚠️ Kein Match für: {filename}")
