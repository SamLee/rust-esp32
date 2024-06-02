# Rust ESP 32

Playing around with rust and the esp32

## Notes
Used `espup` and have to source the `export-esp.sh` file that it creates.

## Notes for windows
If using wsl use `usbipd` to forward the device to wsl.

Run `usbipd list` to get the busid of the device.

Run `usbipd wsl attach --busid 5-2` to attach the device.
