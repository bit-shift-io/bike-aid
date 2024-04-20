#pragma once
#include <string>
//#include "bluetooth.h"
#include "log.h"

class Clock {

  public:
    void update();
    void set_enable(bool);

    // singleton stuff + delete the functions
    static Clock& instance();
    Clock(const Clock&) = delete;
    Clock(Clock&&) = delete;
    Clock& operator=(const Clock&) = delete;
    Clock& operator=(Clock&&) = delete;

  private:
    bool enabled = false;
    unsigned long start_time = 0;
    
    // Delay between loops in ms
    const int INTERVAL = 60000; // 1 minute
    unsigned long last_interval = 0;

    static Clock& rInstance;
    Clock();
    //~Clock();
  };