```bash
# mount /boot
sudo mount -o loop,offset=4194304 2023-04-13-raspbian-openflexure-buster-armhf.img /mnt/rpi-boot
# mount /
sudo mount -v -o offset=266338304 -t ext4 2023-04-13-raspbian-openflexure-buster-armhf.img /mnt/rpi-boot
```

offset => `fdisk -l <img>` => anfang _einheit sektoren. (8192_ 512 = 4194304)

Weitere benötigte Dateien, welche in /boot zu finden sind:

- `bcm2710-rpi-3-b.dtb`
- `kernel8.img`

### User password reset

`/` von img mounten und in `/etc/passwd` das `...:x:...` entfernen. Das entfernen von `:x:` entfernt das kennwort vom benutzer.

```bash
# root:x:0:0:root:/root:/bin/bash
#      ^--- entfernen
# root::0:0:root:/root:/bin/bash
```

## QEMU

`CTRL-a + c` => qemu console
`CTRL-a + x` => qemu terminate
`system_powerdown` => shutdown guest

Resize img für qemu (qemu mag anscheind nur images in der größe `^2`)

```bash
qemu-img resize 2023-04-13-raspbian-openflexure-buster-armhf.img 8G
```

Image Partition muss dennoch angepasst werden

```bash
qemu-system-aarch64 \
    -machine raspi3b \
    -cpu cortex-a72 \
    -nographic \
    -m 1G \
    -smp 4 \
    -dtb bcm2710-rpi-3-b.dtb \
    -kernel kernel8.img \
    -append "rw earlyprintk loglevel=8 console=ttyAMA0,115200 dwc_otg.lpm_enable=0 root=/dev/mmcblk0p2 rootdelay=1" \
    -netdev user,id=net0,hostfwd=tcp::5000-:5000,hostfwd=tcp::2222-:22 \
    -device usb-net,netdev=net0 \
    -device usb-host,hostbus=5,hostaddr=6 \
    -sd 2023-04-13-raspbian-openflexure-buster-armhf.img
```

### Resize partitions

```bash
sudo fdisk /dev/mmcblk0
```

```
Command (m for help): d
Partition number (1,2, default 2): 2
Partition 2 has been deleted.

Command (m for help): n
Partition type
   p   primary (1 primary, 0 extended, 3 free)
   e   extended (container for logical partitions)
Select (default p): e
Partition number (2-4, default 2): 2
First sector (2048-16777215, default 2048): 520192
Last sector, +/-sectors or +/-size{K,M,G,T,P} (520192-16777215, default 16777215):

Created a new partition 2 of type 'Extended' and of size 7.8 GiB.
Partition #2 contains a ext4 signature.

Do you want to remove the signature? [Y]es/[N]o: n
```

```bash
sudo resize2fs /dev/mmcblk0p2
```

## OpenFlexure Server in qemu

mit `DummyCamera` kann `vchiq` (Pi Kamera) umgehen und `DummyStage` (Motoren) deaktiviert werden.

```json
{
  "camera": {
    "type": "DummyCamera"
  },
  "stage": {
    "type": "DummyStage",
    "port": null
  }
}
```
