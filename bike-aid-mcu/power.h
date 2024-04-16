#pragma once
#include "Arduino.h"
#include "bluetooth.h"

class Power {
  
  public:
    void update();
    void set_enable(bool);
    void toggle_power();
    void toggle_lights();

    // singleton stuff + delete the functions
    static Power& instance();
    Power(const Power&) = delete;
    Power(Power&&) = delete;
    Power& operator=(const Power&) = delete;
    Power& operator=(Power&&) = delete;

  private:
    const byte POWER_PIN = 5;
    const byte LIGHT_PIN = 5;
    bool enabled = false;

    static Power& rInstance;
    Power();
    //~Power();
  };