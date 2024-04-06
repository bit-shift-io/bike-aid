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
  // check sensor every update for rising
  byte input = digitalRead(INPUT_PIN);
  if (input != last_state) {
    if (input == HIGH) {
      //Serial.println("rising");
      rotations++;

      last_rotation_time = rotation_time - millis();
      rotation_time = millis();
    }
    last_state = input;
  }

  unsigned long time = millis();
  if (time - last_interval > INTERVAL) {
    last_interval = time;

    
    // calculate speed using time
    //float delta_time = rotation_time - last_rotation_time; // mms
    float speed = (1000.0f / last_rotation_time) * WHEEL_CIRCUMFERENCE; // mm per second
    float kmph = speed * 0.0036f; // 1mm/s = 0.0036km/s
    Serial.print("time:"); Serial.println(kmph);
    Serial.println(last_rotation_time);

    // calculate speed using rotations
    float speed2 = WHEEL_CIRCUMFERENCE * rotations; // mm per second
    float kmph2 = speed2 * 0.0036f; // 1mm/s = 0.0036km/s
    Serial.print("rotations:"); Serial.println(kmph2);
    Serial.println(rotations);

    rotations = 0;
  }
}