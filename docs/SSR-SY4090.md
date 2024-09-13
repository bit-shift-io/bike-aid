# Solid State Relay SY4090

https://www.jaycar.co.nz/pcb-mount-solid-state-dil-relay/p/SY4090

## Notes

https://forum.arduino.cc/t/switch-55-60v-16ma-advice-optocouplers/1300792/10

## Calculations

V=IR
R=V/I
Forward Voltage LED = 1.2-1.3v
3.3v - 1.3v = 2v - 2.1v
R=2v/0.006A = 330ohm

## Pins
1 = +3.3v
2 = GND
3 = N/C
4/6 = Drain - Battery Input +60v 
5 = Source - Switched Output
