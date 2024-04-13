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

  // device information service
  // manufacturer
  // email
  BLEService *device_information_service = pServer->createService(BLEUUID((uint16_t) 0x180a));
  BLECharacteristic *manufacturer_characteristic = device_information_service->createCharacteristic((uint16_t) 0x2a29, BLECharacteristic::PROPERTY_READ);
  manufacturer_characteristic->setValue("Bronson Mathews");
  BLECharacteristic *email_characteristic = device_information_service->createCharacteristic((uint16_t) 0x2A87, BLECharacteristic::PROPERTY_READ);
  email_characteristic->setValue("bronsonmathews@gmail.com");

  device_information_service->start();


  // user data service  0x181C  
  // speed              0x2A67
  // trip duration      0x2BF2
  // odometer           0x2AE3
  // temperature        0x2A6E
  BLEService *user_data_service = pServer->createService(BLEUUID((uint16_t) 0x181C));

  BLECharacteristic *speed_characteristic = user_data_service->createCharacteristic(
                                         (uint16_t) 0x2A67,
                                         BLECharacteristic::PROPERTY_READ |
                                         BLECharacteristic::PROPERTY_NOTIFY);

  BLECharacteristic *trip_duration_characteristic = user_data_service->createCharacteristic(
                                         (uint16_t) 0x2BF2,
                                         BLECharacteristic::PROPERTY_READ |
                                         BLECharacteristic::PROPERTY_NOTIFY);

  BLECharacteristic *trip_distance_characteristic = user_data_service->createCharacteristic(
                                         (uint16_t) 0x2AE3,
                                         BLECharacteristic::PROPERTY_READ |
                                         BLECharacteristic::PROPERTY_NOTIFY);

  BLECharacteristic *temperature_characteristic = user_data_service->createCharacteristic(
                                         (uint16_t) 0x2A6E,
                                         BLECharacteristic::PROPERTY_READ |
                                         BLECharacteristic::PROPERTY_NOTIFY);

  // custom desc
  BLECharacteristic *pCharacteristic = user_data_service->createCharacteristic(
                                         CHARACTERISTIC_UUID,
                                         BLECharacteristic::PROPERTY_READ |
                                         BLECharacteristic::PROPERTY_WRITE);
  pCharacteristic->setValue("Hello World says Neil");

  user_data_service->start();


  // battery service    0x180f
  // level (percent)    0x2A19
  // voltage            0x2B18
  // power watt         0x2B05
  // current            0x2AEE
  // total ah           0x2B06
  BLEService *battery_service = pServer->createService(BLEUUID((uint16_t) 0x180f));

	BLE2904* batteryLevelDescriptor = new BLE2904();
	batteryLevelDescriptor->setFormat(BLE2904::FORMAT_UINT8);
	batteryLevelDescriptor->setNamespace(1);
	batteryLevelDescriptor->setUnit(0x27ad);

	battery_level_characteristic = battery_service->createCharacteristic(
                                          (uint16_t) 0x2a19,
                                          BLECharacteristic::PROPERTY_READ |
                                          BLECharacteristic::PROPERTY_NOTIFY);
	battery_level_characteristic->addDescriptor(batteryLevelDescriptor);
	battery_level_characteristic->addDescriptor(new BLE2902());

  BLECharacteristic *voltage_characteristic = battery_service->createCharacteristic(
                                         (uint16_t) 0x2B18,
                                         BLECharacteristic::PROPERTY_READ |
                                         BLECharacteristic::PROPERTY_NOTIFY);

  BLECharacteristic *power_characteristic = battery_service->createCharacteristic(
                                         (uint16_t) 0x2B05,
                                         BLECharacteristic::PROPERTY_READ |
                                         BLECharacteristic::PROPERTY_NOTIFY);

  BLECharacteristic *current_characteristic = battery_service->createCharacteristic(
                                         (uint16_t) 0x2AEE,
                                         BLECharacteristic::PROPERTY_READ |
                                         BLECharacteristic::PROPERTY_NOTIFY);

  BLECharacteristic *capacity_characteristic = battery_service->createCharacteristic(
                                         (uint16_t) 0x2B06,
                                         BLECharacteristic::PROPERTY_READ |
                                         BLECharacteristic::PROPERTY_NOTIFY);
  capacity_characteristic->setValue("21");

  battery_service->start();

  

  // pin code
  BLESecurity *pSecurity = new BLESecurity();
  pSecurity->setStaticPIN(PIN_CODE); 

  // Start advertising
  BLEAdvertising *pAdvertising = pServer->getAdvertising();  // this still is working for backward compatibility
  pAdvertising->addServiceUUID(SERVICE_UUID); // custom id
  pAdvertising->setScanResponse(true);
  pAdvertising->setMinPreferred(0x06);  // functions that help with iPhone connections issue
  pAdvertising->setMinPreferred(0x12);
  pServer->getAdvertising()->start();

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
      battery_level_characteristic->setValue(&battery_level, 1);
      battery_level_characteristic->notify();
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