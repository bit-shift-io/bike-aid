#pragma once
#include "Arduino.h"
#include <Preferences.h>
#include <nvs_flash.h>
#include "throttle.h"

class Store {
  
  public:
    void update();
    void set_value(String name, std::string value);

    // singleton stuff + delete the functions
    static Store& instance();
    Store(const Store&) = delete;
    Store(Store&&) = delete;
    Store& operator=(const Store&) = delete;
    Store& operator=(Store&&) = delete;

  private:
    Preferences preferences;

    const bool WIPE_STORE = false; // set this only on first setup! This will wipe the flash memory.

    static Store& rInstance;
    Store();
    //~Store();
  };