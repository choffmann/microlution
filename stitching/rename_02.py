import os
import re
import shutil
# Pfad zum Ordner mit den Bildern
quelle_ordner = 'img/5/tiles'  # <-- hier anpassen
ziel_ordner = 'img/5/tiles/umbenannt'  # <-- hier anpassen

# Anzahl der Spalten (x-Werte: 0 bis 3 → 4 Spalten)
anzahl_spalten = 4

# Regex zur Erkennung von 'tile_X_Y.ext'
muster = re.compile(r'^tile_(\d+)_(\d+)(\.\w+)$')

# Zielordner erstellen, falls nicht vorhanden
os.makedirs(ziel_ordner, exist_ok=True)

# Dateien durchgehen
for datei in os.listdir(quelle_ordner):
    match = muster.match(datei)
    if match:
        x, y, ext = int(match.group(1)), int(match.group(2)), match.group(3)
        neues_x = anzahl_spalten - 1 - x
        neuer_name = f"tile_{neues_x}_{y}{ext}"

        quellpfad = os.path.join(quelle_ordner, datei)
        zielpfad = os.path.join(ziel_ordner, neuer_name)

        shutil.copy2(quellpfad, zielpfad)
        print(f"Kopiert und umbenannt: {datei} → {neuer_name}")
    else:
        print(f"Übersprungen (passt nicht): {datei}")

print("Fertig!")
