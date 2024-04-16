/*
  Bike Aid

  This is an ebike tool which does the following:
  - Bluetooth interface
  - Power on/off
  - Lights on/off
  - Battery power meter
  - Alarm system
  - Speedometer
  - Trip Odometer
  - Trip duration
  - Throttle smoothing
  - Throttle limiting / speed limiting
  - Throttle deadband adjustment

  Bronson Mathews, 2024
*/

#include "alarm.h"
#include "clock.h"
#include "throttle.h"
#include "speed.h"
#include "battery.h"
#include "bluetooth.h"
#include "system.h"
#include "power.h"


void setup() {
  Serial.begin(115200);

  // enable modules
  // testing set true
  Alarm::instance().set_enable(true);
  Speed::instance().set_enable(true);
  Throttle::instance().set_enable(true);
  Power::instance().set_enable(true);
  Clock::instance().set_enable(true);
  
  // order important
  Store::instance();
  Bluetooth::instance();
  System::instance();
}

void loop() {
  Alarm::instance().update();
  Throttle::instance().update();
  Speed::instance().update();
  Clock::instance().update();
}