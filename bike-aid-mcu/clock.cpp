#include "clock.h"


Clock::Clock() {
}


Clock& Clock::instance() {
  static auto &&rInstance = Clock();
  return rInstance;
}


void Clock::set_enable(bool enable) {
  Log.print("clock enable ");Log.println(enable);
  enabled = enable;

  if (enabled) {
    start_time = millis();
    last_interval = start_time;
  }
}


void Clock::update() {
  if (!enabled)
  return;

  unsigned long time = millis();
  if (time - last_interval > INTERVAL) {
    last_interval = time;

    int all_minutes = (time - start_time) / 60000;
    int run_hours = all_minutes / 60;
    int run_minutes = all_minutes - (run_hours * 60);

    // convert to string hh:mm
    char buffer[6];
    sprintf(buffer, "%02d:%02d", run_hours, run_minutes);
    Bluetooth::instance().set_value("trip_duration", (std::string) buffer);
  }
}