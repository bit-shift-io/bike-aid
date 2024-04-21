#pragma once
#include <string>
#include "Arduino.h"

class LogClass : public Print {
  
  public:
    LogClass();
    void init(int baud);
    size_t write(uint8_t c);

  private:
    void set_value(uint8_t c);
};

extern LogClass Log;
