#pragma once
#include "Arduino.h"

class Logger : public Print {
  
  public:
    Logger();
    void init(int baud);

    // overwirte this method
    virtual size_t write(uint8_t c) {
        Serial.write(c);
        //Bluetooth::instance()->write(c);
        return 1;
    }
};

extern Logger Log;
