/*
  Bike Aid

  This is an ebike tool which does the following:
  - Bluetooth interface
  - Power on/off?
  - Lights on/off?
  - Battery power meter
  - Alarm system
  - Speedometer/odometer
  - Trip time
  - Throttle smoothing
  - Throttle limiting
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


void setup() {
  //analogReference(EXTERNAL);
  //Serial.begin(9600);
  Serial.begin(115200);

  // enable modules
  //Alarm::instance().setEnable(true);
  Speed::instance().setEnable(true);
  //Throttle::instance().setEnable(true);
  Clock::instance().setEnable(true);
  
  // order important for these ones
  Store::instance(); // init
  Bluetooth::instance();
  System::instance(); // init
}

void loop() {
  Alarm::instance().update();
  Throttle::instance().update();
  Speed::instance().update();
  Clock::instance().update();
  //Bluetooth::instance().update();
  //System::instance().update();
}