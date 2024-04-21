#include "log.h"

LogClass Log;

LogClass::LogClass() {
}

void LogClass::init(int baud) {
  Serial.begin(baud);
  delay(1000);
  println();println();println();
  println("log init");
}


size_t LogClass::write(uint8_t c) { // overwirte this method from print
    set_value(c);
    return 1;
}


void LogClass::set_value(uint8_t c) {
  Serial.write(c);
  //Bluetooth.set_value(Bluetooth.uart_rx_characteristic, c);
}