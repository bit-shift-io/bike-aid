#pragma once
#include "Arduino.h"
#include "bluetooth.h"

class Clock {

  public:
    void update();
    void setEnable(bool);

    // singleton stuff + delete the functions
    static Clock& instance();
    Clock(const Clock&) = delete;
    Clock(Clock&&) = delete;
    Clock& operator=(const Clock&) = delete;
    Clock& operator=(Clock&&) = delete;

  private:
    bool enabled = false;
    int trip_duration = 0;
    // Delay between loops in ms
    const int INTERVAL = 6000; // 1 minute
    unsigned long last_interval = 0;

    static Clock& rInstance;
    Clock();
    //~Clock();
  };