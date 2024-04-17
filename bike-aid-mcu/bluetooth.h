// handy guide https://github.com/nkolban/ESP32_BLE_Arduino/blob/master/examples/BLE_uart/BLE_uart.ino

#pragma once
#include "Arduino.h"
#include "NimBLEDevice.h"
#include "store.h"
#include "throttle.h"
#include "alarm.h"
#include "power.h"

class Bluetooth {

  public:
    // set data
    void set_value(String name, std::string value);

    // callbacks
    void on_connect(BLEServer* pServer);
    void on_disconnect(BLEServer* pServer);
    void on_write(BLECharacteristic *pCharacteristic);

    // singleton stuff + delete the functions
    static Bluetooth& instance();
    Bluetooth(const Bluetooth&) = delete;
    Bluetooth(Bluetooth&&) = delete;
    Bluetooth& operator=(const Bluetooth&) = delete;
    Bluetooth& operator=(Bluetooth&&) = delete;


  private:
    // https://www.uuidgenerator.net/
    const char* SERVICE_UUID = "8fabcc8a-0a6e-4c37-b640-eb5adf88b465";
    const char* THROTTLE_SMOOTHING_UUID = "beb5483e-36e1-0000-b7f5-ea07361b26a8";
    const char* ALARM_ENABLED_UUID = "beb5483e-36e1-0001-b7f5-ea07361b26a8";
    const char* POWER_SYSTEM_UUID = "beb5483e-36e1-0002-b7f5-ea07361b26a8";
    const char* POWER_LIGHTS_UUID = "beb5483e-36e1-0003-b7f5-ea07361b26a8";
    const int PIN_CODE = 123456;

    // server
    BLEServer *pServer = NULL;

    // services

    // characteristics
    BLECharacteristic *power_system_characteristic;
    BLECharacteristic *power_lights_characteristic;
    BLECharacteristic *alarm_enabled_characteristic;
    BLECharacteristic *battery_level_characteristic;
    BLECharacteristic *throttle_smoothing_characteristic;
    BLECharacteristic *temperature_characteristic;
    BLECharacteristic *trip_distance_characteristic;
    BLECharacteristic *trip_duration_characteristic;
    BLECharacteristic *speed_characteristic;

    // callbacks
    bool device_connected = false;
    bool old_device_connected = false;

    // Delay between loops in ms
    const int INTERVAL = 6000; // 1 minute
    unsigned long last_interval = 0;

    static Bluetooth& rInstance;
    Bluetooth();
    //~Bluetooth();
};