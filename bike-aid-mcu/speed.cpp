#include "Arduino.h"
#include "speed.h"


Speed::Speed() {
  pinMode(INPUT_PIN, INPUT);
}


Speed& Speed::instance() {
  static auto &&rInstance = Speed();
  return rInstance;
}


void Speed::set_enable(bool enable) {
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
      rotations++;
      last_rotation_time = rotation_time;
      rotation_time = time;

      /*
      // calculate instant speed for speed limiter here??
      delta_time = rotation_time - last_rotation_time;
      if (delta_time > 20 && delta_time < 5000) {
        // mm per second -> kms (1mm/s = 0.0036km/s)
        instant_speed = (1000.0f / delta_time) * WHEEL_CIRCUMFERENCE * 0.0036f; 
      }
      */
    }
    last_state = input;
  }

  
  if (time - last_interval > INTERVAL) {
    last_interval = time;

    // calculate speed using time, then apply smoothing
    // filter values to high or to low
    delta_time = rotation_time - last_rotation_time; // ms
    if (delta_time > 20 && delta_time < 5000) {
      // mm per second -> kms (1mm/s = 0.0036km/s)
      instant_speed = (1000.0f / delta_time) * WHEEL_CIRCUMFERENCE * 0.0036f; 

      // do smoothing
      float delta_speed = instant_speed - smooth_speed; // calc difference btween speeds
      float adjust = (float) delta_speed / (float) SMOOTH_FACTOR;
      smooth_speed += adjust;

      // trip odometer
      int trip_distance = (rotations * WHEEL_CIRCUMFERENCE) / 1000000; // mm to km 

      // send data
      Bluetooth::instance().set_value("trip_distance", std::to_string(trip_distance));
      Bluetooth::instance().set_value("speed", std::to_string(smooth_speed));
    }
  }
}