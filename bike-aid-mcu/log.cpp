#include "log.h"

Logger Log;

Logger::Logger() {
}

void Logger::init(int baud) {
  Serial.begin(baud);
  Serial.flush();
  delay(1000);
  println();println();println();
  println("boot begin");
  println("log init");
}
