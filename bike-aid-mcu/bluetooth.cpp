#include "BLEServer.h"
#include "Arduino.h"
#include "bluetooth.h"

// class for callbacks
class BluetoothServerCallbacks: public BLEServerCallbacks {
    void onConnect(BLEServer* pServer) {
      Bluetooth::instance().on_connect(pServer);
    };

    void onDisconnect(BLEServer* pServer) {
      Bluetooth::instance().on_disconnect(pServer);
    }
};

Bluetooth::Bluetooth() {
   // create ble device
  BLEDevice::init("Bronson Scooter");

  // create the ble server
  pServer = BLEDevice::createServer();
  pServer->setCallbacks(new BluetoothServerCallbacks());

  // for services, see https://www.bluetooth.com/specifications/assigned-numbers/

  // device info service
  BLEService *pDeviceInfoService = pServer->createService(BLEUUID((uint16_t) 0x180a));
	BLECharacteristic *pPnpCharacteristic = pDeviceInfoService->createCharacteristic((uint16_t) 0x2a50, BLECharacteristic::PROPERTY_READ);
  BLECharacteristic *pManufacturerCharacteristic = pDeviceInfoService->createCharacteristic((uint16_t) 0x2a29, BLECharacteristic::PROPERTY_READ);
  pManufacturerCharacteristic->setValue("Bronson Mathews");
  pDeviceInfoService->start();


  // create unknown service
  BLEService *pService = pServer->createService(SERVICE_UUID);
  BLECharacteristic *pCharacteristic = pService->createCharacteristic(
                                         CHARACTERISTIC_UUID,
                                         BLECharacteristic::PROPERTY_READ |
                                         BLECharacteristic::PROPERTY_WRITE);

  pCharacteristic->setValue("Hello World says Neil");
  pService->start();


  // battery




  // battery
  // https://circuitdigest.com/microcontroller-projects/esp32-ble-server-how-to-use-gatt-services-for-battery-level-indication
  BLEService *pBatteryService = pServer->createService(BLEUUID((uint16_t) 0x180f));

  /*pBatteryLevelCharacteristic = pBatteryService->createCharacteristic(
                                          BLEUUID((uint16_t)0x2A19), 
                                          BLECharacteristic::PROPERTY_READ | 
                                          BLECharacteristic::PROPERTY_NOTIFY);
  //BLEDescriptor *pBatteryDescriptor = new BLEDescriptor(BLEUUID((uint16_t)0x2901));
*/

	BLE2904* batteryLevelDescriptor = new BLE2904();
	batteryLevelDescriptor->setFormat(BLE2904::FORMAT_UINT8);
	batteryLevelDescriptor->setNamespace(1);
	batteryLevelDescriptor->setUnit(0x27ad);

	pBatteryLevelCharacteristic = pBatteryService->createCharacteristic(
                                          (uint16_t) 0x2a19,
                                          BLECharacteristic::PROPERTY_READ |
                                          BLECharacteristic::PROPERTY_NOTIFY);
	pBatteryLevelCharacteristic->addDescriptor(batteryLevelDescriptor);
	pBatteryLevelCharacteristic->addDescriptor(new BLE2902());

  //pBatteryService->addCharacteristic(&pBatteryCharacteristic);
  //pBatteryLevelCharacteristic->addDescriptor(BLEUUID((uint16_t)0x2901));
  //pBatteryLevelCharacteristic->addDescriptor(new BLE2902());
  //pBatteryLevelCharacteristic->setValue("Percentage 0 - 100");
  pBatteryService->start();

  

  // pin code
  BLESecurity *pSecurity = new BLESecurity();
  pSecurity->setStaticPIN(PIN_CODE); 

  // Start advertising
  pServer->getAdvertising()->start();
  /*
  // BLEAdvertising *pAdvertising = pServer->getAdvertising();  // this still is working for backward compatibility
  *pAdvertising = BLEDevice::getAdvertising();
  pAdvertising->addServiceUUID(SERVICE_UUID);
  pAdvertising->setScanResponse(true);
  pAdvertising->setMinPreferred(0x06);  // functions that help with iPhone connections issue
  pAdvertising->setMinPreferred(0x12);
  BLEDevice::startAdvertising();
  */
  Serial.println("Characteristic defined! Now you can read it in your phone!");
}


Bluetooth& Bluetooth::instance() {
  static auto &&rInstance = Bluetooth();
  return rInstance;
}


void Bluetooth::init() {
}

void Bluetooth::setEnable(bool enable) {
  enabled = enable;
}


void Bluetooth::update() {
  if (!enabled)
    return;

  unsigned long time = millis();
  if (time - last_interval > INTERVAL) {
    last_interval = time;

    // check device is still conected
    if (device_connected) {
      uint8_t battery_level = 0;
      pBatteryLevelCharacteristic->setValue(&battery_level, 1);
      pBatteryLevelCharacteristic->notify();
    }

    // disconnecting
    if (!device_connected && old_device_connected) {
      //delay(500); // give the bluetooth stack the chance to get things ready
      pServer->startAdvertising(); // restart advertising
      old_device_connected = device_connected;
    }
    // connecting
    if (device_connected && !old_device_connected) {
		  // do stuff here on connecting
      old_device_connected = device_connected;
    }
  }
}


// callbacks
void Bluetooth::on_connect(BLEServer* pServer) {
  device_connected = true;
};


// callbacks
void Bluetooth::on_disconnect(BLEServer* pServer) {
  device_connected = false;
};