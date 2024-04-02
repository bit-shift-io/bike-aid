// singleton example
// https://hnrck.io/post/singleton-design-pattern/

#pragma once
#include "Arduino.h"

class Alarm {

  public:
    static Alarm& instance();
    void setEnable(bool);
    static void interruptHandler(void);
    // Delete the functions
    Alarm(const Alarm&) = delete;
    Alarm(Alarm&&) = delete;
    Alarm& operator=(const Alarm&) = delete;
    Alarm& operator=(Alarm&&) = delete;

  private:
    const byte SENSOR_PIN = 0;
    bool enabled = false;
    static Alarm* pInstance;
    static Alarm& rInstance;
    Alarm();
    ~Alarm();

};