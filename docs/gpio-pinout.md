| GPIO      | Pin # | Funktion        | Verwedt für        | Schnittstelle |
| --------- | ----- | --------------- | ------------------ | ------------- |
| 3v3 Power | 1     | Stromversorgung | Rotary Encoder VCC | -             |
| GND       | 6     | Masse           | Rotary Encoder GND | -             |
| GPIO17    | 11    | Eingang         | Rotary Encoder DT  | GPIO          |
| GPIO18    | 12    | Eingang         | Rotary Encoder CLK | GPIO          |
| GPIO27    | 13    | Eingang         | Rotary Encoder SW  | GPIO          |
| 3v3 Power | 17    | Stromversorgung | TFT VCC            | -             |
| GND       | 9     | Masse           | TFT GND            | -             |
| GPIO10    | 19    | SPI MOSI        | TFT MOSI           | SPI           |
| GPIO11    | 23    | SPI SCK         | TFT SCK            | SPI           |
| GPIO16    | 36    | Ausgang         | TFT Backlight LED  | GPIO          |
| GPIO9     | 21    | SPI MISO        | TFT MISO           | SPI           |
| GPIO24    | 18    | Ausgang         | TFT DC             | GPIO          |
| GPIO25    | 22    | Ausgang         | TFT RST            | GPIO          |
| GPIO8     | 24    | SPI CS0         | TFT CS             | SPI           |
| GPIO 14   | 8     | UART TX         | Sangaboard         | UART          |
| GPIO 15   | 10    | UART RX         | Sangaboard         | UART          |

Mögliche Konflikte mit dem Sangaboard:

- **GPIO24** und **GPIO25** sind nur dann zu verwenden, wenn keine Firmware-Upgrades für das Sangaboard geplant sind.
- **GPIO14** und **GPIO15** (UART) sollten nicht verwendet werden, diese sind für die Kommunikation mit dem Sangaboard reserviert.
