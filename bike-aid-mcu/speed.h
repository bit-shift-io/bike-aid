#pragma once
#include "Arduino.h"

class Speed {

  public:
    void update();
    void setEnable(bool);

    // singleton stuff + delete the functions
    static Speed& instance();
    Speed(const Speed&) = delete;
    Speed(Speed&&) = delete;
    Speed& operator=(const Speed&) = delete;
    Speed& operator=(Speed&&) = delete;

  private:
    // pins
    const byte INPUT_PIN = 12;
    bool enabled = false;

    // update interval
    const int INTERVAL = 250;
    unsigned long last_interval = 0;

    byte last_state = LOW;

    unsigned long rotation_time = 0;
    unsigned long last_rotation_time = 0;
    float speed = 0;
    int rotations = 0;

    // smoothing
    const int SMOOTH_FACTOR = 3;

    // measure wheel circumference for more accurate speed
    const float WHEEL_CIRCUMFERENCE = 997.46; // 12.5inch diameter -> 317.5mm diameter -> 997.46mm circumference

    static Speed& rInstance;
    Speed();
    //~Speed();
  };