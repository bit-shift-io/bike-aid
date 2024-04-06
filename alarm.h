// singleton example
// https://hnrck.io/post/singleton-design-pattern/

#pragma once
#include "Arduino.h"

class Alarm {

  public:
    void update();
    
    void setEnable(bool);
    static void interruptHandler();
    // singleton stuff + delete the functions
    static Alarm& instance();
    Alarm(const Alarm&) = delete;
    Alarm(Alarm&&) = delete;
    Alarm& operator=(const Alarm&) = delete;
    Alarm& operator=(Alarm&&) = delete;

  private:
    // pins
    const byte INPUT_PIN = 2; // Nano 2 + 3 are interrupts
    bool enabled = false;

    // how many readings in 1 second
    const int INTERVAL = 1000;
    unsigned long last_interval = 0;
    const int SENSITIVITY = 40;
    byte trigger_count = 0;
    bool tigger_state = 0; // 0 - low, 1 - high

    // interupt
    volatile int interrupt_count = 0;
    
    // warnings
    const int WARN_INTERVAL = 10000;
    unsigned long last_warn_interval = 0;
    const byte WARNINGS = 2;
    byte warn_count = 0;

    // class
    static Alarm& rInstance;
    Alarm();
    ~Alarm();
};