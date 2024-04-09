#include "Arduino.h"
#include "battery.h"


Battery::Battery() {
  pinMode(INPUT_PIN, INPUT);
}


Battery& Battery::instance() {
  static auto &&rInstance = Battery();
  return rInstance;
}


void Battery::update() {
  unsigned long time = millis();

  if (time - last_interval > INTERVAL) {
    last_interval = time;

    byte input = digitalRead(INPUT_PIN);
    if (input == LOW) {}
    else
      Serial.print("Battery click");
  }
}