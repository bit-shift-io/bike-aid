#pragma once

class Power {
  
  public:
    void update();
    void set_enable(bool); // system
    bool get_enable(); // system
    void set_lights_enable(bool);
    bool get_lights_enable();

    // singleton stuff + delete the functions
    static Power& instance();
    Power(const Power&) = delete;
    Power(Power&&) = delete;
    Power& operator=(const Power&) = delete;
    Power& operator=(Power&&) = delete;

  private:
    const int POWER_PIN = 8;
    const int LIGHT_PIN = 9;

    bool enabled = false; // system
    bool lights_enabled = false;

    static Power& rInstance;
    Power();
    //~Power();
  };