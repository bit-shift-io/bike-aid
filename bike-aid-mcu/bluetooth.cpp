#include "Arduino.h"
#include "bluetooth.h"


Bluetooth::Bluetooth() {
  Serial.println("Starting BLE work!");
  BLEDevice::init("Bronson Scooter");
  BLEServer *pServer = BLEDevice::createServer();
  BLEService *pService = pServer->createService(SERVICE_UUID);
  BLECharacteristic *pCharacteristic = pService->createCharacteristic(
                                         CHARACTERISTIC_UUID,
                                         BLECharacteristic::PROPERTY_READ |
                                         BLECharacteristic::PROPERTY_WRITE
                                       );

  pCharacteristic->setValue("Hello World says Neil");
  pService->start();
  // BLEAdvertising *pAdvertising = pServer->getAdvertising();  // this still is working for backward compatibility
  BLEAdvertising *pAdvertising = BLEDevice::getAdvertising();
  pAdvertising->addServiceUUID(SERVICE_UUID);
  pAdvertising->setScanResponse(true);
  pAdvertising->setMinPreferred(0x06);  // functions that help with iPhone connections issue
  pAdvertising->setMinPreferred(0x12);
  BLEDevice::startAdvertising();
  Serial.println("Characteristic defined! Now you can read it in your phone!");
}


Bluetooth& Bluetooth::instance() {
  static auto &&rInstance = Bluetooth();
  return rInstance;
}


void Bluetooth::setEnable(bool enable) {
  enabled = enable;
}


void Bluetooth::update() {
  if (!enabled)
    return;
}