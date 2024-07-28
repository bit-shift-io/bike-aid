#include "NimBLECharacteristic.h"
#include <string>
#include <cstdint>
#include "BLEServer.h"
#include "bluetooth.h"
#include "store.h"
#include "throttle.h"
#include "alarm.h"
#include "power.h"
#include "log.h"

// assign global 
BluetoothClass Bluetooth;

// class for callbacks
class BluetoothClassServerCallbacks: public BLEServerCallbacks {
    void onConnect(BLEServer* pServer) {
      Bluetooth.on_connect(pServer);
    };

    void onDisconnect(BLEServer* pServer) {
      Bluetooth.on_disconnect(pServer);
    }
};


// class for callbacks
class BluetoothClassCharacteristicCallbacks: public BLECharacteristicCallbacks {
  void onWrite(BLECharacteristic *pCharacteristic) {
    Bluetooth.on_write(pCharacteristic);
  }
};


BluetoothClass::BluetoothClass() {
}


void BluetoothClass::init() {
   // create ble device
  BLEDevice::init("Bronson Scooter");

  // create the ble server
  pServer = BLEDevice::createServer();
  pServer->setCallbacks(new BluetoothClassServerCallbacks());

  // for services, see https://www.BluetoothClass.com/specifications/assigned-numbers/

  // device information service
  // manufacturer
  // email
  BLEService *device_information_service = pServer->createService(BLEUUID((uint16_t) 0x180a));
  BLECharacteristic *manufacturer_characteristic = device_information_service->createCharacteristic((uint16_t) 0x2a29, NIMBLE_PROPERTY::READ);
  manufacturer_characteristic->setValue("Bronson Mathews");
  BLECharacteristic *email_characteristic = device_information_service->createCharacteristic((uint16_t) 0x2A87, NIMBLE_PROPERTY::READ);
  email_characteristic->setValue("bronsonmathews@gmail.com");

  device_information_service->start();

  // uart service
  BLEService *uart_service = pServer->createService(SERVICE_UART_UUID);
  uart_tx_characteristic = uart_service->createCharacteristic(
                                          CHARACTERISTIC_TX_UUID,
                                          NIMBLE_PROPERTY::NOTIFY);
  uart_rx_characteristic = uart_service->createCharacteristic(
                                          CHARACTERISTIC_RX_UUID,
                                          NIMBLE_PROPERTY::WRITE);
  uart_rx_characteristic->setCallbacks(new BluetoothClassCharacteristicCallbacks());
  uart_service->start();

  // user editable settings
  // settings service   0x2B1E
  // throttle smoothing
  // deadband - upper
  // deadband - lower
  // alarm toggle
  // power system toggle
  // lights toggle
  BLEService *settings_service = pServer->createService(BLEUUID((uint16_t) 0x2B1E));

  power_system_characteristic = settings_service->createCharacteristic(
                                         POWER_SYSTEM_UUID,
                                         NIMBLE_PROPERTY::READ |
                                         NIMBLE_PROPERTY::WRITE);
  power_system_characteristic->setValue(std::to_string(Power::instance().get_enable()));
  power_system_characteristic->setCallbacks(new BluetoothClassCharacteristicCallbacks());

  power_lights_characteristic = settings_service->createCharacteristic(
                                         POWER_LIGHTS_UUID,
                                         NIMBLE_PROPERTY::READ |
                                         NIMBLE_PROPERTY::WRITE);
  power_lights_characteristic->setValue(std::to_string(Power::instance().get_lights_enable()));
  power_lights_characteristic->setCallbacks(new BluetoothClassCharacteristicCallbacks());

  alarm_enabled_characteristic = settings_service->createCharacteristic(
                                         ALARM_ENABLED_UUID,
                                         NIMBLE_PROPERTY::READ |
                                         NIMBLE_PROPERTY::WRITE);
  alarm_enabled_characteristic->setValue(std::to_string(Alarm::instance().get_enable()));
  alarm_enabled_characteristic->setCallbacks(new BluetoothClassCharacteristicCallbacks());

  throttle_smoothing_characteristic = settings_service->createCharacteristic(
                                         THROTTLE_SMOOTHING_UUID,
                                         NIMBLE_PROPERTY::READ |
                                         NIMBLE_PROPERTY::WRITE);
  throttle_smoothing_characteristic->setValue(std::to_string(Throttle::instance().get_increase_smoothing_factor()));
  throttle_smoothing_characteristic->setCallbacks(new BluetoothClassCharacteristicCallbacks());

  settings_service->start();


  // user data service  0x181C  
  // speed              0x2A67
  // trip duration      0x2BF2
  // odometer           0x2AE3
  // temperature        0x2A6E
  BLEService *user_data_service = pServer->createService(BLEUUID((uint16_t) 0x181C));

  speed_characteristic = user_data_service->createCharacteristic(
                                         (uint16_t) 0x2A67,
                                         NIMBLE_PROPERTY::READ |
                                         NIMBLE_PROPERTY::NOTIFY);

  trip_duration_characteristic = user_data_service->createCharacteristic(
                                         (uint16_t) 0x2BF2,
                                         NIMBLE_PROPERTY::READ |
                                         NIMBLE_PROPERTY::NOTIFY);

  trip_distance_characteristic = user_data_service->createCharacteristic(
                                         (uint16_t) 0x2AE3,
                                         NIMBLE_PROPERTY::READ |
                                         NIMBLE_PROPERTY::NOTIFY);

  temperature_characteristic = user_data_service->createCharacteristic(
                                         (uint16_t) 0x2A6E,
                                         NIMBLE_PROPERTY::READ |
                                         NIMBLE_PROPERTY::NOTIFY);

  user_data_service->start();


  // battery service    0x180f
  // level (percent)    0x2A19
  // voltage            0x2B18
  // power watt         0x2B05
  // current            0x2AEE
  // total ah           0x2B06
  BLEService *battery_service = pServer->createService(BLEUUID((uint16_t) 0x180f));

  battery_level_characteristic = battery_service->createCharacteristic(
                                          (uint16_t) 0x2a19,
                                          NIMBLE_PROPERTY::READ |
                                          NIMBLE_PROPERTY::NOTIFY);

  BLECharacteristic *voltage_characteristic = battery_service->createCharacteristic(
                                         (uint16_t) 0x2B18,
                                         NIMBLE_PROPERTY::READ |
                                         NIMBLE_PROPERTY::NOTIFY);

  BLECharacteristic *power_characteristic = battery_service->createCharacteristic(
                                         (uint16_t) 0x2B05,
                                         NIMBLE_PROPERTY::READ |
                                         NIMBLE_PROPERTY::NOTIFY);

  BLECharacteristic *current_characteristic = battery_service->createCharacteristic(
                                         (uint16_t) 0x2AEE,
                                         NIMBLE_PROPERTY::READ |
                                         NIMBLE_PROPERTY::NOTIFY);

  BLECharacteristic *capacity_characteristic = battery_service->createCharacteristic(
                                         (uint16_t) 0x2B06,
                                         NIMBLE_PROPERTY::READ |
                                         NIMBLE_PROPERTY::NOTIFY);

  battery_service->start();

  

  // todo: pin code
  BLESecurity *pSecurity = new BLESecurity();
  pSecurity->setStaticPIN(PIN_CODE); 

  // Start advertising
  BLEAdvertising *pAdvertising = pServer->getAdvertising();  // this still is working for backward compatibility
  /* // old?
  pAdvertising->setScanResponse(true);
  pAdvertising->setMinPreferred(0x06);  // functions that help with iPhone connections issue
  pAdvertising->setMinPreferred(0x12);
  */
  pAdvertising->addServiceUUID(SERVICE_UART_UUID); // uart
  pAdvertising->addServiceUUID(SERVICE_UUID); // custom id
  pAdvertising->start();


  Log.println("BluetoothClass init");
}


