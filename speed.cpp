#include "Arduino.h"
#include "speed.h"


Speed::Speed() {
  pinMode(INPUT_PIN, INPUT);
}


static Speed& Speed::instance() {
  static auto &&rInstance = Speed();
  return rInstance;
}


void Speed::update() {
  unsigned long time = millis();

  if ((last_interval + INTERVAL) < time) {
    last_interval = time;

    byte input = digitalRead(INPUT_PIN);
    if (input == LOW) {}
    else
      Serial.print("speed click");
  }
}