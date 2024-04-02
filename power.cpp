#include "Arduino.h"
#include "power.h"

Power::Power() {

}


void Power::update() {
}

void Power::init() {
  pinMode(SWITCH_PIN, OUTPUT);
}