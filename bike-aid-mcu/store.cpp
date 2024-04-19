#include "store.h"


Store::Store() {
  Log.println("store init");

  if (WIPE_STORE) {
    Log.println("wipe flash...");
    nvs_flash_erase(); // erase the NVS partition and...
    nvs_flash_init(); // initialize the NVS partition.
    Log.println("wipe complete, flash new rom");
    while(true);
  }

  // restore values 
  // Note: Key name is limited to 15 chars.
  // see the url for types
  // https://microcontrollerslab.com/save-data-esp32-flash-permanently-preferences-library/

  preferences.begin("bike-aid", false);
  Throttle::instance().set_increase_smoothing_factor(preferences.getInt("smoothing", 2000));
  preferences.end();
}


Store& Store::instance() {
  static auto &&rInstance = Store();
  return rInstance;
}


void Store::set_value(String name, std::string value) {
  Log.println("store set value");
  if (name == "increase_smoothing_factor") {
    preferences.begin("bike-aid", false);
    preferences.putUInt("smoothing", std::stoi(value)); // to int
    preferences.end();
    return;
  }

  // todo: store other values min smoothing, speed limit, deadband

  Log.println("no set_value for " + name);
}