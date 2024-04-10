#pragma once
#include "Arduino.h"
#include <Preferences.h>
#include <nvs_flash.h>

class Store {
  // https://microcontrollerslab.com/save-data-esp32-flash-permanently-preferences-library/

  public:
    void update();
    void setEnable(bool);

    // singleton stuff + delete the functions
    static Store& instance();
    Store(const Store&) = delete;
    Store(Store&&) = delete;
    Store& operator=(const Store&) = delete;
    Store& operator=(Store&&) = delete;

  private:
    bool enabled = false;
    Preferences preferences;

    const bool WIPE_STORE = false; // set this only on first setup! This will wipe the flash memory.

    static Store& rInstance;
    Store();
    ~Store();
  };