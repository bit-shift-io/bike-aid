#pragma once
#include "Arduino.h"

class Speed {

  public:
    void update();
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

    const int INTERVAL = 1;
    unsigned long last_interval = 0;

    const float WHEEL_CIRCUMFERENCE = 997.46; // 12.5inch diameter -> 317.5mm diameter -> 997.46mm circumference

    static Speed& rInstance;
    Speed();
    ~Speed();

  };