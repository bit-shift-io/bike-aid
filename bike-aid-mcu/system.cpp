#include "esp32-hal.h"
#include "Arduino.h"
#include "system.h"


System::System() {
  print_cpu_info();
  set_power_low();
}


System& System::instance() {
  static auto &&rInstance = System();
  return rInstance;
}


void System::update() {
}


void System::print_cpu_info() {
  Serial.print("CPU Freq = ");
  Serial.print(getCpuFrequencyMhz());
  Serial.println(" MHz");

  Serial.print("XTAL Freq = ");
  Serial.print(getXtalFrequencyMhz());
  Serial.println(" MHz");

  Serial.print("APB Freq = ");
  Serial.print(getApbFrequency());
  Serial.println(" Hz");
}


void System::set_power_low() {
  // wifi power mode
  //esp_wifi_set_ps(WIFI_PS_MAX_MODEM); // lowest
  //esp_wifi_set_ps(WIFI_PS_MIN_MODEM); // low

  // disable adc and wifi
  adc_power_off();
  WiFi.disconnect(true);  // Disconnect from the network
  WiFi.mode(WIFI_OFF);    // Switch WiFi off

  // bluetooth off
  //btStop();

  // cpu freq, bluetooth needs 80mhz or above
  //  240, 160, 80    <<< For all XTAL types
  //  40, 20, 10      <<< For 40MHz XTAL
  //  26, 13          <<< For 26MHz XTAL
  //  24, 12          <<< For 24MHz XTAL
  setCpuFrequencyMhz(80);

  // disable watch dog when using less than 80mhz
  //disableCore0WDT();
}


void System::set_power_high() {
  // wifi power mode
  esp_wifi_set_ps(WIFI_PS_NONE); // default

  // enable adc and wifi
  adc_power_on(); delay(200);
  WiFi.disconnect(false);  // Reconnect the network
  WiFi.mode(WIFI_STA);    // Switch WiFi off

  // bluetooth on
  btStart();

  // cpu freq
  setCpuFrequencyMhz(240);
}
