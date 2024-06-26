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


## Links
https://github.com/joseph-montanez/pico-w-rust-starter-kit

https://www.raspberrypi.com/documentation/microcontrollers/debug-probe.html

https://wiki.icbbuy.com/doku.php?id=developmentboard:nrf52840

https://github.com/joric/nrfmicro/wiki/Alternatives#supermini-nrf52840


## temp notes
https://github.com/esp-rs/esp-hal/tree/main/examples
https://github.com/peterkrull/quad/blob/main/software/rusty-quad/src/task_blinker.rs
https://github.com/embassy-rs/embassy/blob/main/examples/nrf52840/src/bin/channel_sender_receiver.rs
https://github.com/apollolabsdev/ESP32C3/blob/main/embassy_examples/gpio/src/main.rs
https://dev.to/apollolabsbin/sharing-data-among-tasks-in-rust-embassy-synchronization-primitives-59hk
https://hegdenu.net/posts/understanding-async-await-3/
