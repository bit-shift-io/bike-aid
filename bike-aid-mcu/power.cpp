#include "Arduino.h"
#include "power.h"


Power::Power() {
    pinMode(POWER_PIN, OUTPUT);
    pinMode(LIGHT_PIN, OUTPUT);
}


Power& Power::instance() {
  static auto &&rInstance = Power();
  return rInstance;
}


void Power::set_enable(bool enable) {
  enabled = enable;
}


void Power::toggle_power() {
  digitalWrite(POWER_PIN, !digitalRead(POWER_PIN));
}


void Power::toggle_lights() {
  if (!enabled)
    return;
  digitalWrite(LIGHT_PIN, !digitalRead(LIGHT_PIN));
}