# Installation Guide

## Wiring
Raspberry pi debug probe (port D)   ->  NRF
TX/SC (orange - out from probe)     ->  CLK (orange - SWC) 
GND (black)                         ->  GND (black)
RX/SD (yellow - input to I/O)       ->  DIO (yellow - SWD)

## Tools
```Bash
# Probe-RS
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh

# USB Permissions
sudo usermod -aG dialout $USER
sudo cp 69-probe-rs.rules /etc/udev/rules.d/
sudo udevadm control --reload
sudo udevadm trigger

# Debugger
sudo pacman -S gdb-common openocd

# list debug probe
probe-rs list
```


## Getting rust working
Run '''cargo build''' in the rust project root directory  
Then '''cargo run'''


## Error: No connected probes were found.
Need to configure udev rules in linux
```
probe-rs list
ls /dev/ttyACM*
lsusb
```

## Erase the chip
```
probe-rs erase --chip nRF52840_xxAA
```

## Mount usb device
```
reset to gnd 2x within 0.5s
```

## Get latest rust toolchain
```
rustup update
```

## Update nrf42840 firmware for bluetooth
Downloaded from 
```
https://www.nordicsemi.com/Products/Development-software/s140/download
```

Flash using the commands:
```
probe-rs erase --chip nrf52840_xxAA --allow-erase-all
probe-rs download --verify --binary-format hex --chip nRF52840_xxAA s140_nrf52_7.3.0_softdevice.hex
```

## Debug support
Install probe-rs visual studio plugin


## Links
https://github.com/joseph-montanez/pico-w-rust-starter-kit

https://www.raspberrypi.com/documentation/microcontrollers/debug-probe.html

https://wiki.icbbuy.com/doku.php?id=developmentboard:nrf52840

https://github.com/joric/nrfmicro/wiki/Alternatives#supermini-nrf52840


## temp notes

https://github.com/peterkrull/quad/blob/main/software/rusty-quad/src/

https://github.com/embassy-rs/embassy/blob/main/examples/nrf52840/src/bin/channel_sender_receiver.rs

https://dev.to/apollolabsbin/sharing-data-among-tasks-in-rust-embassy-synchronization-primitives-59hk

https://hegdenu.net/posts/understanding-async-await-3/

https://github.com/embassy-rs/nrf-softdevice/blob/master/examples/src/bin/ble_bas_central.rs

https://dev.to/theembeddedrustacean/embedded-rust-embassy-i2c-temperature-sensing-with-bmp180-6on

https://github.com/tclarke/mcp4725/tree/master
