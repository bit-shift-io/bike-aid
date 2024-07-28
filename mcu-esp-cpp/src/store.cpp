#include "store.h"
#include <nvs_flash.h>
#include "throttle.h"
#include "log.h"

StoreClass Store;


StoreClass::StoreClass() {
}


void StoreClass::init() {
  Log.println("StoreClass init");

  if (WIPE_STORE) {
    Log.println("wipe flash...");
    nvs_flash_erase(); // erase the NVS partition and...
    nvs_flash_init(); // initialize the NVS partition.
    Log.println("wipe complete, flash new rom");
    while(true);
  }

  // reStoreClass values 
  // Note: Key name is limited to 15 chars.
  // see the url for types
  // https://microcontrollerslab.com/save-data-esp32-flash-permanently-preferences-library/

  preferences.begin("bike-aid", false);
  Throttle::instance().set_increase_smoothing_factor(preferences.getInt("smoothing", 2000));
  preferences.end();
}


void StoreClass::set_value(StoreClass::type name, std::string value) {
  Log.println("StoreClass set value");

  switch(name) {
    case StoreClass::increase_smoothing_factor:
      preferences.begin("bike-aid", false);
      preferences.putUInt("smoothing", std::stoi(value)); // to int
      preferences.end();
      return;
      break;
  }


  // todo: StoreClass other values min smoothing, speed limit, deadband
  Log.println("missing store set_value ");
}