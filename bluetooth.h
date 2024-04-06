#pragma once
#include "Arduino.h"

class Bluetooth {

  public:
    void update();
    // singleton stuff + delete the functions
    static Bluetooth& instance();
    Bluetooth(const Bluetooth&) = delete;
    Bluetooth(Bluetooth&&) = delete;
    Bluetooth& operator=(const Bluetooth&) = delete;
    Bluetooth& operator=(Bluetooth&&) = delete;

  private:
    // pins
    const byte INPUT_PIN = 12;
    bool enabled = false;

    const int INTERVAL = 1;
    unsigned long last_interval = 0;

    const byte Bluetooth_CAPACITY = 24; // Ah
    const byte Bluetooth_VOLTAGE = 48; // v
    const byte Bluetooth_LOW = 46; // v

    static Bluetooth& rInstance;
    Bluetooth();
    ~Bluetooth();

  };