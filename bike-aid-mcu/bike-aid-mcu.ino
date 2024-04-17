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
  - Temperature

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
#include "temperature.h"


void setup() {
  Serial.begin(9600);

  // order important
  Store::instance();
  Bluetooth::instance();
  System::instance();

  // enable modules
  // testing set true
  Temperature::instance().set_enable(true);
  Alarm::instance().set_enable(true);
  Speed::instance().set_enable(true);
  Throttle::instance().set_enable(false);
  Power::instance().set_enable(true);
  Clock::instance().set_enable(true);
}

void loop() {
  Alarm::instance().update();
  Throttle::instance().update();
  Speed::instance().update();
  Clock::instance().update();
  Temperature::instance().update();
}