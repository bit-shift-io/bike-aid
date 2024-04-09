#include "Arduino.h"
#include "alarm.h"


Alarm::Alarm() {
  pinMode(INPUT_PIN, INPUT);
}


Alarm& Alarm::instance() {
  static auto &&rInstance = Alarm();
  return rInstance;
}


void Alarm::update() {
  if (!enabled)
    return;

  // check sensor every update
  // this saves doing an interrupt
  byte input = digitalRead(INPUT_PIN);
  if (input == HIGH && input != tigger_state)
    trigger_count++;
  tigger_state = input;

  // every interval check
  unsigned long time = millis();
  if (time - last_interval > INTERVAL) {
    last_interval = time;

    if (trigger_count > SENSITIVITY) {
      warn_count++;
      Serial.print("warn:");
      Serial.println(warn_count);
    }

    trigger_count = 0;
  }

  // trigger the alarm
  if (warn_count > WARNINGS)
    Serial.println("ALARM!");

  
  if (warn_count == 0)
    return;

  // warning update at slower rate
  if (time - last_warn_interval > WARN_INTERVAL) {
    last_warn_interval = time;
    warn_count--; // decement
    Serial.println("warn decrease");
  }
}


void Alarm::setEnable(bool enable) {
  if (enable)
    attachInterrupt(digitalPinToInterrupt(INPUT_PIN), interruptHandler, RISING);
  else
    detachInterrupt(digitalPinToInterrupt(INPUT_PIN));

  enabled = enable;
}


void Alarm::interruptHandler() {
  // IRAM_ATTR for esp32?
  Alarm::instance().interrupt_count++;
}