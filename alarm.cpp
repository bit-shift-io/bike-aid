#include "Arduino.h"
#include "alarm.h"


Alarm::Alarm() {
  pinMode(INPUT_PIN, INPUT);
}


static Alarm& Alarm::instance() {
  static auto &&rInstance = Alarm();
  return rInstance;
}

/*
void Alarm::update() {
  unsigned long time = millis();
  if ((last_interval + 100) < time) {
    last_interval = time;

    byte input = digitalRead(INPUT_PIN);
    Serial.print("Alarm:");Serial.print(input);
    Serial.println();
  }
}
*/

void Alarm::setEnable(bool enable) {
  if (enable) {
    attachInterrupt(digitalPinToInterrupt(INPUT_PIN), interruptHandler, CHANGE);
  }
  else {
    detachInterrupt(INPUT_PIN);
  }
}


static void Alarm::interruptHandler() {
  // IRAM_ATTR for esp32?
  Serial.println("alarm!");
}