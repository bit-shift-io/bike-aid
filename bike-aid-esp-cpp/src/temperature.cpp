#include "temperature.h"
//#include "bluetooth.h"
//#include <OneWire.h>
//#include <DallasTemperature.h>
#include "log.h"

Temperature::Temperature() {
  /*
  OneWire one_wire(TEMPERATURE_PIN);
  DallasTemperature temperature_sensors(&one_wire);

  // Start the DS18B20 sensor
  temperature_sensors.begin();
  */
}


Temperature& Temperature::instance() {
  static auto &&rInstance = Temperature();
  return rInstance;
}


void Temperature::set_enable(bool enable) {
  Log.print("temperature enable ");Log.println(enable);
  enabled = enable;
}


void Temperature::update() {
  /*
  if (!enabled)
  return;

  unsigned long time = millis();
  if (time - last_interval > INTERVAL) {
    last_interval = time;

    temperature_sensors.requestTemperatures(); 
    int temp = (int)temperature_sensors.getTempCByIndex(0); // usually a float

    Bluetooth::instance().set_value("temperature", std::to_string(temp));
  }
  */
}