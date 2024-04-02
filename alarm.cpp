#include "Arduino.h"
#include "alarm.h"


Alarm::Alarm() {
  pinMode(SENSOR_PIN, INPUT);
}


static Alarm& Alarm::instance() {
  static auto &&rInstance = Alarm();
  return rInstance;
}


void Alarm::setEnable(bool enable) {
  if (enable) {
    attachInterrupt(SENSOR_PIN, rInstance.interruptHandler, RISING);
  }
  else {
    detachInterrupt(SENSOR_PIN);
  }
}


static void Alarm::interruptHandler(void) {
  // IRAM_ATTR for esp32?
}