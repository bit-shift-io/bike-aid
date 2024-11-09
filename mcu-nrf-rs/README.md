# Installation Guide

## Wiring
```
Raspberry pi debug probe (port D)   ->  NRF
TX/SC (orange - out from probe)     ->  CLK (orange - SWC) 
GND (black)                         ->  GND (black)
RX/SD (yellow - input to I/O)       ->  DIO (yellow - SWD)
```

## Tools
```Bash
# Probe-RS
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh

# USB Permissions
sudo usermod -aG uucp $USER
sudo cp 69-probe-rs.rules /etc/udev/rules.d/
sudo udevadm control --reload
sudo udevadm trigger

# Debugger
sudo pacman -S gdb-common openocd

# list debug probe
probe-rs list
```


## Getting rust working
Run ```cargo build``` in the rust project root directory  
Then ```cargo run```  
or ```cargo embed``` for user input terminal


## Error: No connected probes were found.
Need to configure udev rules in linux
```bash
probe-rs list
ls /dev/ttyACM*
lsusb
```

## Erase the chip
```bash
probe-rs erase --chip nRF52840_xxAA
```

## Get latest rust toolchain
```bash
rustup update
```

## Update nrf42840 firmware for bluetooth
Downloaded from 
```
https://www.nordicsemi.com/Products/Development-software/s140/download
```

Flash using the commands:
```bash
probe-rs erase --chip nrf52840_xxAA --allow-erase-all
probe-rs download --verify --binary-format hex --chip nRF52840_xxAA s140_nrf52_7.3.0_softdevice.hex
```

## Debug support
Install probe-rs visual studio plugin

## Reset device
Double tap rest to ground within 0.5 seconds to reset board

## Stuck in boot loop (bad flash)
Reflash the firmware above

## Links

https://github.com/peterkrull/quad/blob/main/software/rusty-quad/src/
