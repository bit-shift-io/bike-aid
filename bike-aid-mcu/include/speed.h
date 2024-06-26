#pragma once

class Speed {

  public:
    void update();
    void set_enable(bool);

    // singleton stuff + delete the functions
    static Speed& instance();
    Speed(const Speed&) = delete;
    Speed(Speed&&) = delete;
    Speed& operator=(const Speed&) = delete;
    Speed& operator=(Speed&&) = delete;

  private:
    // pins
    const int INPUT_PIN = 5;
    bool enabled = false;

    // update interval
    const int INTERVAL = 250;
    unsigned long last_interval = 0;

    int last_state = 0;

    unsigned long rotation_time = 0;
    unsigned long last_rotation_time = 0;
    float smooth_speed = 0; // for user
    float instant_speed = 0; // for speed limiter
    float delta_time = 0;
    int rotations = 0;

    // smoothing
    const int SMOOTH_FACTOR = 3;

    // measure wheel circumference for more accurate speed
          // todo: measure wheel circumference
    const float WHEEL_CIRCUMFERENCE = 997.46; // 12.5inch diameter -> 317.5mm diameter -> 997.46mm circumference

    static Speed& rInstance;
    Speed();
    //~Speed();
  };