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
sudo usermod -aG uucp $USER # not sure if we need this one?

# need the udev rule
sudo cp 69-probe-rs.rules /etc/udev/rules.d/
sudo udevadm control --reload
sudo udevadm trigger

# Debugger - not required
#sudo pacman -S gdb-common openocd

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
Download the svd ```https://github.com/NordicSemiconductor/nrfx/blob/master/mdk/nrf52840.svd```

## Reset device
Double tap rest to ground within 0.5 seconds to reset board

## Stuck in boot loop (bad flash)
Reflash the firmware above

## Pin Guide
* P0.31 - LED
* P0.29 - Piezo
* P0.20 - Manual Override
* P0.17 - Brake
* P0.09 - Speed
* P0.10 - Power Switch
* P0.10 - Light
* P1.11 - Horn
* P0.06 - I2C SCL - Orange ( Green on breadboard )
* P0.08 - I2C SDA - Yellow

## Pin Notes
* nfc-pins-as-gpio Allow using the NFC pins as regular GPIO P0_09/P0_10 on nRF52
* reset-pin-as-gpio Allow using the RST pin as a regular GPIO P0_18

* P0.13 controls vcc output on/off 3.3v
* P0.14-0.16 set low resets ?
* p0.15 Debug LED

## Hardware Todo
* try a smaller pulldown on the throttle module, replace 100k with 47k to see if it helps with floating throttle
* brake plug seems wobbly
* reset push button - hw
* speedo - hardware/oscilliscope
* brake supply 5v with diode to drop 0.7v. then can setup parkbrake to turn off power.

## App Todo
* disconnect while connecting causes multiple instances of scanner

## MCU Todo
* patch qeue predicate filter
* attach debugger to running mcu
* sport mode acceleration values 10% higher?
* alarm
* ble tracker
* odometer/speed

## Links

https://github.com/peterkrull/quad/blob/main/software/rusty-quad/src/
