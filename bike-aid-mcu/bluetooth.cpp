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


  // create ble service
  BLEService *pService = pServer->createService(SERVICE_UUID);

  // create a characteristic
  BLECharacteristic *pCharacteristic = pService->createCharacteristic(
                                         CHARACTERISTIC_UUID,
                                         BLECharacteristic::PROPERTY_READ |
                                         BLECharacteristic::PROPERTY_WRITE
                                       );

  pCharacteristic->setValue("Hello World says Neil");

  // start the service
  pService->start();

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
      // todo: do stuff
      //return;
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