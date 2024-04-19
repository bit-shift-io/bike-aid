#include "battery.h"


Battery::Battery() {
  pinMode(INPUT_PIN, INPUT);
}


Battery& Battery::instance() {
  static auto &&rInstance = Battery();
  return rInstance;
}


void Battery::set_enable(bool enable) {
  Log.println("battery enable " + enable);
  enabled = enable;
}


void Battery::update() {
  if (!enabled)
    return;
    
  unsigned long time = millis();

  if (time - last_interval > INTERVAL) {
    last_interval = time;

    byte input = analogRead(INPUT_PIN);
    // todo: battery meter
  }
}