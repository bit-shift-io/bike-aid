// singleton example
// https://hnrck.io/post/singleton-design-pattern/

#pragma once
#include "Arduino.h"

class Alarm {

  public:
    /*
    void update(); // testing
    unsigned long last_interval = 0; // testing
    */
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

    static Alarm& rInstance;
    Alarm();
    ~Alarm();

};