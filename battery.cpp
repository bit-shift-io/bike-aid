#include "Arduino.h"
#include "battery.h"


Battery::Battery() {
  pinMode(INPUT_PIN, INPUT);
}


static Battery& Battery::instance() {
  static auto &&rInstance = Battery();
  return rInstance;
}


void Battery::update() {
  unsigned long time = millis();

  if ((last_interval + INTERVAL) < time) {
    last_interval = time;

    byte input = digitalRead(INPUT_PIN);
    if (input == LOW) {}
    else
      Serial.print("Battery click");
  }
}