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

#include "throttle.h"

Throttle throttle;

void setup() {
  //analogReference(EXTERNAL);
  Serial.begin(9600);
  throttle.init();
}

void loop() {
  throttle.update();
}