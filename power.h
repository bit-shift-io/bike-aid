#pragma once

#include "Arduino.h"

class Power {

  private:
    /* 
    PINS
    ===========================
    */
    const byte SWITCH_PIN = 0; //D0;


  public:
    Power();
    void    init();
    void    update();
  };