#pragma once
#include <WiFi.h>
#include <esp_wifi.h>
#include "driver/adc.h"
#include <esp_task_wdt.h>
#include "esp32-hal.h"
#include "log.h"

// guide 
// https://mischianti.org/esp32-practical-power-saving-manage-wifi-and-cpu-1/

class SystemClass {
  
  public:
    SystemClass();
    void init();
    void set_power_low();
    void set_power_high();
    void print_cpu_info();
    void update();
};

extern SystemClass System;