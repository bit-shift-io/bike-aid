#pragma once
#include <Preferences.h>
#include <nvs_flash.h>
#include "throttle.h"

class StoreClass {
  
  public:
    StoreClass();
    void init();
    void update();
    void set_value(String name, std::string value);

  private:
    Preferences preferences;
    const bool WIPE_STORE = false; // set this only on first setup! This will wipe the flash memory.
};

extern StoreClass Store;