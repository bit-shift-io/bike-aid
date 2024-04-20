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
  Log.print("power enable ");Log.println(enable);
  enabled = enable;
  digitalWrite(POWER_PIN, enabled);
  //Bluetooth.set_value("power_system", std::to_string(enabled));
}


void Power::set_lights_enable(bool enable) {
  lights_enabled = enable;
  digitalWrite(LIGHT_PIN, lights_enabled);
  //Bluetooth.set_value("power_lights", std::to_string(lights_enabled));
}


bool Power::get_enable() {
    return enabled;
}


bool Power::get_lights_enable() {
    return lights_enabled;
}