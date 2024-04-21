#pragma once
#include <Preferences.h>

class StoreClass {

  public:
    enum type {
      increase_smoothing_factor
    };

    StoreClass();
    void init();
    void set_value(StoreClass::type name, std::string value);

  private:
    Preferences preferences;
    const bool WIPE_STORE = false; // set this only on first setup! This will wipe the flash memory.
};

extern StoreClass Store;