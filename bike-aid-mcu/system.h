#pragma once
#include "global.h"
#include <WiFi.h>
#include <esp_wifi.h>
#include "driver/adc.h"
#include <esp_task_wdt.h>
#include "esp32-hal.h"

// guide 
// https://mischianti.org/esp32-practical-power-saving-manage-wifi-and-cpu-1/

class System {
  
  public:
    void set_power_low();
    void set_power_high();
    void print_cpu_info();
    void update();

    // singleton stuff + delete the functions
    static System& instance();
    System(const System&) = delete;
    System(System&&) = delete;
    System& operator=(const System&) = delete;
    System& operator=(System&&) = delete;

  private:
    bool enabled = false;

    const int INTERVAL = 500;
    unsigned long last_interval = 0;

    static System& rInstance;
    System();
    //~System();
  };