/*
  Bike Aid

  This is an ebike tool which does the following:
  - Bluetooth interface
  - Power on/off
  - Battery power meter
  - Alarm system
  - Speedometer
  - Throttle smoothing
  - Throttle limiting
  - Throttle deadband adjustment

  Bronson Mathews, 2024
*/

#include "alarm.h"
#include "throttle.h"
#include "speed.h"
#include "battery.h"


void setup() {
  //analogReference(EXTERNAL);
  Serial.begin(9600);
  //Alarm::instance().setEnable(true);
}

void loop() {
  //Alarm::instance().update();
  //Throttle::instance().update();
  Speed::instance().update();
}