void BluetoothClass::set_value(BLECharacteristic *pCharacteristic, std::string value) {
  if (!pCharacteristic)
    return;

  pCharacteristic->setValue(value);
  pCharacteristic->notify();
  Log.println("bluetooh set_value");
}


void BluetoothClass::set_value(BLECharacteristic *pCharacteristic, uint8_t value) {
  if (!pCharacteristic)
    return;
  pCharacteristic->setValue(value);
  pCharacteristic->notify();
  Log.println("bluetooh set_value");
}


// callbacks for user changables
void BluetoothClass::on_write(BLECharacteristic *pCharacteristic) {
  // we can only recieve bytes, so need to convert to string to manipulate it
  const uint8_t *data = pCharacteristic->getValue();
  size_t size = pCharacteristic->getDataLength();
  std::string value = pCharacteristic->getValue();
  //int int_val = std::stoi(value.c_str()); // debug, we send strings via ble

  if (pCharacteristic == throttle_smoothing_characteristic) {
    Store.set_value(StoreClass::increase_smoothing_factor, value);
    return;
  }

  if (pCharacteristic == throttle_smoothing_characteristic) {
    Store.set_value(StoreClass::increase_smoothing_factor, value);
    return;
  }

  if (pCharacteristic == uart_rx_characteristic) {
    // notify data recieved
    uart_tx_characteristic->notify(data, size);

    // process data
    if (value.length() > 0) {
      Log.println("*********");
      Log.print("Received Value: ");
      for (int i = 0; i < value.length(); i++)
        Log.print(value[i]);

      Log.println();
      Log.println("*********");
    }

    return;
  }


  Log.println("ops! no characteristic on write");
}


// callbacks
void BluetoothClass::on_connect(BLEServer* pServer) {
  device_connected = true;
};


// callbacks
void BluetoothClass::on_disconnect(BLEServer* pServer) {
  device_connected = false;
  delay(500); // give the BluetoothClass stack the chance to get things ready
  pServer->startAdvertising(); // restart advertising
  //old_device_connected = device_connected;
};