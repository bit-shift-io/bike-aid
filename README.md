# bike-aid
ebike throttle aid using arduino


### Purpose

Arduino Throttle Correction for 3-wire thottles

This project is suitable for electric vehicles with a non-programmable controller. 
The arduino is inserted between a 3-wire throttle and the controller.

* Provides smoothing for aggressive throttle response by slowing down changes in output (wheelie/jerk control)
* Reduces throttle dead-band/deadzone by mapping input to output values as per configuration
* Optionally provides an adjustable speed limit (hardcoded or potentiometer/switch)

### Wiring and Implementation

Wiring is very simple.  Smoothing capacitor can be any value that eliminates
jumping of motor speed at steady throttle.  I used a 10uF for my setup.  
A high value may cause the throttle to stay high longer than desired. 

### Tuning

Tune behaviors according to your throttle's actual values, which you can see if you watch serial output.

If you don't want speed limiting, comment out `#LIMIT_ENABLE`

**Note** that when plugged into USB, Arduino is running at 5V, but when powered by the motor controller,
it is most likely running on 4.0V-4.5V.  This changes the numbers and behavior a little when you unplug USB.

### Programming attiny85
https://www.instructables.com/How-to-Program-an-Attiny85-From-an-Arduino-Uno/
Files > Preferences > Additional Board Managers ...
https://raw.githubusercontent.com/damellis/attiny/ide-1.6.x-boards-manager/package_damellis_attiny_index.json
Restart the IDE
Tools > Boards > Board Manager
Install 'attiny'
Restart IDE
Tools > Boards > attiny

