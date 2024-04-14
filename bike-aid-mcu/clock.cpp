#include "Arduino.h"
#include "clock.h"


Clock::Clock() {
}


Clock& Clock::instance() {
  static auto &&rInstance = Clock();
  return rInstance;
}


void Clock::setEnable(bool enable) {
  enabled = enable;
}


void Clock::update() {
  if (!enabled)
  return;

  unsigned long time = millis();
  if (time - last_interval > INTERVAL) {
    last_interval = time;

    int all_minutes = time / 60000;
    int run_hours = all_minutes / 60;
    int run_minutes = all_minutes - (run_hours * 60);
    // todo: store hr and min in 2 byte or a string?
  }
}