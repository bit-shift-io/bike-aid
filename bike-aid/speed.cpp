#include "Arduino.h"
#include "speed.h"


Speed::Speed() {
  pinMode(INPUT_PIN, INPUT);
}


static Speed& Speed::instance() {
  static auto &&rInstance = Speed();
  return rInstance;
}


void Speed::setEnable(bool enable) {
  enabled = enable;
}


void Speed::update() {
  if (!enabled)
    return;

  unsigned long time = millis();

  // check sensor every update for rising
  byte input = digitalRead(INPUT_PIN);
  if (input != last_state) {
    if (input == HIGH) {
      //Serial.println("rising");
      //rotations++;

      last_rotation_time = rotation_time;
      rotation_time = time;
    }
    last_state = input;
  }

  
  if (time - last_interval > INTERVAL) {
    last_interval = time;

    // calculate speed using time
    // filter values to high or to low
    float delta_time = rotation_time - last_rotation_time; // ms
    if (delta_time > 20 && delta_time < 5000) {
      float mm_sec = (1000.0f / delta_time) * WHEEL_CIRCUMFERENCE; // mm per second
      float cur_speed = mm_sec * 0.0036f; // 1mm/s = 0.0036km/s 

      // do smoothing
      float delta_speed = cur_speed - speed; // calc difference btween speeds
      float adjust = (float) delta_speed / (float) SMOOTH_FACTOR;
      speed += adjust;

      Serial.print("kph:"); Serial.println(speed);
    }
  }
}