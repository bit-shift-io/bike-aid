#pragma once
#include "Arduino.h"
#include <string>
#include "bluetooth.h"

class LogClass : public Print {
  
  public:
    LogClass();
    void init(int baud);
    void set_value(uint8_t c);

    // overwirte this method
    size_t write(uint8_t c) {
        Serial.write(c);
        set_value(c);
        return 1;
    }
};

extern LogClass Log;
