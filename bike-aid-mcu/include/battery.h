#pragma once

class Battery {
  
  public:
    void update();
    void set_enable(bool);

    // singleton stuff + delete the functions
    static Battery& instance();
    Battery(const Battery&) = delete;
    Battery(Battery&&) = delete;
    Battery& operator=(const Battery&) = delete;
    Battery& operator=(Battery&&) = delete;

  private:
    // pins
    const int INPUT_PIN = 12;
    bool enabled = false;

    const int INTERVAL = 1;
    unsigned long last_interval = 0;

    const int BATTERY_CAPACITY = 24; // Ah
    const int BATTERY_VOLTAGE = 48; // v
    const int BATTERY_LOW = 46; // v

    static Battery& rInstance;
    Battery();
    ~Battery();

  };