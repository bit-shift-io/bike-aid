#include "Arduino.h"
#include "store.h"


Store::Store() {
  if (!WIPE_STORE)
    return;

  nvs_flash_erase(); // erase the NVS partition and...
  nvs_flash_init(); // initialize the NVS partition.
  while(true);
}


Store& Store::instance() {
  static auto &&rInstance = Store();
  return rInstance;
}


void Store::setEnable(bool enable) {
  enabled = enable;
}


void Store::update() {
  if (!enabled)
    return;

  // example to write to memory
  preferences.begin("bike-aid", false);

  // Get the counter value, if the key does not exist, return a default value of 0
  // Note: Key name is limited to 15 chars.
  unsigned int counter = preferences.getUInt("counter", 0);

  // Store the counter to the Preferences
  preferences.putUInt("counter", counter);

  // note we can also store some kind of structs, which may be usefull eg network credentials and password
  // note 2, if storing more than a few values, investigate SPIFFS/JSON writing to file instead

  // close
  preferences.end();
}