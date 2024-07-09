#pragma once

class Alarm {

  public:
    void update();  
    void set_enable(bool);
    bool get_enable();
    static void interruptHandler();
    
    // singleton stuff + delete the functions
    static Alarm& instance();
    Alarm(const Alarm&) = delete;
    Alarm(Alarm&&) = delete;
    Alarm& operator=(const Alarm&) = delete;
    Alarm& operator=(Alarm&&) = delete;

  private:
    const int INPUT_PIN = 2; // Nano 2 + 3 are interrupts
    bool enabled = false;
    bool alarm_active = false;

    // how many readings in 1 second
    const int INTERVAL = 1000;
    unsigned long last_interval = 0;
    const int SENSITIVITY = 40;
    int trigger_count = 0;
    bool tigger_state = 0; // 0 - low, 1 - high

    // interupt
    volatile int interrupt_count = 0;
    
    // warnings
    const int WARN_INTERVAL = 10000;
    unsigned long last_warn_interval = 0;
    const int WARNINGS = 2;
    int warn_count = 0;

    // class
    static Alarm& rInstance;
    Alarm();
    //~Alarm();
};