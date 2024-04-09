#include "Arduino.h"
#include "store.h"


Store::Store() {

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
}