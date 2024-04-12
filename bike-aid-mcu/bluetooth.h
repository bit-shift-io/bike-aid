#pragma once
#include "Arduino.h"
#include <BLEDevice.h>
#include <BLEUtils.h>
#include <BLEServer.h>

class Bluetooth {

  public:
    void update();
    void setEnable(bool);
    
    // singleton stuff + delete the functions
    static Bluetooth& instance();
    Bluetooth(const Bluetooth&) = delete;
    Bluetooth(Bluetooth&&) = delete;
    Bluetooth& operator=(const Bluetooth&) = delete;
    Bluetooth& operator=(Bluetooth&&) = delete;

  private:
    const char* SERVICE_UUID = "4fafc201-1fb5-459e-8fcc-c5c9c331914b"; // todo: change
    const char* CHARACTERISTIC_UUID = "beb5483e-36e1-4688-b7f5-ea07361b26a8"; // todo: change

    bool enabled = false;


    static Bluetooth& rInstance;
    Bluetooth();
    //~Bluetooth();

  };