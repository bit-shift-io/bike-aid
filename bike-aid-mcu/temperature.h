#pragma once
#include "Arduino.h"
#include "bluetooth.h"
#include <OneWire.h>
#include <DallasTemperature.h>

class Temperature {

  public:
    void update();
    void set_enable(bool);

    // singleton stuff + delete the functions
    static Temperature& instance();
    Temperature(const Temperature&) = delete;
    Temperature(Temperature&&) = delete;
    Temperature& operator=(const Temperature&) = delete;
    Temperature& operator=(Temperature&&) = delete;

  private:
    const byte TEMPERATURE_PIN = 6; 

    DallasTemperature temperature_sensors;
    
    bool enabled = false;
    unsigned long start_time = 0;
    
    // Delay between loops in ms
    const int INTERVAL = 60000; // 1 minute
    unsigned long last_interval = 0;

    static Temperature& rInstance;
    Temperature();
    //~Temperature();
  };