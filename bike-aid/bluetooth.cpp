#include "Arduino.h"
#include "bluetooth.h"


Bluetooth::Bluetooth() {
  pinMode(INPUT_PIN, INPUT);
}


static Bluetooth& Bluetooth::instance() {
  static auto &&rInstance = Bluetooth();
  return rInstance;
}


void Bluetooth::update() {
  unsigned long time = millis();

  if (time - last_interval > INTERVAL) {
    last_interval = time;

    byte input = digitalRead(INPUT_PIN);
    if (input == LOW) {}
    else
      Serial.print("Bluetooth click");
  }
}