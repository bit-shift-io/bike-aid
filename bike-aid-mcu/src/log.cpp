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


void LogClass::set_value(uint8_t c) {
  //Bluetooth.set_value("log", std::string((char *)c));
  size_t len;
  std::string s(reinterpret_cast<char const*>(c), len);
  //Bluetooth.set_value("log", s);
}
